interface RawEvent {
  id: string;
  name: string;
  description: string | null;
  start_time: string;
  end_time: string | null;
  location: string | null;
  people: Array<string>;
  category: string | null;
  tags: Array<string>;
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
    name: string;
    media_type: string;
    signed_url: string;
  }>;
}

interface RawPage {
  id: string;
  title: string;
  body: string;
}

interface RawConfig {
  timezone: string | null;
}

interface Envelope<T> {
  retry_after_ms?: number;
  value: T;
}

export interface Event {
  id: string;
  name: string;
  summary?: string;
  description?: string;
  startTime: Date;
  endTime?: Date;
  location?: string;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

export interface Link {
  name: string;
  url: string;
}

export interface File {
  name: string;
  mediaType: string;
  signedUrl: string;
}

export interface Page {
  id: string;
  title: string;
  body: string;
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
}

export type ApiResult<T> =
  | {
    ok: true;
    value: T;
    etag?: string;
  }
  | {
    ok: false;
    code: number;
  };

// TODO: Implement pagination instead of fetching all events at once. This
// should be fairly effective, since the user will only see the first day of
// the schedule on first page load.
//
// It's important that we still fetch all events eagerly, rather than lazily
// paginating as the user tabs through the schedule. This is necessary so the
// app works offline.
const getEvents = async (envId: string, etag?: string): Promise<ApiResult<Array<Event>>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/events`,
    {
      headers: {
        ...(etag !== undefined
          ? {
            "If-None-Match": etag,
          }
          : {}),
      },
    },
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawEvents: Envelope<{ events: Array<RawEvent> }> = await response.json();

  const events: Array<Event> = rawEvents.value.events.map((event) => ({
    id: event.id,
    name: event.name,
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
    etag: response.headers.get("ETag") ?? undefined,
  };
};

const getInfo = async (envId: string, etag?: string): Promise<ApiResult<Info>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/info`,
    {
      headers: {
        ...(etag !== undefined
          ? {
            "If-None-Match": etag,
          }
          : {}),
      },
    },
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawInfo: Envelope<RawInfo> = await response.json();

  const info: Info = {
    name: rawInfo.value.name ?? undefined,
    description: rawInfo.value.description ?? undefined,
    websiteUrl: rawInfo.value.website_url ?? undefined,
    links: rawInfo.value.links.map((link) => ({
      name: link.name,
      url: link.url,
    })),
    files: rawInfo.value.files.map((file) => ({
      name: file.name,
      mediaType: file.media_type,
      signedUrl: file.signed_url,
    })),
  };

  return {
    ok: true,
    value: info,
    etag: response.headers.get("ETag") ?? undefined,
  };
};

const getPages = async (envId: string, etag?: string): Promise<ApiResult<Array<Page>>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/pages`,
    {
      headers: {
        ...(etag !== undefined
          ? {
            "If-None-Match": etag,
          }
          : {}),
      },
    },
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawPages: Envelope<{ pages: Array<RawPage> }> = await response.json();

  const pages: Array<Page> = rawPages.value.pages.map((page) => ({
    id: page.id,
    title: page.title,
    body: page.body,
  }));

  return {
    ok: true,
    value: pages,
    etag: response.headers.get("ETag") ?? undefined,
  };
};

const getConfig = async (envId: string): Promise<ApiResult<Config>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/config`,
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawConfig: RawConfig = await response.json();

  const config: Config = {
    timezone: rawConfig.timezone ?? undefined,
  };

  return { ok: true, value: config };
};

export default {
  getEvents,
  getInfo,
  getPages,
  getConfig,
};
