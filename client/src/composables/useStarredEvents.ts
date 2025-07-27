import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";

const starredEvents = ref<Array<string>>();

const useStarredEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storageKey = computed(() => `starred:${envId.value}`);

  watchEffect(() => {
    if (starredEvents.value === undefined) {
      const storedValue = localStorage.getItem(storageKey.value);
      if (storedValue) {
        try {
          starredEvents.value = JSON.parse(storedValue);
        } catch {
          starredEvents.value = [];
        }
      } else {
        starredEvents.value = [];
      }

      return;
    }

    localStorage.setItem(storageKey.value, JSON.stringify(starredEvents.value));
  });

  return starredEvents;
};

export default useStarredEvents;
