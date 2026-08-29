import {
  type Ref,
  type MaybeRefOrGetter,
  type DeepReadonly,
  toValue,
  toRef,
  provide,
  onMounted,
  inject,
  ref,
  computed,
  watch,
  nextTick,
} from "vue";
import { useRoute, useRouter } from "vue-router";
import api, {
  type Announcement,
  type ApiResult,
  type Config,
  type Event,
  type Info,
  type Page,
  type Person,
} from "@/utils/api";
import { envContext } from "@/context";
import useEnvId from "./useEnvId";
import useScheduleSync from "./useScheduleSync";
import { onIdle } from "@/utils/idle";

export type FetchResult<T> =
  | { status: "success"; value: T; etag?: string }
  | { status: "pending" }
  | { status: "error"; code: number | "offline" };

// The status views switch on. `"offline"` is derived from the error code rather
// than being its own `FetchResult` variant, so the code stays the single source
// of truth for why a fetch failed.
type FetchStatus = FetchResult<unknown>["status"] | "offline";

// An API endpoint is either refetched on every reload (`"global"`), or only on
// specific routes.
type FetchPolicy = "global" | ReadonlyArray<string>;

const matchesRoute = (policy: FetchPolicy, routeName: string | undefined): boolean =>
  policy === "global" || (routeName !== undefined && policy.includes(routeName));

interface StoredValue<T> {
  instance: string;
  etag?: string;
  // When the server last refetched this entry, as epoch milliseconds.
  fetched_at?: number;
  value: T;
}

const unwrapFetchValue = <T>(result: Readonly<Ref<FetchResult<T>>>): Readonly<Ref<T | undefined>> =>
  computed(() => (result.value.status === "success" ? result.value.value : undefined));

const unwrapFetchArray = <T>(
  result: Readonly<Ref<FetchResult<Array<T>>>>,
): Readonly<Ref<Array<T>>> =>
  computed(() => (result.value.status === "success" ? result.value.value : []));

const unwrapFetchStatus = (
  result: Readonly<Ref<FetchResult<unknown>>>,
): Readonly<Ref<FetchStatus>> =>
  computed(() =>
    result.value.status === "error" && result.value.code === "offline"
      ? "offline"
      : result.value.status,
  );

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

// Rewrite when the data was last refetched, leaving the value itself alone.
const setStoredFetchedAt = (key: string, fetchedAt: number): void => {
  const stored = getItem<unknown>(key);
  if (stored === undefined) return;
  setItem(key, { ...stored, fetched_at: fetchedAt });
};

const useRemoteDataInner = <T, S>({
  key,
  instance,
  result,
  fetcher,
  toCache,
  fromCache,
  fetchPolicy,
}: {
  key: string;
  instance: Readonly<Ref<string>>;
  result: Ref<FetchResult<T>>;
  fetcher: () => Promise<ApiResult<T>>;
  // Some values may need to be serialized manually before being stored.
  toCache: (data: T) => S;
  fromCache: (data: S) => T;
  fetchPolicy: FetchPolicy;
}): {
  reload: () => Promise<void>;
  invalidate: () => void;
} => {
  const route = useRoute();
  const router = useRouter();

  const shouldFetch = (): boolean => matchesRoute(fetchPolicy, route.name as string | undefined);

  const BASE_RETRY_DELAY_MS = 1500;

  const MAX_RETRIES = 3;

  let retryCount = 0;
  let retryTimeout: ReturnType<typeof setTimeout> | undefined;

  const cancelRetry = () => {
    if (retryTimeout !== undefined) {
      clearTimeout(retryTimeout);
      retryTimeout = undefined;
    }
  };

  // Retry with exponential backoff, jittered across half the delay. The jitter
  // matters: clients go stale together, because they go stale when the shared
  // edge cache entry expires. Without jitter they'd all come back in lockstep,
  // and that synchronized burst is what turns a slow upstream into a dead one.
  const scheduleRetry = () => {
    if (retryCount >= MAX_RETRIES) return;
    const delay = BASE_RETRY_DELAY_MS * Math.pow(2, retryCount);
    const jittered = delay * (0.5 + Math.random() * 0.5);
    retryCount++;
    retryTimeout = setTimeout(() => {
      retryTimeout = undefined;
      void reload();
    }, jittered);
  };

  // Fetch the most recent data from the server and update the ref.
  const reload = async (): Promise<void> => {
    cancelRetry();
    const fetchApiResult = await fetcher();

    if (!fetchApiResult.ok && fetchApiResult.code === 304) {
      // Server returned 304 Not Modified; the cached data is still current.
      retryCount = 0;
      setStoredFetchedAt(key, Date.now());
      return;
    }

    if (!fetchApiResult.ok && fetchApiResult.code === "offline") {
      // We never reached the server. Don't mistake this for a 404 that means
      // we've switched environments.
      //
      // We don't schedule a retry; retrying is only for waiting for the edge
      // cache to repopulate. Reset the retry count.
      retryCount = 0;

      if (result.value.status === "pending") {
        // Nothing was cached, so there is nothing to show. Say so, rather than
        // leaving the view to spin on `"pending"` forever.
        result.value = { status: "error", code: "offline" };
      }

      return;
    }

    const fetchResult: FetchResult<T> = fetchApiResult.ok
      ? { status: "success", value: fetchApiResult.value, etag: fetchApiResult.etag }
      : { status: "error", code: fetchApiResult.code };

    if (fetchResult.status === "error" && fetchResult.code === 404) {
      // If the API request returned a 404, that may mean the environment ID
      // has changed. Check if the current environment ID is an alias, in which
      // case we need to redirect.
      const aliasResult = await api.getAlias(instance.value);

      if (aliasResult.ok) {
        const currentStoredValue = getItem<S>(key);

        if (currentStoredValue !== undefined && currentStoredValue.instance !== aliasResult.value) {
          // Update the stored instance ID to the new environment ID.
          currentStoredValue.instance = aliasResult.value;
          setItem(key, currentStoredValue);
        }

        if (envContext.mode === "custom") {
          // When using a custom domain, the env ID is baked into the DOM by
          // the client worker. If the env ID changes server-side, we need to reload the
          // page.
          window.location.reload();
        } else {
          await router.push({
            name: route.name as string,
            params: {
              ...route.params,
              envId: aliasResult.value,
            },
            query: route.query,
          });
        }
      } else {
        // This environment ID is not a valid alias; it just doesn't exist.
        result.value = { status: "error", code: fetchResult.code };
      }
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
        fetched_at: Date.now(),
        value: toCache(fetchResult.value),
      };

      setItem(key, storedValue);

      // The server classifies how current the data is; we decide our own retry
      // schedule from that. Only `"stale"` means fresher data is on its way.
      // `"backoff"` means upstream has been failing and refreshes are in
      // cooldown, so there is nothing to come back for, and retrying would only
      // add to the load that's keeping it down.
      if (fetchApiResult.ok && fetchApiResult.freshness === "stale") {
        scheduleRetry();
      } else {
        retryCount = 0;
      }
    } else if (result.value.status === "pending") {
      // If the API request succeeded previously, we can just keep displaying
      // the data that's currently cached.
      //
      // If the API request never succeeded in the first place, then we should
      // show an error, because we have nothing else to show the user.
      result.value = { status: "error", code: fetchResult.code };
    }
  };

  const invalidate = () => {
    setStoredFetchedAt(key, 0);
  };

  onMounted(() => {
    watch(
      instance,
      () => {
        cancelRetry();
        retryCount = 0;

        const storedValue = getItem<S>(key);

        if (!storedValue || storedValue.instance !== instance.value) {
          removeItem(key);
          if (shouldFetch()) {
            void reload();
          } else {
            result.value = { status: "pending" };
          }
          return;
        }

        let value;

        try {
          value = fromCache(storedValue.value);
        } catch {
          // This can happen if the shape of the cached data has changed and we
          // need to clear it and re-fetch from the server.
          if (shouldFetch()) {
            void reload();
          } else {
            result.value = { status: "pending" };
          }
          return;
        }

        setResultIfModified(result, value, toCache);

        // Once the cached data has been loaded, refetch the latest data in the
        // background, but only when it is relevant for the current route.
        if (shouldFetch()) {
          void reload();
        }
      },
      { immediate: true },
    );
  });

  return { reload, invalidate };
};

type DataSource<T> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  data: T;
  status: Readonly<Ref<FetchStatus>>;
  reload: () => Promise<void>;
  invalidate: () => void;
};

interface StoredEvent {
  id: string;
  name: string;
  summary?: string;
  description?: string;
  start_time: string;
  end_time?: string;
  location?: string;
  people: Array<{
    id: string;
    name: string;
  }>;
  category?: string;
  tags: Array<string>;
}

const eventsRef = ref<FetchResult<Array<Event>>>({ status: "pending" });

const useRemoteEvents: DataSource<Readonly<Ref<Array<DeepReadonly<Event>>>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Array<Event>, Array<StoredEvent>>({
    key: "events",
    instance: toRef(envId),
    fetchPolicy,
    result: eventsRef,
    fetcher: () => api.getEvents(toValue(envId), getItem<Array<StoredEvent>>("events")?.etag),
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

  return {
    reload,
    invalidate,
    status: unwrapFetchStatus(eventsRef),
    data: unwrapFetchArray(eventsRef),
  };
};

interface StoredPerson {
  id: string;
  name: string;
  bio?: string;
}

const peopleRef = ref<FetchResult<Array<Person>>>({ status: "pending" });

const useRemotePeople: DataSource<Readonly<Ref<Array<DeepReadonly<Person>>>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Array<Person>, Array<StoredPerson>>({
    key: "people",
    instance: toRef(envId),
    fetchPolicy,
    result: peopleRef,
    fetcher: () => api.getPeople(toValue(envId), getItem<Array<StoredPerson>>("people")?.etag),
    toCache: (data) =>
      data.map((person) => ({
        id: person.id,
        name: person.name,
        bio: person.bio,
      })),
    fromCache: (data) =>
      data.map((person) => ({
        id: person.id,
        name: person.name,
        bio: person.bio,
      })),
  });

  return {
    reload,
    invalidate,
    status: unwrapFetchStatus(peopleRef),
    data: unwrapFetchArray(peopleRef),
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
    id: string;
    name: string;
    media_type: string;
  }>;
}

const infoRef = ref<FetchResult<Info>>({ status: "pending" });

const useRemoteInfo: DataSource<Readonly<Ref<Info | undefined>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Info, StoredInfo>({
    key: "info",
    instance: toRef(envId),
    fetchPolicy,
    result: infoRef,
    fetcher: () => api.getInfo(toValue(envId), getItem<StoredInfo>("info")?.etag),
    toCache: (data) => ({
      name: data.name,
      description: data.description,
      website_url: data.websiteUrl,
      links: data.links.map((link) => ({ name: link.name, url: link.url })),
      files: data.files.map((file) => ({
        id: file.id,
        name: file.name,
        media_type: file.mediaType,
      })),
    }),
    fromCache: (data) => ({
      name: data.name,
      description: data.description,
      websiteUrl: data.website_url,
      links: data.links.map((link) => ({ name: link.name, url: link.url })),
      files: data.files.map((file) => ({
        id: file.id,
        name: file.name,
        mediaType: file.media_type,
      })),
    }),
  });

  return {
    reload,
    invalidate,
    status: unwrapFetchStatus(infoRef),
    data: unwrapFetchValue(infoRef),
  };
};

interface StoredPage {
  id: string;
  title: string;
  body: string;
  files: Array<{
    id: string;
    name: string;
    media_type: string;
  }>;
}

const pagesRef = ref<FetchResult<Array<Page>>>({ status: "pending" });

const useRemotePages: DataSource<Readonly<Ref<Array<DeepReadonly<Page>>>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Array<Page>, Array<StoredPage>>({
    key: "pages",
    instance: toRef(envId),
    fetchPolicy,
    result: pagesRef,
    fetcher: () => api.getPages(toValue(envId), getItem<Array<StoredPage>>("pages")?.etag),
    toCache: (data) =>
      data.map((page) => ({
        id: page.id,
        title: page.title,
        body: page.body,
        files: page.files.map((file) => ({
          id: file.id,
          name: file.name,
          media_type: file.mediaType,
        })),
      })),
    fromCache: (data) =>
      data.map((page) => ({
        id: page.id,
        title: page.title,
        body: page.body,
        files: page.files.map((file) => ({
          id: file.id,
          name: file.name,
          mediaType: file.media_type,
        })),
      })),
  });

  return {
    reload,
    invalidate,
    status: unwrapFetchStatus(pagesRef),
    data: unwrapFetchArray(pagesRef),
  };
};

interface StoredAnnouncement {
  id: string;
  title: string;
  body: string;
  attachments: Array<{
    id: string;
    name: string;
    media_type: string;
  }>;
  created_at: string;
  updated_at: string | null;
}

const announcementsRef = ref<FetchResult<Array<Announcement>>>({ status: "pending" });

const useRemoteAnnouncements: DataSource<Readonly<Ref<Array<DeepReadonly<Announcement>>>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Array<Announcement>, Array<StoredAnnouncement>>(
    {
      key: "announcements",
      instance: toRef(envId),
      fetchPolicy,
      result: announcementsRef,
      fetcher: () =>
        api.getAnnouncements(
          toValue(envId),
          getItem<Array<StoredAnnouncement>>("announcements")?.etag,
        ),
      toCache: (data) =>
        data.map((announcement) => ({
          id: announcement.id,
          title: announcement.title,
          body: announcement.body,
          attachments: announcement.attachments.map((attachment) => ({
            id: attachment.id,
            name: attachment.name,
            media_type: attachment.mediaType,
          })),
          created_at: announcement.createdAt.toISOString(),
          updated_at: announcement.updatedAt?.toISOString() ?? null,
        })),
      fromCache: (data) =>
        data.map((announcement) => ({
          id: announcement.id,
          title: announcement.title,
          body: announcement.body,
          attachments: announcement.attachments.map((attachment) => ({
            id: attachment.id,
            name: attachment.name,
            mediaType: attachment.media_type,
          })),
          createdAt: new Date(announcement.created_at),
          updatedAt: announcement.updated_at ? new Date(announcement.updated_at) : undefined,
        })),
    },
  );

  return {
    reload,
    invalidate,
    status: unwrapFetchStatus(announcementsRef),
    data: unwrapFetchArray(announcementsRef),
  };
};

interface StoredConfig {
  timezone?: string;
  day_cutoff_time?: string;
  local_cache_max_age?: number;
  hide_announcements?: boolean;
  use_feedback?: boolean;
  feedback_icon?: string;
  feedback_title?: string;
  feedback_detail?: string;
  feedback_url?: string;
  use_schedule_sharing?: boolean;
  use_calendar_export?: boolean;
  use_schedule_sync?: boolean;
  use_push_notifications?: boolean;
}

const configRef = ref<FetchResult<Config>>({ status: "pending" });

const useRemoteConfig: DataSource<Readonly<Ref<Config | undefined>>> = (
  envId: MaybeRefOrGetter<string>,
  fetchPolicy: FetchPolicy,
) => {
  const { reload, invalidate } = useRemoteDataInner<Config, StoredConfig>({
    key: "config",
    instance: toRef(envId),
    fetchPolicy,
    result: configRef,
    fetcher: () => api.getConfig(toValue(envId)),
    toCache: (data) => ({
      timezone: data.timezone,
      day_cutoff_time: data.dayCutoffTime,
      local_cache_max_age: data.localCacheMaxAge,
      hide_announcements: data.hideAnnouncements,
      use_feedback: data.useFeedback,
      feedback_icon: data.feedbackIcon,
      feedback_title: data.feedbackTitle,
      feedback_detail: data.feedbackDetail,
      feedback_url: data.feedbackUrl,
      use_schedule_sharing: data.useScheduleSharing,
      use_calendar_export: data.useCalendarExport,
      use_schedule_sync: data.useScheduleSync,
      use_push_notifications: data.usePushNotifications,
    }),
    fromCache: (data) => ({
      timezone: data.timezone,
      dayCutoffTime: data.day_cutoff_time,
      localCacheMaxAge: data.local_cache_max_age,
      hideAnnouncements: data.hide_announcements,
      useFeedback: data.use_feedback,
      feedbackIcon: data.feedback_icon,
      feedbackTitle: data.feedback_title,
      feedbackDetail: data.feedback_detail,
      feedbackUrl: data.feedback_url,
      useScheduleSharing: data.use_schedule_sharing,
      useCalendarExport: data.use_calendar_export,
      useScheduleSync: data.use_schedule_sync,
      usePushNotifications: data.use_push_notifications,
    }),
  });

  return {
    reload,
    invalidate,
    data: unwrapFetchValue(configRef),
    status: unwrapFetchStatus(configRef),
  };
};

const dataSources = {
  events: useRemoteEvents,
  people: useRemotePeople,
  info: useRemoteInfo,
  pages: useRemotePages,
  announcements: useRemoteAnnouncements,
  config: useRemoteConfig,
} as const;

// When each API endpoint is refetched from the server—either on every route,
// or only on specific ones.
//
// The "global" routes provide information that's necessary for every page in
// the app:
//
// - `info` contains the app title that appears in the header.
// - `announcements` is necessary to keep the unread announcements count up to
//   date.
// - `config` is necessary for gating certain features.
const FETCH_POLICIES: Record<keyof typeof dataSources, FetchPolicy> = {
  events: ["schedule", "event"],
  people: ["event"],
  info: "global",
  pages: ["info", "page"],
  announcements: "global",
  config: "global",
};

// A default for how stale a cached endpoint may get before we refetch it, when
// the environment hasn't set `local_cache_max_age` itself.
const DEFAULT_LOCAL_CACHE_MAX_AGE_MS = 5 * 60 * 1000;

type CombinedDataSource = () => {
  data: {
    [K in keyof typeof dataSources]: ReturnType<(typeof dataSources)[K]>["data"];
  };
  status: {
    [K in keyof typeof dataSources]: ReturnType<(typeof dataSources)[K]>["status"];
  };
  reload: {
    [K in keyof typeof dataSources]: ReturnType<(typeof dataSources)[K]>["reload"];
  };
  reloadAll: () => Promise<void>;
  invalidate: () => void;
};

// We fetch all the data for the current route from the server eagerly on
// component mount and when `reload()` is called. This allows the app works
// offline.
const useRemoteData: CombinedDataSource = () => {
  const route = useRoute();
  const envId = useEnvId();

  const routeName = () => route.name as string | undefined;

  const dataSourceResponses = Object.fromEntries(
    Object.entries(dataSources).map(([key, ds]) => [
      key,
      ds(envId, FETCH_POLICIES[key as keyof typeof dataSources]),
    ]),
  );

  const scheduleSync = useScheduleSync();

  // Refresh only from the API endpoints the current route uses. This is used
  // by the manual refresh button. When syncing is enabled, we also pull the
  // latest schedule from the server alongside the rest of the data.
  const reloadAll = async () => {
    await Promise.all([
      ...Object.entries(dataSourceResponses)
        .filter(([key]) =>
          matchesRoute(FETCH_POLICIES[key as keyof typeof dataSources], routeName()),
        )
        .map(([, ds]) => ds.reload()),
      scheduleSync.pullSchedule(),
    ]);
  };

  // Mark all cached data stale, without discarding any of it.
  const invalidate = () => {
    for (const ds of Object.values(dataSourceResponses)) {
      ds.invalidate();
    }
  };

  // Typically, the client only fetches data relevant to the current page. For
  // the purpose of making sure the app has fresh data available offline, we
  // should periodically fetch all the data, regardless of the current route.
  const refetchForLocalCache = () => {
    if (!navigator.onLine) return;

    const maxAge =
      (configRef.value.status === "success" ? configRef.value.value.localCacheMaxAge : undefined) ??
      DEFAULT_LOCAL_CACHE_MAX_AGE_MS;

    const now = Date.now();

    for (const [key, ds] of Object.entries(dataSourceResponses)) {
      // Skip fetching data relevant to the current route, to avoid hitting the
      // endpoint twice.
      if (matchesRoute(FETCH_POLICIES[key as keyof typeof dataSources], routeName())) continue;

      const stored = getItem<unknown>(key);

      const isFresh =
        stored !== undefined &&
        stored.instance === envId.value &&
        now - (stored.fetched_at ?? 0) < maxAge;

      if (!isFresh) {
        void ds.reload();
      }
    }
  };

  onMounted(() => {
    // Refetching data for the purpose of updating the local cache is lower
    // priority than fetching data for the current route.
    void nextTick(() => {
      onIdle(refetchForLocalCache);
    });
  });

  return {
    reloadAll: reloadAll,
    invalidate,
    reload: Object.fromEntries(
      Object.entries(dataSourceResponses).map(([key, ds]) => [key, ds.reload]),
    ) as ReturnType<CombinedDataSource>["reload"],
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
  const remoteData = useRemoteData();
  provide(remoteDataKey, remoteData);

  // Pull the synced schedule once on app load (if syncing is enabled), mirroring how the rest of
  // the data is fetched eagerly on mount.
  void useScheduleSync().pullSchedule();

  // When the service worker receives a push for a new announcement, it
  // messages us to refetch the announcements list.
  if ("serviceWorker" in navigator) {
    navigator.serviceWorker.addEventListener("message", (event) => {
      if ((event.data as { type?: string } | undefined)?.type === "announcement") {
        void remoteData.reload.announcements();
      }
    });
  }
};

const injectRemoteData = () => {
  const data = inject<ReturnType<CombinedDataSource>>(remoteDataKey);

  if (!data) {
    throw new Error("Views must be wrapped in a <AppRoot></AppRoot>.");
  }

  return data;
};

export default injectRemoteData;
