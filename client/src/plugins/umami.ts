import type { Router } from "vue-router";

declare global {
  interface Window {
    umami: {
      track: () => void;
    };
  }
}

type UmamiPluginOptions = {
  websiteID: string;
  scriptSrc: string;
  router: Router;
  hostUrl?: string;
};

export const VueUmamiPlugin = (options: UmamiPluginOptions): { install: () => void } => ({
  install: () => {
    if (window.location.hostname.includes("localhost")) {
      console.warn("Umami plugin not installed due to being on localhost.");
      return;
    }

    const { scriptSrc, websiteID, router, hostUrl }: UmamiPluginOptions = options;

    if (!websiteID) {
      console.warn("Website ID not provided for Umami plugin, skipping.");
      return;
    }

    attachUmamiToRouter(router);

    onDocumentReady(() => {
      initUmamiScript(scriptSrc, websiteID, hostUrl);
    });
  },
});

const attachUmamiToRouter = (router: Router): void => {
  router.afterEach((): void => {
    window.umami.track();
  });
};

const onDocumentReady = (callback: () => void): void => {
  if (document.readyState !== "loading") {
    callback();
  } else {
    document.addEventListener("DOMContentLoaded", callback);
  }
};

const initUmamiScript = (scriptSrc: string, websiteID: string, hostUrl?: string): void => {
  const script: HTMLScriptElement = document.createElement("script");

  script.defer = true;
  script.src = scriptSrc;

  script.onload = (): void => {
    console.log("Umami plugin loaded");
    window.umami.track();
  };

  script.setAttribute("data-website-id", websiteID);
  script.setAttribute("data-auto-track", "false");

  if (hostUrl) {
    script.setAttribute("data-host-url", hostUrl);
  }

  document.head.appendChild(script);
};
