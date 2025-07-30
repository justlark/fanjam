import { watchEffect } from "vue";
import useRemoteData from "@/composables/useRemoteData";

export const usePageMeta = () => {
  const {
    data: { info },
  } = useRemoteData();

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
