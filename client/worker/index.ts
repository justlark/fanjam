import { type Fetcher, type Request } from "@cloudflare/workers-types";

interface Env {
  ASSETS: Fetcher;
}

const headerPatterns = {
  "/": {
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
    // We need to include all of these origins in the `connect-src` so the
    // service worker can cache them and the app can work offline.
    "Content-Security-Policy":
      "default-src 'self'; connect-src 'self' https://api.fanjam.live https://api-test.fanjam.live https://fonts.googleapis.com https://fonts.gstatic.com https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdn.jsdelivr.net; font-src 'self' https://fonts.gstatic.com https://cdn.jsdelivr.net; manifest-src 'self' data:; frame-ancestors 'none';",
    "Referrer-Policy": "strict-origin",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains; preload",
  },
  "/app/": {
    "X-Robots-Tag": "noindex",
  },
};

export default {
  async fetch(request: Request, env: Env) {
    const requestUrl = new URL(request.url);

    const response = await env.ASSETS.fetch(request);

    // Copy response headers from origin.
    const newHeaders = new Headers();
    for (const [key, value] of response.headers.entries()) {
      newHeaders.set(key, value);
    }

    const newResponse = new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers: newHeaders,
    });

    // Add response headers.
    for (const [prefix, patternHeaders] of Object.entries(headerPatterns)) {
      if (requestUrl.pathname.startsWith(prefix)) {
        for (const [key, value] of Object.entries(patternHeaders)) {
          newResponse.headers.set(key, value);
        }
      }
    }

    return newResponse;
  },
};
