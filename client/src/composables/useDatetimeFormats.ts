import { computed, type Ref } from "vue";

import useRemoteData from "@/composables/useRemoteData";
import { parseDayCutoff } from "@/utils/time";

export interface DatetimeFormats {
  timezone: string;
  // How long after midnight the schedule rolls over to the next day, in
  // minutes. Zero means days roll over at midnight.
  dayCutoffMinutes: number;
  shortTime: Intl.DateTimeFormat;
  shortDate: Intl.DateTimeFormat;
  mediumDate: Intl.DateTimeFormat;
  shortDatetime: Intl.DateTimeFormat;
  shortWeekday: Intl.DateTimeFormat;
  longWeekday: Intl.DateTimeFormat;
}

// Derived rather than assigned from a watcher, so that the formats can never
// lag the config they come from. A watcher updates them a tick late, and
// anything reading them in that window formats against the previous timezone
// and day cutoff — which is enough to land the schedule on the wrong day.
const useDatetimeFormats = (): Readonly<Ref<DatetimeFormats | undefined>> => {
  const {
    data: { config },
  } = useRemoteData();

  return computed(() => {
    const timezone = config.value?.timezone ?? Intl.DateTimeFormat().resolvedOptions().timeZone;

    return {
      timezone,
      dayCutoffMinutes: parseDayCutoff(config.value?.dayCutoffTime),
      shortTime: new Intl.DateTimeFormat(undefined, {
        timeStyle: "short",
        timeZone: timezone,
      }),
      shortDate: new Intl.DateTimeFormat(undefined, {
        dateStyle: "short",
        timeZone: timezone,
      }),
      mediumDate: new Intl.DateTimeFormat(undefined, {
        dateStyle: "medium",
        timeZone: timezone,
      }),
      shortDatetime: new Intl.DateTimeFormat(undefined, {
        timeStyle: "short",
        dateStyle: "medium",
        timeZone: timezone,
      }),
      shortWeekday: new Intl.DateTimeFormat(undefined, {
        weekday: "short",
        timeZone: timezone,
      }),
      longWeekday: new Intl.DateTimeFormat(undefined, {
        weekday: "long",
        timeZone: timezone,
      }),
    };
  });
};

export default useDatetimeFormats;
