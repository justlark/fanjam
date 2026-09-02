import { fileURLToPath, URL } from "node:url";
import { execSync } from "node:child_process";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";
import tailwindcss from "@tailwindcss/vite";
import { VitePWA } from "vite-plugin-pwa";

import { cloudflare } from "@cloudflare/vite-plugin";

const git = (command: string): string =>
  execSync(command, { stdio: ["ignore", "pipe", "ignore"] })
    .toString()
    .trim();

// We use this so a running client can tell whether there's a newer version of
// the bundle to fetch.
const buildVersion = ((): string | undefined => {
  try {
    const rev = git("git rev-parse HEAD");

    // If the working tree is dirty, append a timestamp so the client treats it
    // as a different build.
    return git("git status --porcelain").length > 0 ? `${rev}.${Date.now().toString(36)}` : rev;
  } catch {
    // If we're not in a git repo, we simply don't emit a build version and the
    // client doesn't attempt to update itself until the user reloads the page.
    // This should never happen.
    return undefined;
  }
})();

export default defineConfig(({ mode }) => ({
  define: {
    __BUILD_VERSION: JSON.stringify(buildVersion),
  },
  plugins: [
    {
      name: "fanjam:emit-version",
      apply: "build",
      applyToEnvironment: (environment) => environment.name === "client",
      generateBundle() {
        this.emitFile({
          type: "asset",
          // Use `fileName` rather than `name` so it doesn't get a
          // cache-busting filename.
          fileName: "version.json",
          source: JSON.stringify({ version: buildVersion }),
        });
      },
    },
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
        //
        // Do not add `**/*.json` here; it breaks `/version.json`.
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
