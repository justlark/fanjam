import { ref, watchEffect, type Ref } from "vue";

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

const datetimeFormats = ref<DatetimeFormats>();

const useDatetimeFormats = (): Readonly<Ref<DatetimeFormats | undefined>> => {
  const {
    data: { config },
  } = useRemoteData();

  watchEffect(() => {
    const timezone = config.value?.timezone ?? Intl.DateTimeFormat().resolvedOptions().timeZone;

    datetimeFormats.value = {
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

  return datetimeFormats;
};

export default useDatetimeFormats;
