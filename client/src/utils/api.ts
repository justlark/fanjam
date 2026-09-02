interface RawEvent {
  id: string;
  name: string;
  summary: string | null;
  description: string | null;
  start_time: string;
  end_time: string | null;
  location: {
    id: string;
    name: string;
  } | null;
  people: Array<{
    id: string;
    name: string;
  }>;
  category: {
    id: string;
    name: string;
  } | null;
  tags: Array<{
    id: string;
    name: string;
  }>;
}

interface RawPerson {
  id: string;
  name: string;
  bio: string | null;
}

interface RawInfo {
  name: string | null;
  description: string | null;
  website_url: string | null;
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

interface RawPage {
  id: string;
  title: string;
  body: string;
  files: Array<{
    id: string;
    name: string;
    media_type: string;
  }>;
}

interface RawAnnouncement {
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

interface RawAlias {
  env_id: string;
}

interface RawConfig {
  timezone: string | null;
  day_cutoff_time: string | null;
  local_cache_max_age: number | null;
  hide_announcements: boolean | null;
  use_feedback: boolean | null;
  feedback_icon: string | null;
  feedback_title: string | null;
  feedback_detail: string | null;
  feedback_url: string | null;
  use_schedule_sharing: boolean | null;
  use_calendar_export: boolean | null;
  use_schedule_sync: boolean | null;
  use_push_notifications: boolean | null;
}

// How current the data the server sent us is, and what we should do about it.
//
// The server classifies the upstream condition; we decide our own retry
// schedule from it. `"backoff"` means NocoDB has been failing and refreshes are
// in cooldown, so retrying would only add to the load keeping it down — there
// is no fresher data coming until that cooldown expires.
//
// Absent on responses from a server older than this client, in which case we
// treat the data as fresh and simply don't auto-refresh.
export type Freshness = "fresh" | "stale" | "backoff";

// The API schema version this build of the client understands. The server
// sends this schema version with every API response.
//
// Because new builds of the server affect everyone immediately, but new builds
// of the client only affect clients when they refresh the page and register
// the new service worker, we need some way of tracking the discrepancy.
const SCHEMA_VERSION = 2;

interface Envelope<T> {
  // Absent on responses from a server predating this field, which are
  // implicitly version 1.
  schema_version?: number;
  freshness?: Freshness;
  value: T;
}

const isReadable = (envelope: Envelope<unknown>): boolean =>
  (envelope.schema_version ?? 1) === SCHEMA_VERSION;

export interface EventPerson {
  id: string;
  name: string;
}

export interface EventLocation {
  id: string;
  name: string;
}

export interface EventCategory {
  id: string;
  name: string;
}

export interface EventTag {
  id: string;
  name: string;
}

export interface Person {
  id: string;
  name: string;
  bio?: string;
}

export interface Event {
  id: string;
  name: string;
  summary?: string;
  description?: string;
  startTime: Date;
  endTime?: Date;
  location?: EventLocation;
  people: Array<EventPerson>;
  category?: EventCategory;
  tags: Array<EventTag>;
}

export interface Link {
  name: string;
  url: string;
}

export interface File {
  id: string;
  name: string;
  mediaType: string;
}

export interface Page {
  id: string;
  title: string;
  body: string;
  files: Array<File>;
}

export interface Announcement {
  id: string;
  title: string;
  body: string;
  attachments: Array<File>;
  createdAt: Date;
  updatedAt?: Date;
}

export interface File {
  id: string;
  name: string;
  mediaType: string;
}

export interface Info {
  name?: string;
  description?: string;
  websiteUrl?: string;
  links: Array<Link>;
  files: Array<File>;
}

export interface Config {
  timezone?: string;
  dayCutoffTime?: string;
  localCacheMaxAge?: number;
  hideAnnouncements?: boolean;
  useFeedback?: boolean;
  feedbackIcon?: string;
  feedbackTitle?: string;
  feedbackDetail?: string;
  feedbackUrl?: string;
  useScheduleSharing?: boolean;
  useCalendarExport?: boolean;
  useScheduleSync?: boolean;
  usePushNotifications?: boolean;
}

const isOk = (response: Response): boolean => response.ok;

// `fetch` rejects rather than resolving with `ok: false` when the network is
// unreachable, which is the normal offline case. Swallow the rejection here so
// callers can distinguish the offline case from the error case.
const safeFetch = async (input: string, init?: RequestInit): Promise<Response | undefined> => {
  try {
    return await fetch(input, init);
  } catch {
    return undefined;
  }
};

// The server answers conditional requests with a 304 Not Modified, but we
// deliberately don't pass `If-None-Match` ourselves. Because the server sends
// `Cache-Control: no-cache`, the browser will reply with `If-None-Match`
// implicitly on its own. This is important, because sending `If-None-Match`
// ourselves would force every one of those GET requests to be preflighted,
// ignoring the CORS cache and the `Access-Control-Max-Age` the server sends.
// This is not a problem when we rely on the browser to do the conditional
// request for us.

export type ApiResult<T> =
  | {
      ok: true;
      value: T;
      freshness?: Freshness;
    }
  | {
      ok: false;
      // `outdated` means the API schema version sent by the server and the API
      // schema version understood by this build of the client disagree.
      code: number | "offline" | "outdated";
    };

// TODO: Implement pagination instead of fetching all events at once. This
// should be fairly effective, since the user will only see the first day of
// the schedule on first page load.
//
// It's important that we still fetch all events eagerly, rather than lazily
// paginating as the user tabs through the schedule. This is necessary so the
// app works offline.
const getEvents = async (envId: string): Promise<ApiResult<Array<Event>>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/events`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawEvents: Envelope<{ events: Array<RawEvent> }> = await response.json();

  if (!isReadable(rawEvents)) {
    return { ok: false, code: "outdated" };
  }

  const events: Array<Event> = rawEvents.value.events.map((event) => ({
    id: event.id,
    name: event.name,
    summary: event.summary ?? undefined,
    description: event.description ?? undefined,
    startTime: new Date(event.start_time),
    endTime: event.end_time ? new Date(event.end_time) : undefined,
    location: event.location ?? undefined,
    people: event.people,
    category: event.category ?? undefined,
    tags: event.tags,
  }));

  return {
    ok: true,
    value: events,
    freshness: rawEvents.freshness,
  };
};

const getPeople = async (envId: string): Promise<ApiResult<Array<Person>>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/people`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawPeople: Envelope<{ people: Array<RawPerson> }> = await response.json();

  if (!isReadable(rawPeople)) {
    return { ok: false, code: "outdated" };
  }

  const people: Array<Person> = rawPeople.value.people.map((person) => ({
    id: person.id,
    name: person.name,
    bio: person.bio ?? undefined,
  }));

  return {
    ok: true,
    value: people,
    freshness: rawPeople.freshness,
  };
};

const getInfo = async (envId: string): Promise<ApiResult<Info>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/info`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawInfo: Envelope<RawInfo> = await response.json();

  if (!isReadable(rawInfo)) {
    return { ok: false, code: "outdated" };
  }

  const info: Info = {
    name: rawInfo.value.name ?? undefined,
    description: rawInfo.value.description ?? undefined,
    websiteUrl: rawInfo.value.website_url ?? undefined,
    links: rawInfo.value.links.map((link) => ({
      name: link.name,
      url: link.url,
    })),
    files: rawInfo.value.files.map((file) => ({
      id: file.id,
      name: file.name,
      mediaType: file.media_type,
    })),
  };

  return {
    ok: true,
    value: info,
    freshness: rawInfo.freshness,
  };
};

const getPages = async (envId: string): Promise<ApiResult<Array<Page>>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/pages`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawPages: Envelope<{ pages: Array<RawPage> }> = await response.json();

  if (!isReadable(rawPages)) {
    return { ok: false, code: "outdated" };
  }

  const pages: Array<Page> = rawPages.value.pages.map((page) => ({
    id: page.id,
    title: page.title,
    body: page.body,
    files: page.files.map((file) => ({
      id: file.id,
      name: file.name,
      mediaType: file.media_type,
    })),
  }));

  return {
    ok: true,
    value: pages,
    freshness: rawPages.freshness,
  };
};

const getAnnouncements = async (envId: string): Promise<ApiResult<Array<Announcement>>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/announcements`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawAnnouncements: Envelope<{ announcements: Array<RawAnnouncement> }> =
    await response.json();

  if (!isReadable(rawAnnouncements)) {
    return { ok: false, code: "outdated" };
  }

  const announcements: Array<Announcement> = rawAnnouncements.value.announcements.map(
    (announcement) => ({
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
    }),
  );

  return {
    ok: true,
    value: announcements,
    freshness: rawAnnouncements.freshness,
  };
};

const getConfig = async (envId: string): Promise<ApiResult<Config>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/config`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawConfig: RawConfig = await response.json();

  const config: Config = {
    timezone: rawConfig.timezone ?? undefined,
    dayCutoffTime: rawConfig.day_cutoff_time ?? undefined,
    localCacheMaxAge: rawConfig.local_cache_max_age ?? undefined,
    hideAnnouncements: rawConfig.hide_announcements ?? undefined,
    useFeedback: rawConfig.use_feedback ?? undefined,
    feedbackIcon: rawConfig.feedback_icon ?? undefined,
    feedbackTitle: rawConfig.feedback_title ?? undefined,
    feedbackDetail: rawConfig.feedback_detail ?? undefined,
    feedbackUrl: rawConfig.feedback_url ?? undefined,
    useScheduleSharing: rawConfig.use_schedule_sharing ?? undefined,
    useCalendarExport: rawConfig.use_calendar_export ?? undefined,
    useScheduleSync: rawConfig.use_schedule_sync ?? undefined,
    usePushNotifications: rawConfig.use_push_notifications ?? undefined,
  };

  return { ok: true, value: config };
};

const getAlias = async (aliasId: string): Promise<ApiResult<string>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/aliases/${aliasId}`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const rawConfig: RawAlias = await response.json();

  return { ok: true, value: rawConfig.env_id };
};

const postSubscription = async (
  envId: string,
  subscription: PushSubscriptionJSON,
): Promise<ApiResult<void>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/subscription`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(subscription),
    },
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  return { ok: true, value: undefined };
};

const deleteSubscription = async (envId: string, endpoint: string): Promise<ApiResult<void>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/subscription`,
    {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ endpoint }),
    },
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  return { ok: true, value: undefined };
};

const putSchedule = async (
  envId: string,
  syncCode: string,
  schedule: Array<string>,
): Promise<ApiResult<void>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/schedule/${syncCode}`,
    {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ schedule }),
    },
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  return { ok: true, value: undefined };
};

const getSchedule = async (envId: string, syncCode: string): Promise<ApiResult<Array<string>>> => {
  const response = await safeFetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/schedule/${syncCode}`,
  );

  if (response === undefined) {
    return { ok: false, code: "offline" };
  }

  if (!isOk(response)) {
    return { ok: false, code: response.status };
  }

  const body: { schedule: Array<string> } = await response.json();

  return { ok: true, value: body.schedule };
};

export default {
  getEvents,
  getPeople,
  getInfo,
  getPages,
  getAnnouncements,
  getConfig,
  getAlias,
  postSubscription,
  deleteSubscription,
  putSchedule,
  getSchedule,
};
