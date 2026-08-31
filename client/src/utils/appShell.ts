// The HTML document is generated per-request by the edge worker, so it is
// never a build asset the service worker can precache. It has to be cached at
// runtime instead.
export const APP_SHELL_CACHE_NAME = "origin-cache";

// The landing page at `/` and an app at `/app/:envId/` are different documents
// served from the same origin, so the cache key must reflect this.
export const appShellCacheKey = (rawUrl: string): string => {
  const url = new URL(rawUrl);
  const mount = /^\/app\/[^/]+/.exec(url.pathname);
  return `${url.origin}${mount ? mount[0] : ""}/`;
};

// Previously, we were caching under the bare origin. We have some logic in the
// service worker to migrate clients still using this cache key.
export const legacyAppShellCacheKey = (rawUrl: string): string => new URL(rawUrl).origin;
