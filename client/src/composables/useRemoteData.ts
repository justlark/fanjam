import { type Ref, toRef, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type ApiResult, type Event, type Info, type Page } from "@/utils/api";

export type FetchResult<T> =
  | { status: "success"; value: T; etag?: string }
  | { status: "pending" }
  | { status: "error"; code: number };

interface StoredValue<T> {
  instance: string;
  etag?: string;
  value: T;
}

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

const hasErrorCode = (result: FetchResult<unknown>, code: number): boolean =>
  result.status === "error" && result.code === code;

const storageKey = (key: string): string => `store:${key}`;

const setItem = (key: string, value: unknown): void => {
  localStorage.setItem(storageKey(key), JSON.stringify(value));
};

const getItem = <T>(key: string): StoredValue<T> | undefined => {
  const serialized = localStorage.getItem(storageKey(key));
  return serialized ? (JSON.parse(serialized) as StoredValue<T>) : undefined;
};

const removeItem = (key: string): void => {
  localStorage.removeItem(storageKey(key));
};

const hasLoaded = new Set<string>();

const useRemoteDataInner = <T, S>({
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
  fetcher: () => Promise<ApiResult<T>>;
  // Some values may need to be serialized manually before being stored.
  toCache: (data: T) => S;
  fromCache: (data: S) => T;
}): {
  reload: () => Promise<void>;
  clear: () => void;
} => {
  // Fetch the most recent data from the server and update the ref.
  const reload = async (): Promise<void> => {
    const fetchApiResult = await fetcher();
    const fetchResult: FetchResult<T> = fetchApiResult.ok
      ? { status: "success", value: fetchApiResult.value, etag: fetchApiResult.etag }
      : { status: "error", code: fetchApiResult.code };

    const storedValue = getItem<S>(key);

    if (fetchResult.status === "error" && fetchResult.code === 304 && storedValue !== undefined) {
      // If the server returns a 304 Not Modified, we can just keep displaying
      // the data we already have cached locally.
      result.value = { status: "success", value: fromCache(storedValue.value) };
    } else if (fetchResult.status === "success") {
      result.value = { status: "success", value: fetchResult.value };

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

      const storedValue: StoredValue<S> = {
        instance: instance.value,
        etag: fetchResult.etag,
        value: toCache(fetchResult.value),
      };

      setItem(key, storedValue);
    } else if (result.value.status === "pending") {
      // If the API request succeeded previously, we can just keep displaying
      // the data that's currently cached.
      //
      // If the API request never succeeded in the first place, then we should
      // show an error, because we have nothing else to show the user.
      result.value = { status: "error", code: fetchResult.code };
    }
  };

  const clear = () => {
    removeItem(key);
  };

  watchEffect(async () => {
    const storedValue = getItem<S>(key);

    if (!storedValue || storedValue.instance !== instance.value) {
      await reload();
      return;
    }

    let value;

    try {
      value = fromCache(storedValue.value);
    } catch {
      // This can happen if the shape of the cached data has changed and we
      // need to clear it and re-fetch from the server.
      await reload();
      return;
    }

    result.value = { status: "success", value };

    // Refetch the data exactly once when the user refreshes the page.
    if (!hasLoaded.has(key)) {
      hasLoaded.add(key);
      await reload();
    }
  });

  return { reload, clear };
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

const useRemoteEvents = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storedValue: StoredValue<unknown> | undefined = getItem("events");

  const { reload, clear } = useRemoteDataInner<Array<Event>, Array<StoredEvent>>({
    key: "events",
    instance: envId,
    result: eventsRef,
    fetcher: () => api.getEvents(envId.value, storedValue?.etag),
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

  return { reload, clear, result: eventsRef };
};

interface StoredInfo {
  name?: string;
  description?: string;
  website_url?: string;
  links: Array<{
    name: string;
    url: string;
  }>;
  files: Array<{
    name: string;
    media_type: string;
    signed_url: string;
  }>;
}

const infoRef = ref<FetchResult<Info>>({ status: "pending" });

const useRemoteInfo = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storedValue: StoredValue<unknown> | undefined = getItem("info");

  const { reload, clear } = useRemoteDataInner<Info, StoredInfo>({
    key: "info",
    instance: envId,
    result: infoRef,
    fetcher: () => api.getInfo(envId.value, storedValue?.etag),
    toCache: (data) => ({
      name: data.name,
      description: data.description,
      website_url: data.websiteUrl,
      links: data.links.map((link) => ({ name: link.name, url: link.url })),
      files: data.files.map((file) => ({
        name: file.name,
        media_type: file.mediaType,
        signed_url: file.signedUrl,
      })),
    }),
    fromCache: (data) => ({
      name: data.name,
      description: data.description,
      websiteUrl: data.website_url,
      links: data.links.map((link) => ({ name: link.name, url: link.url })),
      files: data.files.map((file) => ({
        name: file.name,
        mediaType: file.media_type,
        signedUrl: file.signed_url,
      })),
    }),
  });

  return { reload, clear, result: infoRef };
};

interface StoredPage {
  id: string;
  title: string;
  body: string;
}

const pagesRef = ref<FetchResult<Array<Page>>>({ status: "pending" });

const useRemotePages = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storedValue: StoredValue<unknown> | undefined = getItem("pages");

  const { reload, clear } = useRemoteDataInner<Array<Page>, Array<StoredPage>>({
    key: "pages",
    instance: envId,
    result: pagesRef,
    fetcher: () => api.getPages(envId.value, storedValue?.etag),
    toCache: (data) =>
      data.map((page) => ({
        id: page.id,
        title: page.title,
        body: page.body,
      })),
    fromCache: (data) =>
      data.map((page) => ({
        id: page.id,
        title: page.title,
        body: page.body,
      })),
  });

  return { reload, clear, result: pagesRef };
};

// We fetch *all* data from the server eagerly on first page load and when
// `reload()` is called. This is primarily so the app works offline.
const useRemoteData = () => {
  const { reload: reloadEvents, clear: clearEvents, result: eventsResult } = useRemoteEvents();

  const { reload: reloadInfo, clear: clearInfo, result: infoResult } = useRemoteInfo();

  const { reload: reloadPages, clear: clearPages, result: pagesResult } = useRemotePages();

  const reload = async () => {
    await Promise.all([reloadEvents(), reloadInfo(), reloadPages()]);
  };

  const clear = () => {
    clearEvents();
    clearInfo();
    clearPages();
  };

  const isNotFound = computed(
    () =>
      hasErrorCode(eventsResult.value, 404) ||
      hasErrorCode(infoResult.value, 404) ||
      hasErrorCode(pagesResult.value, 404),
  );

  return {
    reload,
    clear,
    isNotFound,
    status: {
      events: toRef(() => eventsResult.value.status),
      info: toRef(() => infoResult.value.status),
      pages: toRef(() => pagesResult.value.status),
    },
    data: {
      events: unwrapFetchResult(eventsResult, []),
      info: unwrapFetchResult(infoResult, undefined),
      pages: unwrapFetchResult(pagesResult, []),
    },
  };
};

export default useRemoteData;
