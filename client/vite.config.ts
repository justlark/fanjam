import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import { VitePWA } from "vite-plugin-pwa";

import { cloudflare } from "@cloudflare/vite-plugin";

export default defineConfig(({ mode }) => ({
  plugins: [
    vue(),
    vueDevTools(),
    cloudflare(),
    tailwindcss(),
    // This plugin installs a service worker to allow this app to work offline.
    VitePWA({
      // This is confusing; let me explain.
      //
      // By default, the service worker is only installed in `prod`
      // deployments. We also want to enable it in `test` deployments, for
      // testing. This option configures which Vite mode the service worker
      // should be enabled in, and sets it to the current mode (i.e. always
      // enabled). However, for some reason, it's typed to only accept the
      // default modes (`"development" | "production"`).
      //
      // We need to lie to the type system here, but this is not the same thing
      // as actually passing the string "production".
      mode: mode as "production",
      // We generate the manifest dynamically at the edge, so don't let the
      // plugin do it.
      manifest: false,
      // Normally, this plugin automatically includes icons linked in the
      // manifest. However, because we're generating the manifest ourselves, we
      // need to tell it where to find them.
      includeAssets: ["icons/*"],
      // This is all quite complicated and I don't fully grok it, but this
      // configuration accomplishes a few things:
      //
      // - It ensures the service worker fetches the `index.html` from the edge
      // function instead of bundling the default/static one, which it would
      // otherwise prefer.
      // - It ensures headers from upstream aren't clobbered by the service
      // worker.
      // - It caches resources fetched from CDNs, which aren't included in the
      // bundle.
      // - The app can work completely offline and transition seamlessly.
      workbox: {
        // Exclude `**/*.html`, which is an implicit default.
        globPatterns: ["**/*.{js,css}"],
        navigateFallback: null,
        runtimeCaching: [
          {
            urlPattern: ({ request, url }) =>
              request.destination === "document" && url.origin === "https://fanjam.live",
            handler: "NetworkFirst",
            options: {
              cacheName: "origin-cache",
              plugins: [
                {
                  cacheKeyWillBeUsed: ({ request }) => Promise.resolve(new URL(request.url).origin),
                  cacheWillUpdate: ({ response }) => {
                    return Promise.resolve(response.status === 200 ? response : null);
                  },
                },
              ],
            },
          },
          {
            urlPattern: ({ request, url }) =>
              request.destination === "document" && url.origin === "https://test.fanjam.live",
            handler: "NetworkFirst",
            options: {
              cacheName: "origin-test-cache",
              plugins: [
                {
                  cacheKeyWillBeUsed: ({ request }) => Promise.resolve(new URL(request.url).origin),
                  cacheWillUpdate: ({ response }) => {
                    return Promise.resolve(response.status === 200 ? response : null);
                  },
                },
              ],
            },
          },
          {
            urlPattern: ({ url }) => url.origin === "https://fonts.googleapis.com",
            handler: "StaleWhileRevalidate",
            options: {
              cacheName: "google-fonts-cache",
              expiration: {
                maxEntries: 10,
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
          {
            urlPattern: ({ url }) => url.origin === "https://fonts.gstatic.com",
            handler: "StaleWhileRevalidate",
            options: {
              cacheName: "gstatic-fonts-cache",
              expiration: {
                maxEntries: 10,
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
          {
            urlPattern: ({ url }) => url.origin === "https://cdn.jsdelivr.net",
            handler: "StaleWhileRevalidate",
            options: {
              cacheName: "jsdelivr-cache",
              expiration: {
                maxEntries: 10,
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
        ],
      },
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    // Necessary for Playwright testing.
    host: "0.0.0.0",
    allowedHosts: ["localhost", "hostmachine"],
  },
}));
