import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
const instanceStorageKey = "starred:key";
const valueStorageKey = "starred:value";

const starredEvents = ref<Array<string>>();

const useStarredEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  watchEffect(() => {
    const storedInstance = localStorage.getItem(instanceStorageKey);

    if (storedInstance !== envId.value) {
      localStorage.removeItem(instanceStorageKey);
      localStorage.removeItem(valueStorageKey);
    }
  });

  watchEffect(() => {
    if (starredEvents.value === undefined) {
      const storedValue = localStorage.getItem(valueStorageKey);
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

    localStorage.setItem(instanceStorageKey, envId.value);
    localStorage.setItem(valueStorageKey, JSON.stringify(starredEvents.value));
  });

  return starredEvents;
};

export default useStarredEvents;
