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
        globPatterns: ["**/*.{js,css}"],
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
