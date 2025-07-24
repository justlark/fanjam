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

export interface RawAbout {
  name: string;
  description: string | null;
  link: string | null;
}

export interface RawDump {
  events: Array<RawEvent>;
  about: RawAbout | null;
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
  link?: string;
}

export interface Dump {
  events: Array<Event>;
  about?: About;
}

export type ApiResult<T> =
  | {
      ok: true;
      value: T;
    }
  | {
      ok: false;
      status: number;
    };

const getDump = async (envId: string): Promise<ApiResult<Dump>> => {
  const response = await fetch(
    `https://${import.meta.env.VITE_API_HOST as string}/events/${envId}`,
  );

  if (!response.ok) {
    return { ok: false, status: response.status };
  }

  const rawDump: RawDump = await response.json();

  const dump: Dump = {
    events: rawDump.events.map((event) => ({
      id: event.id,
      name: event.name,
      description: event.description ?? undefined,
      startTime: new Date(event.start_time),
      endTime: event.end_time ? new Date(event.end_time) : undefined,
      location: event.location ?? undefined,
      people: event.people,
      category: event.category ?? undefined,
      tags: event.tags,
    })),
    about: rawDump.about
      ? {
          name: rawDump.about.name,
          description: rawDump.about.description ?? undefined,
          link: rawDump.about.link ?? undefined,
        }
      : undefined,
  };

  return { ok: true, value: dump };
};

export default {
  getDump,
};
