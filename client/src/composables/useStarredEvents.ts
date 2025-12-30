import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { debounce } from "@/utils/debounce";

const starredEvents = ref<Set<string>>(new Set());
const currentEnvId = ref<string>();

const debouncedSetItem = debounce((storageKey: string, starredEvents: Set<string>) => {
  localStorage.setItem(storageKey, JSON.stringify(Array.from(starredEvents)));
}, 100);

const useStarredEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storageKey = computed(() => `starred:${envId.value}`);

  if (currentEnvId.value === undefined || currentEnvId.value !== envId.value) {
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

    currentEnvId.value = envId.value;
  }

  watchEffect(() => {
    debouncedSetItem(storageKey.value, starredEvents.value);
  });

  return starredEvents;
};

export default useStarredEvents;
