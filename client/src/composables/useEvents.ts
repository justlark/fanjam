import { type Ref, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type Event } from "@/utils/api";

const eventsCache: Map<string, Array<Event>> = new Map();

export interface UseEventsReturn {
  events: Readonly<Ref<Array<Event>>>;
  reload: () => Promise<void>;
}

// TODO: Eventually, we'll want to persist this data to the local storage so
// the app can work offline.
export const useEvents = (): UseEventsReturn => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const events = ref<Array<Event>>([]);

  const reload = async (): Promise<void> => {
    const eventsResult = await api.getEvents(envId.value);

    if (eventsResult.ok) {
      eventsCache.set(envId.value, eventsResult.value);
      events.value = eventsResult.value;
    }
  };

  watchEffect(async () => {
    if (eventsCache.has(envId.value)) {
      events.value = eventsCache.get(envId.value) || [];
      return;
    }

    await reload();
  });

  return { events, reload };
};

export default useEvents;
