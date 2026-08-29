// Must match the cache name used in the service worker.
const FILES_CACHE_NAME = "files-cache";

// We fetch and allow the service worker to cache the file rather than caching
// it here ourselves. This ensures that we're respecting the service worker's
// caching strategy, including any expiration policies.
export const prefetchFiles = async (urls: ReadonlyArray<string>): Promise<void> => {
  // The Cache API needs a secure context, so it is absent on the plain-HTTP dev server.
  if (!("caches" in window)) return;

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
