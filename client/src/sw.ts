/// <reference lib="webworker" />

import { CacheableResponsePlugin } from "workbox-cacheable-response";
import { ExpirationPlugin } from "workbox-expiration";
import { cleanupOutdatedCaches, precacheAndRoute } from "workbox-precaching";
import { registerRoute } from "workbox-routing";
import { NetworkFirst, StaleWhileRevalidate } from "workbox-strategies";
import { clientsClaim } from "workbox-core";
import { APP_SHELL_CACHE_NAME, appShellCacheKey, legacyAppShellCacheKey } from "./utils/appShell";

declare const self: ServiceWorkerGlobalScope & {
  __WB_MANIFEST: Array<{ url: string; revision: string | null }>;
};

// Bootstrap Icons' stylesheet appends a cache-busting query param to its
// `@font-face` source, and Vite carries that query through into the built CSS.
// Workbox matches precache entries on the full URL including the search
// string, so without this the request for
// `bootstrap-icons-<hash>.woff2?<param>` would miss the entry for
// `bootstrap-icons-<hash>.woff2` and every icon would be tofu when running
// offline. Everything we precache is either fingerprinted or an app icon, so
// no precached response depends on a query parameter.
precacheAndRoute(self.__WB_MANIFEST, { ignoreURLParametersMatching: [/.*/] });
cleanupOutdatedCaches();

// Take over the page that installed us, rather than waiting for the next
// navigation. On a first-ever visit the page starts out uncontrolled, so
// without this every runtime fetch it makes--attachments and the like--
// bypasses the worker and lands in no cache at all. That first visit is
// the one the offline guarantee rests on, so it is the one that has to fill
// the caches.
clientsClaim();

// Fira Sans and the Bootstrap icons used to be fetched from Google and
// jsDelivr at runtime and cached here. They are build assets now, so nothing
// will ever read these again. Because the app asks the browser for persistent
// storage, leaving them behind spends storage quota the offline cache needs.
// `cleanupOutdatedCaches()` doesn't cover them; it only knows about outdated
// precaches.
const ORPHANED_CACHES = ["google-fonts-stylesheets", "google-fonts-webfonts", "jsdelivr-cache"];

self.addEventListener("activate", (event) => {
  event.waitUntil(
    Promise.all(ORPHANED_CACHES.map((name) => caches.delete(name))).then(() => undefined),
  );
});

// When the user accepts the page refresh prompt in the app, allow the new
// service worker to take over immediately.
self.addEventListener("message", (event) => {
  if ((event.data as { type?: string } | undefined)?.type === "SKIP_WAITING") {
    void self.skipWaiting();
  }
});

// File uploads are served under a different path depending on whether the
// environment is using a custom domain.
registerRoute(
  ({ url }) =>
    url.origin === self.location.origin &&
    url.pathname.match(/^\/(?:app\/[^/]+\/)?files\/([^/]+)\/?$/) !== null,
  new StaleWhileRevalidate({
    cacheName: "files-cache",
    plugins: [
      new ExpirationPlugin({ maxEntries: 20, maxAgeSeconds: 30 * 24 * 60 * 60 }),
      new CacheableResponsePlugin({ statuses: [0, 200] }),
    ],
  }),
);

// The web manifest is generated per-request by the edge worker rather than
// being a build asset, so we need to cache it explicitly.
registerRoute(
  ({ url }) =>
    url.origin === self.location.origin &&
    /^\/(?:app\/[^/]+\/)?app\.webmanifest\/?$/.test(url.pathname),
  new StaleWhileRevalidate({
    cacheName: "manifest-cache",
    plugins: [new CacheableResponsePlugin({ statuses: [0, 200] })],
  }),
);

// The HTML document is generated dynamically by an edge function, so prefer
// the network.
registerRoute(
  ({ request, url }) => request.destination === "document" && url.origin === self.location.origin,
  new NetworkFirst({
    cacheName: APP_SHELL_CACHE_NAME,
    networkTimeoutSeconds: 2,
    plugins: [
      {
        cacheKeyWillBeUsed: ({ request }) => Promise.resolve(appShellCacheKey(request.url)),
        cacheWillUpdate: ({ response }) =>
          Promise.resolve(response.status === 200 ? response : null),

        // This is necessary because we changed the format of the cache key at
        // one point, and we need to migrate existing clients to use the new
        // key.
        //
        // This has to hook every cache read, not just `handlerDidError`.
        // `NetworkFirst` also reads the cache when the network times out, and
        // on a spotty connection the request hangs rather than failing. If we
        // only migrated on error, a timeout that missed the new key would
        // leave the page waiting on the network indefinitely.
        cachedResponseWillBeUsed: async ({ request, cachedResponse }) => {
          if (cachedResponse !== undefined) return cachedResponse;

          const cache = await caches.open(APP_SHELL_CACHE_NAME);
          const shell = await cache.match(legacyAppShellCacheKey(request.url));
          if (shell === undefined) return undefined;

          await cache.put(appShellCacheKey(request.url), shell.clone());

          return shell;
        },
      },
      {
        // `NetworkFirst` only falls back to the cache when the request fails
        // outright, not when it comes back with an error status. The edge
        // worker answers with a 503 when it can't resolve a custom domain, and
        // the cached app shell is a much better answer than that. So we treat
        // a 5xx as a network failure. If there's no cached shell to fall back
        // on, we hand back the original response rather than a generic
        // network error.
        fetchDidSucceed: ({ response, state }) => {
          if (response.status >= 500) {
            if (state) state.serverError = response;
            throw new Error(`Server responded with ${String(response.status)}`);
          }

          return Promise.resolve(response);
        },
        handlerDidError: ({ state }) => Promise.resolve(state?.serverError as Response | undefined),
      },
    ],
  }),
);

// This is the shape of the JSON payload for push notifications. `url` is a
// relative path so it works whether or not the environment is being served
// from a custom domain.
interface PushPayload {
  title: string;
  body: string;
  url: string;
  icon: string | null;
}

self.addEventListener("push", (event) => {
  if (!event.data) return;

  let payload: PushPayload;
  try {
    payload = event.data.json() as PushPayload;
  } catch {
    return;
  }

  event.waitUntil(
    (async () => {
      await self.registration.showNotification(payload.title, {
        body: payload.body,
        icon: payload.icon ?? "/icons/icon-padded.png",
        data: { url: payload.url },
      });

      // Nudge any open clients to refetch the announcement list.
      const clients = await self.clients.matchAll({
        type: "window",
        includeUncontrolled: true,
      });
      for (const client of clients) {
        client.postMessage({ type: "announcement" });
      }
    })(),
  );
});

self.addEventListener("notificationclick", (event) => {
  event.notification.close();
  const targetPath = (event.notification.data as { url?: string } | undefined)?.url ?? "/";
  const targetUrl = new URL(targetPath, self.location.origin).toString();

  event.waitUntil(
    (async () => {
      const allClients = await self.clients.matchAll({
        type: "window",
        includeUncontrolled: true,
      });
      const existing = allClients.find((c) => new URL(c.url).origin === self.location.origin);
      if (existing) {
        await existing.focus();
        await existing.navigate(targetUrl).catch(() => undefined);
        return;
      }
      await self.clients.openWindow(targetUrl);
    })(),
  );
});
