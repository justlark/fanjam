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

export interface RawInfo {
  about?: {
    name: string;
    description: string | null;
    website_url: string | null;
  };
  links: Array<{
    name: string;
    url: string;
  }>;
}

export interface Event {
  id: string;
  name: string;
  description?: string;
  startTime: Date;
  endTime?: Date;
  location?: string;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

export interface About {
  name: string;
  description?: string;
  websiteUrl?: string;
}

export interface Link {
  name: string;
  url: string;
}

export interface Info {
  about?: About;
  links: Array<Link>;
}

export type ApiResult<T> =
  | {
    ok: true;
    value: T;
  }
  | {
    ok: false;
    code: number;
  };

const getEvents = async (envId: string): Promise<ApiResult<Array<Event>>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/events`,
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawEvents: { events: Array<RawEvent> } = await response.json();

  const events: Array<Event> = rawEvents.events.map((event) => ({
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

  return { ok: true, value: events };
};

const getInfo = async (envId: string): Promise<ApiResult<Info>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/apps/${envId}/info`,
  );

  if (!response.ok) {
    return { ok: false, code: response.status };
  }

  const rawInfo: RawInfo = await response.json();

  const info: Info = {
    about: rawInfo.about
      ? {
        name: rawInfo.about.name,
        description: rawInfo.about.description ?? undefined,
        websiteUrl: rawInfo.about.website_url ?? undefined,
      }
      : undefined,
    links: rawInfo.links.map((link) => ({
      name: link.name,
      url: link.url,
    })),
  };

  return { ok: true, value: info };
};

export default {
  getEvents,
  getInfo,
};
