import {
  type Ref,
  type MaybeRefOrGetter,
  type Reactive,
  type DeepReadonly,
  toValue,
  toRef,
  reactive,
  provide,
  onMounted,
  readonly,
  inject,
  ref,
  computed,
  watch,
} from "vue";
import { useRoute } from "vue-router";
import api, { type ApiResult, type Config, type Event, type Info, type Page } from "@/utils/api";

export type FetchResult<T> =
  | { status: "success"; value: T; etag?: string }
  | { status: "pending" }
  | { status: "error"; code: number };

interface StoredValue<T> {
  instance: string;
  etag?: string;
  value: T;
}

const unwrapFetchValue = <T>(
  result: Readonly<Ref<FetchResult<T>>>,
): Readonly<Ref<T | undefined>> => {
  return computed(() => (result.value.status === "success" ? result.value.value : undefined));
};

const unwrapFetchStatus = (
  result: Readonly<Ref<FetchResult<unknown>>>,
): Readonly<Ref<FetchResult<unknown>["status"]>> => computed(() => result.value.status);

// Feed the `input` array into the `output` array in chunks, yielding to the
// browser to render one chunk at a time.
const lazyRenderArray = <T>(
  input: MaybeRefOrGetter<FetchResult<Array<T>>>,
  output: Reactive<Array<T>>,
  chunkSize: number,
) => {
  const renderChunk = () => {
    const value = toValue(input);

    if (value.status !== "success" || output.length >= value.value.length) {
      return;
    }

    const next = value.value.slice(output.length, output.length + chunkSize);
    (output as Array<T>).push(...next);

    requestAnimationFrame(renderChunk);
  };

  onMounted(() => {
    watch(
      input,
      () => {
        output.length = 0;
        requestAnimationFrame(renderChunk);
      },
      { immediate: true },
    );
  });
};

const setResultIfModified = <T>(
  result: Ref<FetchResult<T>>,
  value: T,
  serialize: (data: T) => unknown,
) => {
  if (
    result.value.status !== "success" ||
    JSON.stringify(serialize(result.value.value)) !== JSON.stringify(serialize(value))
  ) {
    result.value = { status: "success", value };
  }
};

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
      setResultIfModified(result, fromCache(storedValue.value), toCache);
    } else if (fetchResult.status === "success") {
      setResultIfModified(result, fetchResult.value, toCache);

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

  onMounted(() => {
    const storedValue = getItem<S>(key);

    if (!storedValue || storedValue.instance !== instance.value) {
      // Fetch the data on the initial page load, before it's cached locally.
      void reload();
      return;
    }

    let value;

    try {
      value = fromCache(storedValue.value);
    } catch {
      // This can happen if the shape of the cached data has changed and we
      // need to clear it and re-fetch from the server.
      void reload();
      return;
    }

    setResultIfModified(result, value, toCache);

    // Once the cached data has been loaded, refetch the latest data in the
    // background.
    void reload();

    watch(instance, reload);
  });

  return { reload, clear };
};

type DataSource<T> = (envId: MaybeRefOrGetter<string>) => {
  data: T;
  status: Readonly<Ref<FetchResult<unknown>["status"]>>;
  reload: () => Promise<void>;
  clear: () => void;
};

interface StoredEvent {
  id: string;
  name: string;
  summary?: string;
  description?: string;
  start_time: string;
  end_time?: string;
  location?: string;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

const eventsRef = ref<FetchResult<Array<Event>>>({ status: "pending" });

const useRemoteEvents: DataSource<Readonly<Reactive<Array<DeepReadonly<Event>>>>> = (
  envId: MaybeRefOrGetter<string>,
) => {
  const storedValue: StoredValue<unknown> | undefined = getItem("events");

  const { reload, clear } = useRemoteDataInner<Array<Event>, Array<StoredEvent>>({
    key: "events",
    instance: toRef(envId),
    result: eventsRef,
    fetcher: () => api.getEvents(toValue(envId), storedValue?.etag),
    toCache: (data) =>
      data.map((event) => ({
        id: event.id,
        name: event.name,
        summary: event.summary,
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
        summary: event.summary,
        description: event.description,
        startTime: new Date(event.start_time),
        endTime: event.end_time ? new Date(event.end_time) : undefined,
        location: event.location,
        people: event.people,
        category: event.category,
        tags: event.tags,
      })),
  });

  const data = reactive<Array<Event>>([]);
  lazyRenderArray(eventsRef, data, 5);

  return {
    reload,
    clear,
    status: unwrapFetchStatus(eventsRef),
    data: readonly(data),
  };
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

const useRemoteInfo: DataSource<Readonly<Ref<Info | undefined>>> = (
  envId: MaybeRefOrGetter<string>,
) => {
  const storedValue: StoredValue<unknown> | undefined = getItem("info");

  const { reload, clear } = useRemoteDataInner<Info, StoredInfo>({
    key: "info",
    instance: toRef(envId),
    result: infoRef,
    fetcher: () => api.getInfo(toValue(envId), storedValue?.etag),
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

  return {
    reload,
    clear,
    status: unwrapFetchStatus(infoRef),
    data: unwrapFetchValue(infoRef),
  };
};

interface StoredPage {
  id: string;
  title: string;
  body: string;
}

const pagesRef = ref<FetchResult<Array<Page>>>({ status: "pending" });

const useRemotePages: DataSource<Readonly<Reactive<Array<Page>>>> = (
  envId: MaybeRefOrGetter<string>,
) => {
  const storedValue: StoredValue<unknown> | undefined = getItem("pages");

  const { reload, clear } = useRemoteDataInner<Array<Page>, Array<StoredPage>>({
    key: "pages",
    instance: toRef(envId),
    result: pagesRef,
    fetcher: () => api.getPages(toValue(envId), storedValue?.etag),
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

  const data = reactive<Array<Page>>([]);
  lazyRenderArray(pagesRef, data, 5);

  return { reload, clear, status: unwrapFetchStatus(pagesRef), data: readonly(data) };
};

interface StoredConfig {
  timezone?: string;
}

const configRef = ref<FetchResult<Config>>({ status: "pending" });

const useRemoteConfig: DataSource<Readonly<Ref<Config | undefined>>> = (
  envId: MaybeRefOrGetter<string>,
) => {
  const { reload, clear } = useRemoteDataInner<Config, StoredConfig>({
    key: "config",
    instance: toRef(envId),
    result: configRef,
    fetcher: () => api.getConfig(toValue(envId)),
    toCache: (data) => ({
      timezone: data.timezone,
    }),
    fromCache: (data) => ({
      timezone: data.timezone,
    }),
  });

  return {
    reload,
    clear,
    data: unwrapFetchValue(configRef),
    status: unwrapFetchStatus(configRef),
  };
};

const dataSources = {
  events: useRemoteEvents,
  info: useRemoteInfo,
  pages: useRemotePages,
  config: useRemoteConfig,
} as const;

type CombinedDataSource = () => {
  data: {
    [K in keyof typeof dataSources]: ReturnType<(typeof dataSources)[K]>["data"];
  };
  status: {
    [K in keyof typeof dataSources]: ReturnType<(typeof dataSources)[K]>["status"];
  };
  reload: () => Promise<void>;
  clear: () => void;
};

// We fetch *all* data from the server eagerly on first page load and when
// `reload()` is called. This is primarily so the app works offline.
const useRemoteData: CombinedDataSource = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const dataSourceResponses = Object.fromEntries(
    Object.entries(dataSources).map(([key, ds]) => [key, ds(envId)]),
  );

  const reload = async () => {
    await Promise.all(Object.values(dataSourceResponses).map((ds) => ds.reload()));
  };

  const clear = () => {
    for (const ds of Object.values(dataSourceResponses)) {
      ds.clear();
    }
  };

  return {
    reload,
    clear,
    status: Object.fromEntries(
      Object.entries(dataSourceResponses).map(([key, ds]) => [key, ds.status]),
    ) as ReturnType<CombinedDataSource>["status"],
    data: Object.fromEntries(
      Object.entries(dataSourceResponses).map(([key, ds]) => [key, ds.data]),
    ) as ReturnType<CombinedDataSource>["data"],
  };
};

const remoteDataKey = Symbol("data");

export const provideRemoteData = () => {
  provide(remoteDataKey, useRemoteData());
};

const injectRemoteData = () => {
  const data = inject<ReturnType<typeof useRemoteData>>(remoteDataKey);

  if (!data) {
    throw new Error("Views must be wrapped in a <PageRoot></PageRoot>.");
  }

  return data;
};

export default injectRemoteData;
