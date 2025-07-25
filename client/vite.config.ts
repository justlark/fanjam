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
    // We generate the manifest dynamically in the app.
    VitePWA({ manifest: false }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
