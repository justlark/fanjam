import { computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";

export const usePageMeta = () => {
  const {
    data: { info },
  } = useRemoteData();

  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const webAppManifest = computed(() => ({
    name: info.value?.name ?? "FanJam",
    description: info.value?.description ?? undefined,
    scope: `${window.location.origin}/app/${envId.value}/`,
    start_url: `${window.location.origin}/app/${envId.value}/`,
    display: "standalone",
    icons: [
      {
        src: `${window.location.origin}/icons/icon.png`,
        type: "image/png",
      },
      {
        src: `${window.location.origin}/icons/icon-maskable.png`,
        type: "image/png",
        purpose: "maskable",
      },
      {
        src: `${window.location.origin}/icons/icon-monochrome.png`,
        type: "image/png",
        purpose: "monochrome",
      },
      {
        src: `${window.location.origin}/icons/icon-monochrome-maskable.png`,
        type: "image/png",
        purpose: "monochrome maskable",
      },
    ],
    shortcuts: [
      {
        name: "Schedule",
        url: `${window.location.origin}/app/${envId.value}/schedule`,
      },
      {
        name: "Info",
        url: `${window.location.origin}/app/${envId.value}/info`,
      },
    ],
  }));

  // This is a trick for dynamically generating an app manifest.
  watchEffect(() => {
    const content = encodeURIComponent(JSON.stringify(webAppManifest.value));
    const url = `data:application/manifest+json,${content}`;

    let element = document.querySelector('link[rel="manifest"]');

    if (!element) {
      element = document.createElement("link");
      element.setAttribute("rel", "manifest");
      document.querySelector("head")?.appendChild(element);
    }

    element.setAttribute("href", url);
  });

  // Dynamically set the page title and description.
  watchEffect(() => {
    if (info.value?.name) {
      document.title = info.value.name;
    }

    if (info.value?.description) {
      document
        .querySelector('meta[name="description"]')
        ?.setAttribute("content", info.value.description);
    }
  });
};

export default usePageMeta;
