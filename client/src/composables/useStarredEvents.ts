import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";

const starredEvents = ref<Set<string>>(new Set());
let currentEventId: string | undefined = undefined;

const useStarredEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storageKey = computed(() => `starred:${envId.value}`);

  if (currentEventId === undefined || currentEventId !== envId.value) {
    const storedValue = localStorage.getItem(storageKey.value);

    if (storedValue) {
      try {
        const parsed = JSON.parse(storedValue);
        if (Array.isArray(parsed)) {
          starredEvents.value = new Set(parsed);
        }
      } catch {
        starredEvents.value = new Set();
      }
    }

    watchEffect(() => {
      localStorage.setItem(storageKey.value, JSON.stringify(Array.from(starredEvents.value)));
    });

    currentEventId = envId.value;
  }

  return starredEvents;
};

export default useStarredEvents;
