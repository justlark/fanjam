import { ref, watch, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { debounce } from "@/utils/debounce";
import { decodeBase64url } from "@/utils/encoding";

const starredEvents = ref<Set<string>>(new Set());
const currentEnvId = ref<string>();

const debouncedSetItem = debounce((storageKey: string, starredEvents: Set<string>) => {
  localStorage.setItem(storageKey, JSON.stringify(Array.from(starredEvents)));
}, 100);

const useStarredEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const storageKey = computed(() => `starred:${envId.value}`);

  const loadFromStorage = () => {
    const storedValue = localStorage.getItem(storageKey.value);
    try {
      const parsed = storedValue ? JSON.parse(storedValue) : null;
      starredEvents.value = new Set(Array.isArray(parsed) ? parsed : []);
    } catch {
      starredEvents.value = new Set();
    }
  };

  const encodedSharedSchedule = computed(() => {
    const queryParam = route.query.share;
    if (typeof queryParam === "string") {
      return queryParam;
    }

    return undefined;
  });

  if (currentEnvId.value === undefined || currentEnvId.value !== envId.value) {
    loadFromStorage();
    currentEnvId.value = envId.value;
  }

  watchEffect(() => {
    // Don't overwrite the user's real starred events while viewing a shared schedule.
    if (!encodedSharedSchedule.value) {
      debouncedSetItem(storageKey.value, starredEvents.value);
    }
  });

  watch(encodedSharedSchedule, (newEncodedSharedSchedule, oldEncodedSharedSchedule) => {
    if (!newEncodedSharedSchedule) {
      if (oldEncodedSharedSchedule) {
        loadFromStorage();
      }

      return;
    }

    try {
      const decoded = decodeBase64url(newEncodedSharedSchedule);
      starredEvents.value = new Set([...decoded.split(",")]);
    } catch {
      starredEvents.value = new Set();
    }
  }, { immediate: true });

  return starredEvents;
};

export default useStarredEvents;
