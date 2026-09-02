import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import { VitePWA } from "vite-plugin-pwa";

import { cloudflare } from "@cloudflare/vite-plugin";

export default defineConfig(({ mode }) => ({
  // Baked into the client bundle so the app knows when to invalidate caches
  // due to a new update, which may have changed the shape of the data.
  define: {
    __BUILD_ID: JSON.stringify(Date.now().toString(36)),
  },
  plugins: [
    vue(),
    // The Vue dev tools can interfere with Playwright tests by intercepting
    // clicks that happen near it.
    ...[mode === "playwright" ? [] : [vueDevTools()]],
    cloudflare(),
    tailwindcss(),
    // This plugin installs a service worker to allow this app to work offline
    // and to receive Web Push notifications. We use `injectManifest` rather
    // than `generateSW` because we need a custom `push` event handler, which
    // Workbox's auto-generated worker doesn't provide.
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
      strategies: "injectManifest",
      srcDir: "src",
      filename: "sw.ts",
      // We generate the manifest dynamically at the edge, so don't let the
      // plugin do it.
      manifest: false,
      // Normally, this plugin automatically includes icons linked in the
      // manifest. However, because we're generating the manifest ourselves, we
      // need to tell it where to find them.
      includeAssets: ["icons/*"],
      injectManifest: {
        // Exclude `**/*.html`, which is an implicit default.
        //
        // Fonts have to be listed explicitly. Otherwise the service worker will
        // not cache icons. Only `woff2`: the font packages ship `woff`
        // fallbacks alongside it, and Vite emits those because the stylesheets
        // reference them, but no browser new enough to run a service worker
        // will ever ask for one. Precaching them would just double the install
        // payload.
        globPatterns: ["**/*.{js,css,woff2}"],
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
    allowedHosts: ["localhost"],
  },
}));
