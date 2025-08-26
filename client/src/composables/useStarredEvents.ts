import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";

const starredEvents = ref<Array<string>>();
const starredEventsSet = ref<Set<string>>(new Set());

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
          starredEventsSet.value = new Set(starredEvents.value);
        } catch {
          starredEvents.value = [];
          starredEventsSet.value.clear();
        }
      } else {
        starredEvents.value = [];
        starredEventsSet.value.clear();
      }

      return;
    }

    localStorage.setItem(storageKey.value, JSON.stringify(starredEvents.value));
  });

  watchEffect(() => {
    starredEvents.value = Array.from(starredEventsSet.value);
  });

  return starredEventsSet;
};

export default useStarredEvents;
