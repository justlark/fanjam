import { ref, watchEffect, type Ref } from "vue";

import useRemoteData from "@/composables/useRemoteData";

export interface DatetimeFormats {
  timezone: string;
  shortTime: Intl.DateTimeFormat;
  shortDate: Intl.DateTimeFormat;
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
      shortTime: new Intl.DateTimeFormat(undefined, {
        timeStyle: "short",
        timeZone: timezone,
      }),
      shortDate: new Intl.DateTimeFormat(undefined, {
        dateStyle: "short",
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
