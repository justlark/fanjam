import { type DeepReadonly } from "vue";
import { createEvents, type DateArray, type EventAttributes } from "ics";
import type { Event } from "@/utils/api";

// Convert a Date to the `[Y, M, D, h, m]` UTC tuple that the `ics` library
// expects when `*InputType: "utc"` is set. Months are 1-indexed for ics.
const toUtcDateArray = (date: Date): DateArray => [
  date.getUTCFullYear(),
  date.getUTCMonth() + 1,
  date.getUTCDate(),
  date.getUTCHours(),
  date.getUTCMinutes(),
];

const buildEventAttributes = (
  event: DeepReadonly<Event> & { endTime: Date },
  envId: string,
  appUrl: (segment: string) => string,
): EventAttributes => {
  const link = appUrl(`events/${event.id}`);
  const body = (event.summary ?? event.description ?? "").trimStart();
  const description = body.length > 0 ? `${body}\n\nView in app: ${link}` : `View in app: ${link}`;

  return {
    title: event.name,
    start: toUtcDateArray(event.startTime),
    startInputType: "utc",
    end: toUtcDateArray(event.endTime),
    endInputType: "utc",
    location: event.location,
    description,
    url: link,
    uid: `${event.id}@fanjam.live:${envId}`,
    productId: "fanjam/ics",
  };
};

const triggerDownload = (icsText: string, filename: string): void => {
  const blob = new Blob([icsText], { type: "text/calendar;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
};

// Build a .ics file containing one VEVENT per starred event and trigger a
// browser download. Events without an `endTime` are skipped (we don't guess
// durations). If no events remain after that filter, nothing is downloaded.
export const downloadStarredEventsIcs = (
  events: ReadonlyArray<DeepReadonly<Event>>,
  envId: string,
  appUrl: (segment: string) => string,
  filename = "my-schedule.ics",
): void => {
  const exportable = events.filter(
    (event): event is DeepReadonly<Event> & { endTime: Date } => event.endTime !== undefined,
  );
  if (exportable.length === 0) return;

  const attributes = exportable.map((event) => buildEventAttributes(event, envId, appUrl));
  const { error, value } = createEvents(attributes);
  if (error || !value) {
    console.error("Failed to generate .ics file", error);
    return;
  }

  triggerDownload(value, filename);
};
