import { APP_SHELL_CACHE_NAME, appShellCacheKey } from "./appShell";

// Must match the cache name used in the service worker.
const FILES_CACHE_NAME = "files-cache";

// A `fetch()` made before the service worker controls this page bypasses it
// entirely, so nothing it returns reaches the worker's caches. On a first-ever
// visit the page starts out uncontrolled and only becomes controlled once the
// worker activates and claims it, which can easily land after this code runs.
const serviceWorkerControlling = async (timeoutMs = 5000): Promise<boolean> => {
  if (!("serviceWorker" in navigator)) return false;
  if (navigator.serviceWorker.controller !== null) return true;

  // If nothing is registered, nothing is ever going to claim this page, and
  // waiting for `controllerchange` would just burn the whole timeout. The Vite
  // dev server never registers a worker, so on `localhost`--a secure context,
  // where `navigator.serviceWorker` exists--this is the *only* outcome, and
  // every call would otherwise stall for `timeoutMs`.
  if ((await navigator.serviceWorker.getRegistration()) === undefined) return false;

  return await new Promise<boolean>((resolve) => {
    const finish = (controlling: boolean) => {
      clearTimeout(timer);
      navigator.serviceWorker.removeEventListener("controllerchange", onControllerChange);
      resolve(controlling);
    };

    const onControllerChange = () => {
      finish(true);
    };

    const timer = setTimeout(() => {
      finish(false);
    }, timeoutMs);

    navigator.serviceWorker.addEventListener("controllerchange", onControllerChange);
  });
};

// Cache the app shell for this mount point. The service worker's can't cache
// it on first visit; the navigation that installed the worker happened before
// the worker existed, and nothing fetches a document again until the user
// comes back, by which time they may already be offline, with no shell to boot
// from.
export const cacheAppShell = async (): Promise<void> => {
  // The Cache API needs a secure context, so it is absent on the plain-HTTP
  // dev server.
  if (!("caches" in window)) return;

  const key = appShellCacheKey(window.location.href);

  try {
    const cache = await caches.open(APP_SHELL_CACHE_NAME);

    // Every subsequent visit goes through the service worker, which keeps this
    // entry current on its own.
    if ((await cache.match(key)) !== undefined) return;

    const response = await fetch(key);
    if (response.ok) await cache.put(key, response);
  } catch {
    // Worst case, the shell isn't there yet and the next visit tries again.
  }
};

// We fetch and allow the service worker to cache the file rather than caching
// it here ourselves. This ensures that we're respecting the service worker's
// caching strategy, including any expiration policies.
export const prefetchFiles = async (urls: ReadonlyArray<string>): Promise<void> => {
  // The Cache API needs a secure context, so it is absent on the plain-HTTP dev server.
  if (!("caches" in window)) return;

  // Without the service worker in the loop these fetches would cache nothing.
  if (!(await serviceWorkerControlling())) return;

  for (const url of urls) {
    // This could take a while, so it makes sense to check periodically.
    if (!navigator.onLine) return;

    try {
      const cached = await caches.match(url, { cacheName: FILES_CACHE_NAME });
      if (cached !== undefined) continue;

      // Read the body to completion so the service worker's copy finishes
      // streaming into the cache, then drop it.
      const response = await fetch(url);
      if (response.ok) await response.blob();
    } catch {
      // Worst-case, the file isn't available offline and gets fetched on
      // demand later.
    }
  }
};
