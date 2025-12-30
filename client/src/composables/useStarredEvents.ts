import { ref, computed, watchEffect, type WatchHandle } from "vue";
import { useRoute } from "vue-router";
import { debounce } from "@/utils/debounce";

const starredEvents = ref<Set<string>>(new Set());
let currentEventId: string | undefined = undefined;
let watchHandle: WatchHandle | undefined = undefined;

const debouncedSetItem = debounce((storageKey: string, events: Set<string>) => {
  localStorage.setItem(storageKey, JSON.stringify(Array.from(events)));
}, 250);

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

    if (watchHandle) {
      watchHandle.stop();
    }

    watchHandle = watchEffect(() => {
      debouncedSetItem(storageKey.value, starredEvents.value);
    });

    currentEventId = envId.value;
  }

  return starredEvents;
};

export default useStarredEvents;
