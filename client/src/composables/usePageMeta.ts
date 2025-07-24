import { computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { useRemoteInfo } from "@/composables/useRemoteData";

export const usePageMeta = () => {
  const { value: info } = useRemoteInfo();

  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const webAppManifest = computed(() => ({
    name: info.value?.about?.name ?? "FanJam",
    description: info.value?.about?.description ?? undefined,
    scope: `/app/${envId.value}/`,
    start_url: `/app/${envId.value}/`,
    display: "standalone",
    icons: [
      {
        src: "/icons/icon.png",
        type: "image/png",
      },
      {
        src: "/icons/icon-maskable.png",
        type: "image/png",
        purpose: "maskable",
      },
      {
        src: "/icons/icon-monochrome.png",
        type: "image/png",
        purpose: "monochrome",
      },
      {
        src: "/icons/icon-monochrome-maskable.png",
        type: "image/png",
        purpose: "monochrome maskable",
      },
    ],
    shortcuts: [
      {
        name: "Schedule",
        url: `/app/${envId.value}/schedule`,
      },
      {
        name: "Info",
        url: `/app/${envId.value}/info`,
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
    if (info.value?.about?.name) {
      document.title = info.value.about.name;
    }

    if (info.value?.about?.description) {
      document
        .querySelector('meta[name="description"]')
        ?.setAttribute("content", info.value.about.description);
    }
  });
};
