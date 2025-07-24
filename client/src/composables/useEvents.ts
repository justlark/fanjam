import { type Ref, ref, computed, watch } from "vue";
import { useRoute } from "vue-router";
import api, { type Event } from "@/utils/api";

const EVENTS_LOCAL_STORAGE_KEY = "events";
const ENV_ID_LOCAL_STORAGE_KEY = "env_id";

const eventsCache: Map<string, Array<Event>> = new Map();

interface LocalStorageEvent {
  id: string;
  name: string;
  description?: string;
  start_time: string;
  end_time?: string;
  location?: string;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

const toLocalStorageEvent = (event: Event): LocalStorageEvent => ({
  id: event.id,
  name: event.name,
  description: event.description,
  start_time: event.startTime.toISOString(),
  end_time: event.endTime ? event.endTime.toISOString() : undefined,
  location: event.location,
  people: event.people,
  category: event.category,
  tags: event.tags,
});

const fromLocalStorageEvent = (event: LocalStorageEvent): Event => ({
  id: event.id,
  name: event.name,
  description: event.description,
  startTime: new Date(event.start_time),
  endTime: event.end_time ? new Date(event.end_time) : undefined,
  location: event.location,
  people: event.people,
  category: event.category,
  tags: event.tags,
});

export interface UseEventsReturn {
  events: Readonly<Ref<Array<Event>>>;
  status: Readonly<Ref<"success" | "loading" | "not-found">>;
  reload: () => Promise<void>;
}

// TODO: Eventually, we'll want to persist this data to the local storage so
// the app can work offline.
export const useEvents = (): UseEventsReturn => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const events = ref<Array<Event>>([]);
  const status = ref<"success" | "loading" | "not-found">("loading");

  const reload = async (): Promise<void> => {
    const eventsResult = await api.getEvents(envId.value);

    // We don't throw and error or show an error page if getting the events
    // from the server fails, because this app should be able to work offline.
    // If it can't reach the server, it should just use the cached events from
    // the local storage.
    if (eventsResult.ok) {
      events.value = eventsResult.value;

      eventsCache.set(envId.value, eventsResult.value);

      // We store the env ID so we know to invalidate the local storage cache
      // when the user switches to a different environment. Because the browser
      // will limit the amount of storage we're allowed to use, we only want to
      // store the events for one environment at a time.
      localStorage.setItem(ENV_ID_LOCAL_STORAGE_KEY, envId.value);

      localStorage.setItem(
        EVENTS_LOCAL_STORAGE_KEY,
        JSON.stringify(eventsResult.value.map(toLocalStorageEvent)),
      );
    } else if (eventsResult.status === 404) {
      status.value = "not-found";
    }
  };

  watch(
    envId,
    async () => {
      if (eventsCache.has(envId.value)) {
        events.value = eventsCache.get(envId.value) || [];
        return;
      } else {
        const storedEnvId = localStorage.getItem(ENV_ID_LOCAL_STORAGE_KEY);

        if (storedEnvId !== envId.value) {
          localStorage.removeItem(EVENTS_LOCAL_STORAGE_KEY);
          localStorage.removeItem(ENV_ID_LOCAL_STORAGE_KEY);
        }

        const storedEvents = localStorage.getItem(EVENTS_LOCAL_STORAGE_KEY);

        // Even if the events were cached in the local storage, we still fetch
        // them from the API in the background, in case they updated on the
        // server since we last visited. However, in the meantime, we can show
        // the cached events.
        if (storedEvents) {
          const parsedEvents: Array<LocalStorageEvent> = JSON.parse(storedEvents);
          const newEvents = parsedEvents.map(fromLocalStorageEvent);

          events.value = newEvents;

          eventsCache.set(envId.value, newEvents);
        }
      }

      await reload();
    },
    { immediate: true },
  );

  watch(
    events,
    (newEvents) => {
      if (newEvents.length > 0) {
        status.value = "success";
      }
    },
    { immediate: true },
  );

  return { events, reload, status };
};

export default useEvents;
