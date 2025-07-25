import { type Ref, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type Event, type Info } from "@/utils/api";

const fetchCache: Map<string, unknown> = new Map();

export type FetchResult<T> =
  | { status: "success"; value: T }
  | { status: "pending" }
  | { status: "error"; code: number };

function unwrapFetchResult<T>(
  result: Readonly<Ref<FetchResult<T>>>,
  defaultValue: T,
): Readonly<Ref<T>>;

function unwrapFetchResult<T>(
  result: Readonly<Ref<FetchResult<T>>>,
  defaultValue?: T,
): Readonly<Ref<T | undefined>> {
  return computed(() => (result.value.status === "success" ? result.value.value : defaultValue));
}

const useRemoteData = <T, S>({
  key,
  instance,
  result,
  fetcher,
  toCache,
  fromCache,
}: {
  key: string;
  instance: Readonly<Ref<string>>;
  result: Ref<FetchResult<T>>;
  fetcher: () => Promise<Extract<FetchResult<T>, { status: "success" | "error" }>>;
  // We want the option to change the shape of the data in memory without
  // changing the shape of the data in the browser local storage, since the
  // latter would have to be migrated or deleted.
  toCache: (data: T) => S;
  fromCache: (data: S) => T;
}): { reload: () => Promise<void> } => {
  const cacheKey = computed(() => `${key}:${instance.value}`);
  const instanceStorageKey = computed(() => `${key}:key`);
  const valueStorageKey = computed(() => `${key}:value`);

  // Fetch the most recent data from the server and update the ref.
  const reload = async (): Promise<void> => {
    const fetchResult = await fetcher();

    if (fetchResult.status === "success") {
      result.value = { status: "success", value: fetchResult.value };

      fetchCache.set(cacheKey.value, toCache(fetchResult.value));

      // We use the browser local storage to cut down on the initial page load
      // time and to allow the app to function offline.
      //
      // Because the browser will only give us so much storage space per
      // origin, and because users are unlikely to be attending multiple cons
      // simultaneously, we only cache the data for the current environment.
      //
      // However, we need to keep track of *which* environment we're caching
      // data for, so we know to invalidate the cache if the user switches to a
      // different environment.
      localStorage.setItem(instanceStorageKey.value, instance.value);
      localStorage.setItem(valueStorageKey.value, JSON.stringify(toCache(fetchResult.value)));
    } else if (result.value.status === "pending") {
      // If the API request succeeded previously, we don't want to show the
      // user an error and wipe the screen; we can just keep displaying the
      // data that's currently cached.
      //
      // If the API request never succeeded in the first place, then we should
      // show an error, because we have nothing else to show the user.
      result.value = { status: "error", code: fetchResult.code };
    }
  };

  watchEffect(async () => {
    if (fetchCache.has(cacheKey.value)) {
      result.value = { status: "success", value: fromCache(fetchCache.get(cacheKey.value) as S) };
      return;
    } else {
      const storedInstance = localStorage.getItem(instanceStorageKey.value);

      if (storedInstance !== instance.value) {
        localStorage.removeItem(instanceStorageKey.value);
        localStorage.removeItem(valueStorageKey.value);
      }

      const storedValue = localStorage.getItem(valueStorageKey.value);

      if (storedValue) {
        const value = fromCache(JSON.parse(storedValue));

        result.value = { status: "success", value };

        fetchCache.set(cacheKey.value, toCache(value));
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
    key: "events",
    instance: envId,
    result: eventsRef,
    fetcher: async () => {
      const result = await api.getEvents(envId.value);
      return result.ok
        ? { status: "success", value: result.value }
        : { status: "error", code: result.code };
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

  return { reload, result: eventsRef, value: unwrapFetchResult(eventsRef, []) };
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
    key: "info",
    instance: envId,
    result: infoRef,
    fetcher: async () => {
      const result = await api.getInfo(envId.value);
      return result.ok
        ? { status: "success", value: result.value }
        : { status: "error", code: result.code };
    },
    toCache: (data) => data,
    fromCache: (data) => data,
  });

  return { reload, result: infoRef, value: unwrapFetchResult(infoRef, undefined) };
};
