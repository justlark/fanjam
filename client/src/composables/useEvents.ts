import { type Ref, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type Dump, type Event, type About } from "@/utils/api";

const EVENTS_LOCAL_STORAGE_KEY = "events";
const ENV_ID_LOCAL_STORAGE_KEY = "env_id";

const dumpCache: Map<string, Dump> = new Map();

const events = ref<Array<Event>>([]);
const about = ref<About>();
const status = ref<"success" | "loading" | "not-found">("loading");

interface LocalStorageDump {
  events: Array<{
    id: string;
    name: string;
    description?: string;
    start_time: string;
    end_time?: string;
    location?: string;
    people: Array<string>;
    category?: string;
    tags: Array<string>;
  }>;
  about?: {
    name: string;
    description?: string;
    link?: string;
  };
}

const toLocalStorageDump = (dump: Dump): LocalStorageDump => ({
  events: dump.events.map((event) => ({
    id: event.id,
    name: event.name,
    description: event.description,
    start_time: event.startTime.toISOString(),
    end_time: event.endTime ? event.endTime.toISOString() : undefined,
    location: event.location,
    people: event.people,
    category: event.category,
    tags: event.tags,
  })),
  about: dump.about
    ? {
        name: dump.about.name,
        description: dump.about.description,
        link: dump.about.link,
      }
    : undefined,
});

const fromLocalStorageDump = (dump: LocalStorageDump): Dump => ({
  events: dump.events.map((event) => ({
    id: event.id,
    name: event.name,
    description: event.description,
    startTime: new Date(event.start_time),
    endTime: event.end_time ? new Date(event.end_time) : undefined,
    location: event.location,
    people: event.people,
    category: event.category,
    tags: event.tags,
  })),
  about: dump.about
    ? {
        name: dump.about.name,
        description: dump.about.description,
        link: dump.about.link,
      }
    : undefined,
});

export interface UseEventsReturn {
  events: Readonly<Ref<Array<Event>>>;
  about: Readonly<Ref<About | undefined>>;
  status: Readonly<Ref<"success" | "loading" | "not-found">>;
  reload: () => Promise<void>;
}

export const useEvents = (): UseEventsReturn => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const reload = async (): Promise<void> => {
    const dumpResult = await api.getDump(envId.value);

    // We don't throw and error or show an error page if getting the events
    // from the server fails, because this app should be able to work offline.
    // If it can't reach the server, it should just use the cached events from
    // the local storage.
    if (dumpResult.ok) {
      events.value = dumpResult.value.events;
      about.value = dumpResult.value.about;

      dumpCache.set(envId.value, dumpResult.value);

      // We store the env ID so we know to invalidate the local storage cache
      // when the user switches to a different environment. Because the browser
      // will limit the amount of storage we're allowed to use, we only want to
      // store the events for one environment at a time.
      localStorage.setItem(ENV_ID_LOCAL_STORAGE_KEY, envId.value);

      localStorage.setItem(
        EVENTS_LOCAL_STORAGE_KEY,
        JSON.stringify(toLocalStorageDump(dumpResult.value)),
      );
    } else if (dumpResult.status === 404) {
      status.value = "not-found";
    }
  };

  watchEffect(async () => {
    if (dumpCache.has(envId.value)) {
      events.value = dumpCache.get(envId.value)?.events ?? [];
      about.value = dumpCache.get(envId.value)?.about;
      return;
    } else {
      const storedEnvId = localStorage.getItem(ENV_ID_LOCAL_STORAGE_KEY);

      if (storedEnvId !== envId.value) {
        localStorage.removeItem(EVENTS_LOCAL_STORAGE_KEY);
        localStorage.removeItem(ENV_ID_LOCAL_STORAGE_KEY);
      }

      const storedDump = localStorage.getItem(EVENTS_LOCAL_STORAGE_KEY);

      // Even if the events were cached in the local storage, we still fetch
      // them from the API in the background, in case they updated on the
      // server since we last visited. However, in the meantime, we can show
      // the cached events.
      if (storedDump) {
        const parsedDump: LocalStorageDump = JSON.parse(storedDump);
        const newDump = fromLocalStorageDump(parsedDump);

        events.value = newDump.events;
        about.value = newDump.about;

        dumpCache.set(envId.value, newDump);
      }
    }

    await reload();
  });

  watchEffect(() => {
    if (events.value.length > 0) {
      status.value = "success";
    }
  });

  return { events, about, reload, status };
};

export default useEvents;
