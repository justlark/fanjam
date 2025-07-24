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

    if (fetchResult.status === "success") {
      result.value = { status: "success", value: fetchResult.value };

      if (fetchResult.value !== undefined && fetchResult.value !== null) {
        fetchCache.set(cacheKey.value, toCache(fetchResult.value));
        localStorage.setItem(`${key.category}:key`, key.instance);
        localStorage.setItem(`${key.category}:value`, JSON.stringify(toCache(fetchResult.value)));
      } else {
        fetchCache.delete(cacheKey.value);
        localStorage.removeItem(`${key.category}:key`);
        localStorage.removeItem(`${key.category}:value`);
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
    key: { category: "events", instance: envId.value },
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
    key: { category: "info", instance: envId.value },
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
