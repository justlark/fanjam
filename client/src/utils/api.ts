interface RawEvent {
  id: string;
  name: string;
  summary: string | null;
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
  files: Array<{
    name: string;
    media_type: string;
    signed_url: string;
  }>;
}

interface RawAnnouncement {
  id: string;
  title: string;
  body: string;
  attachments: Array<{
    name: string;
    media_type: string;
    signed_url: string;
  }>;
  created_at: string;
  updated_at: string;
}

interface RawAlias {
  env_id: string;
}

interface RawConfig {
  timezone: string | null;
  hide_announcements: boolean | null;
  use_feedback: boolean | null;
  feedback_icon: string | null;
  feedback_title: string | null;
  feedback_detail: string | null;
  feedback_url: string | null;
}

interface Envelope<T> {
  stale?: boolean;
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
  files: Array<File>;
}

export interface Announcement {
  id: string;
  title: string;
  body: string;
  attachments: Array<File>;
  createdAt: Date;
  updatedAt: Date;
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
  hideAnnouncements?: boolean;
  useFeedback?: boolean;
  feedbackIcon?: string;
  feedbackTitle?: string;
  feedbackDetail?: string;
  feedbackUrl?: string;
}

export type ApiResult<T> =
  | {
      ok: true;
      value: T;
      etag?: string;
      stale?: boolean;
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
    etag: response.headers.get("ETag") ?? undefined,
    stale: rawEvents.stale,
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
    stale: rawInfo.stale,
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
    files: page.files.map((file) => ({
      name: file.name,
      mediaType: file.media_type,
      signedUrl: file.signed_url,
    })),
  }));

  return {
    ok: true,
    value: pages,
    etag: response.headers.get("ETag") ?? undefined,
    stale: rawPages.stale,
  };
};

const getAnnouncements = async (
  envId: string,
  etag?: string,
): Promise<ApiResult<Array<Announcement>>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/announcements`,
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

  const rawAnnouncements: Envelope<{ announcements: Array<RawAnnouncement> }> =
    await response.json();

  const announcements: Array<Announcement> = rawAnnouncements.value.announcements.map(
    (announcement) => ({
      id: announcement.id,
      title: announcement.title,
      body: announcement.body,
      attachments: announcement.attachments.map((attachment) => ({
        name: attachment.name,
        mediaType: attachment.media_type,
        signedUrl: attachment.signed_url,
      })),
      createdAt: new Date(announcement.created_at),
      updatedAt: new Date(announcement.updated_at),
    }),
  );

  return {
    ok: true,
    value: announcements,
    etag: response.headers.get("ETag") ?? undefined,
    stale: rawAnnouncements.stale,
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
    hideAnnouncements: rawConfig.hide_announcements ?? undefined,
    useFeedback: rawConfig.use_feedback ?? undefined,
    feedbackIcon: rawConfig.feedback_icon ?? undefined,
    feedbackTitle: rawConfig.feedback_title ?? undefined,
    feedbackDetail: rawConfig.feedback_detail ?? undefined,
    feedbackUrl: rawConfig.feedback_url ?? undefined,
  };

  return { ok: true, value: config };
};

const getAlias = async (aliasId: string): Promise<ApiResult<string>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/aliases/${aliasId}`,
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawConfig: RawAlias = await response.json();

  return { ok: true, value: rawConfig.env_id };
};

export default {
  getEvents,
  getInfo,
  getPages,
  getAnnouncements,
  getConfig,
  getAlias,
};
