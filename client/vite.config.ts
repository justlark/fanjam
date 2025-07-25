import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import { VitePWA } from "vite-plugin-pwa";

import { cloudflare } from "@cloudflare/vite-plugin";

export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    cloudflare(),
    tailwindcss(),
    // This plugin installs a service worker to allow this app to work offline.
    VitePWA({
      // We generate the manifest dynamically in the app, so don't let the
      // plugin do it.
      manifest: false,
      // Normally, this plugin automatically includes icons linked in the
      // manifest. However, because we're generating the manifest ourselves, we
      // need to tell it where to find them.
      includeAssets: ["icons/*"],
      workbox: {
        runtimeCaching: [
          // This is necessary to ensure that the service worker doesn't
          // clobber response headers from the origin (such as the CSP).
          {
            urlPattern: /^https:\/\/fanjam\.live\/.*/i,
            handler: "StaleWhileRevalidate",
            options: {
              cacheName: "origin-cache",
              cacheableResponse: {
                statuses: [0, 200],
              },
              plugins: [
                {
                  // Preserve original request
                  cacheKeyWillBeUsed: ({ request }) => Promise.resolve(request.url),
                  // Return the response as-is to preserve headers.
                  cacheWillUpdate: ({ response }) => Promise.resolve(response.status === 200 ? response : null),
                }
              ]
            },
          },
          // Cache assets fetched from CDNs.
          {
            urlPattern: /^https:\/\/fonts\.googleapis\.com\/.*/i,
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
            urlPattern: /^https:\/\/fonts\.gstatic\.com\/.*/i,
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
            urlPattern: /^https:\/\/cdn\.jsdelivr\.net\/.*/i,
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
});
