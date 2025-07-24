import { type Ref, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type Dump, type Event, type About, type Link, type Info } from "@/utils/api";

const EVENTS_LOCAL_STORAGE_KEY = "events";
const ENV_ID_LOCAL_STORAGE_KEY = "env_id";

const dumpCache: Map<string, Dump> = new Map();

const events = ref<Array<Event>>([]);
const about = ref<About>();
const links = ref<Array<Link>>([]);
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
    website_url?: string;
  };
  links: Array<{
    name: string;
    url: string;
  }>;
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
      website_url: dump.about.websiteUrl,
    }
    : undefined,
  links: dump.links.map((link) => ({
    name: link.name,
    url: link.url,
  })),
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
      websiteUrl: dump.about.website_url,
    }
    : undefined,
  links: dump.links.map((link) => ({
    name: link.name,
    url: link.url,
  })),
});

export interface UseEventsReturn {
  events: Readonly<Ref<Array<Event>>>;
  about: Readonly<Ref<About | undefined>>;
  links: Readonly<Ref<Array<Link>>>;
  status: Readonly<Ref<"success" | "loading" | "not-found">>;
  reload: () => Promise<void>;
}

// TODO: Refactor so we don't need to remember to update the refs (`events`,
// `about`, `links`, etc.) in multiple places.
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
      links.value = dumpResult.value.links;

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
      links.value = dumpCache.get(envId.value)?.links ?? [];

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
        links.value = newDump.links;

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

  return { events, about, links, reload, status };
};

export default useEvents;

// ---

const fetchCache: Map<string, unknown> = new Map();

export type FetchResult<T> =
  | { status: "success"; value: T }
  | { status: "pending" }
  | { status: "error"; code: number };

interface CacheKey {
  category: string;
  instance: string;
}

const useRemoteData = <T, S>({
  key,
  result,
  fetcher,
  toCache,
  fromCache,
}: {
  key: CacheKey;
  result: Ref<FetchResult<T>>;
  fetcher: () => Promise<Extract<FetchResult<T>, { status: "success" | "error" }>>;
  toCache: (data: NonNullable<T>) => S;
  fromCache: (data: S) => NonNullable<T>;
}): { reload: () => Promise<void> } => {
  const cacheKey = computed(() => `${key.category}:${key.instance}`);

  const reload = async (): Promise<void> => {
    const fetchResult = await fetcher();

    // We don't throw and error or show an error page if getting the events
    // from the server fails, because this app should be able to work offline.
    // If it can't reach the server, it should just use the cached events from
    // the local storage.
    if (fetchResult.status === "success") {
      result.value = { status: "success", value: fetchResult.value };

      fetchCache.set(cacheKey.value, fetchResult.value);

      // We store the env ID so we know to invalidate the local storage cache
      // when the user switches to a different environment. Because the browser
      // will limit the amount of storage we're allowed to use, we only want to
      // store the events for one environment at a time.
      localStorage.setItem(`${key.category}:key`, key.instance);

      if (fetchResult.value === undefined || fetchResult.value === null) {
        localStorage.removeItem(`${key.category}:value`);
      } else {
        localStorage.setItem(`${key.category}:value`, JSON.stringify(toCache(fetchResult.value)));
      }
    } else {
      result.value = { status: "error", code: fetchResult.code };
    }
  };

  watchEffect(async () => {
    if (fetchCache.has(cacheKey.value)) {
      result.value = { status: "success", value: fromCache(fetchCache.get(cacheKey.value) as S) };
      return;
    } else {
      const storedCacheKey = localStorage.getItem(`${key.category}:key`);

      if (storedCacheKey !== key.instance) {
        localStorage.removeItem(`${key.category}:key`);
        localStorage.removeItem(`${key.category}:value`);
      }

      const storedValue = localStorage.getItem(`${key.category}:value`);

      // Even if the events were cached in the local storage, we still fetch
      // them from the API in the background, in case they updated on the
      // server since we last visited. However, in the meantime, we can show
      // the cached events.
      if (storedValue) {
        const value = fromCache(JSON.parse(storedValue));

        result.value = { status: "success", value };

        fetchCache.set(cacheKey.value, value);
      }
    }

    await reload();
  });

  return { reload };
};

interface StoredEvent {
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

const eventsRef = ref<FetchResult<Array<Event>>>({ status: "pending" });

export const useRemoteEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const { reload } = useRemoteData<Array<Event>, Array<StoredEvent>>({
    key: { category: "events", instance: envId.value },
    result: eventsRef,
    fetcher: async () => {
      const result = await api.getDump(envId.value);
      if (result.ok) {
        return { status: "success", value: result.value.events };
      } else {
        return { status: "error", code: result.status };
      }
    },
    toCache: (data) =>
      data.map((event) => ({
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
    fromCache: (data) =>
      data.map((event) => ({
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
  });

  return { reload, events: eventsRef };
};

interface StoredInfo {
  about?: {
    name: string;
    description?: string;
    website_url?: string;
  };
  links: Array<{
    name: string;
    url: string;
  }>;
}

const infoRef = ref<FetchResult<Info>>({ status: "pending" });

export const useRemoteInfo = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const { reload } = useRemoteData<Info, StoredInfo>({
    key: { category: "events", instance: envId.value },
    result: infoRef,
    fetcher: async () => {
      const result = await api.getDump(envId.value);
      if (result.ok) {
        return {
          status: "success",
          value: { about: result.value.about, links: result.value.links },
        };
      } else {
        return { status: "error", code: result.status };
      }
    },
    toCache: (data) => data,
    fromCache: (data) => data,
  });

  return { reload, about: infoRef };
};
