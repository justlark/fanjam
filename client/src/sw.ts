/// <reference lib="webworker" />

import { CacheableResponsePlugin } from "workbox-cacheable-response";
import { ExpirationPlugin } from "workbox-expiration";
import { cleanupOutdatedCaches, precacheAndRoute } from "workbox-precaching";
import { googleFontsCache } from "workbox-recipes";
import { registerRoute } from "workbox-routing";
import { NetworkFirst, StaleWhileRevalidate } from "workbox-strategies";

declare const self: ServiceWorkerGlobalScope & {
  __WB_MANIFEST: Array<{ url: string; revision: string | null }>;
};

precacheAndRoute(self.__WB_MANIFEST);
cleanupOutdatedCaches();

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
      new ExpirationPlugin({ maxEntries: 10 }),
      new CacheableResponsePlugin({ statuses: [0, 200] }),
    ],
  }),
);

// Because this is a SPA, we cache the `index.html` by origin rather than by
// full URL.
//
// Because the `index.html` is generated dynamically by and edge function, we
// use `NetworkFirst`.
registerRoute(
  ({ request, url }) => request.destination === "document" && url.origin === self.location.origin,
  new NetworkFirst({
    cacheName: "origin-cache",
    plugins: [
      {
        cacheKeyWillBeUsed: ({ request }) => Promise.resolve(new URL(request.url).origin),
        cacheWillUpdate: ({ response }) =>
          Promise.resolve(response.status === 200 ? response : null),
      },
    ],
  }),
);

// This is a workbox-provided recipe for caching Google fonts.
googleFontsCache();

// We need to cache assets from CDNs for offline use.
registerRoute(
  ({ url }) => url.origin === "https://cdn.jsdelivr.net",
  new StaleWhileRevalidate({
    cacheName: "jsdelivr-cache",
    plugins: [
      new ExpirationPlugin({ maxEntries: 10 }),
      new CacheableResponsePlugin({ statuses: [0, 200] }),
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
