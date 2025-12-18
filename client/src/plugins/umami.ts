import type { Router } from "vue-router";

type UmamiTrackProps = {
  url: string;
};

declare global {
  interface Window {
    umami: {
      track: (func?: (props: UmamiTrackProps) => UmamiTrackProps) => void;
    };
  }
}

type UmamiPluginOptions = {
  scriptSrc: string;
  router: Router;
};

export const VueUmamiPlugin = (options: UmamiPluginOptions): { install: () => void } => ({
  install: () => {
    if (window.location.hostname.includes("localhost")) {
      console.warn("Umami plugin not installed due to being on localhost.");
      return;
    }

    const { scriptSrc, router }: UmamiPluginOptions = options;

    attachUmamiToRouter(router);

    onDocumentReady(() => {
      trackOnScriptLoad(scriptSrc);
    });
  },
});

const attachUmamiToRouter = (router: Router): void => {
  router.afterEach((to): void => {
    window.umami.track((props) => ({ ...props, url: to.fullPath }));
  });
};

const onDocumentReady = (callback: () => void): void => {
  if (document.readyState !== "loading") {
    callback();
  } else {
    document.addEventListener("DOMContentLoaded", callback);
  }
};

const trackOnScriptLoad = (scriptSrc: string): void => {
  const script = document.querySelector(`head > script[src='${scriptSrc}']`) as HTMLScriptElement;

  script.onload = (): void => {
    console.log("Umami plugin loaded");
    window.umami.track();
  };
};
