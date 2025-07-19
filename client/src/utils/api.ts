interface RawEvent {
  id: string;
  name: string;
  description: string;
  start_time: string;
  end_time: string;
  location: string;
  people: Array<string>;
  category: string;
  tags: Array<string>;
}

export interface Event {
  id: string;
  name: string;
  description: string;
  startTime: Date;
  endTime: Date;
  location: string;
  people: Array<string>;
  category: string;
  tags: Array<string>;
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

const getEvents = async (envId: string): Promise<ApiResult<Array<Event>>> => {
  const response = await fetch(`https://${import.meta.env.VITE_API_HOST}/events/${envId}`);

  if (!response.ok) {
    return { ok: false, status: response.status };
  }

  const rawEvents: Array<RawEvent> = await response.json();

  const events: Array<Event> = rawEvents.map((event) => ({
    id: event.id,
    name: event.name,
    description: event.description,
    startTime: new Date(event.start_time),
    endTime: new Date(event.end_time),
    location: event.location,
    people: event.people,
    category: event.category,
    tags: event.tags,
  }));

  return { ok: true, value: events };
};

export default {
  getEvents,
};
