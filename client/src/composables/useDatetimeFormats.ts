import { ref, watchEffect, type Ref } from "vue";

import useRemoteData from "@/composables/useRemoteData";

export interface DatetimeFormats {
  shortTime: Intl.DateTimeFormat;
  shortDate: Intl.DateTimeFormat;
  shortWeekday: Intl.DateTimeFormat;
  longWeekday: Intl.DateTimeFormat;
}

const datetimeFormats = ref<DatetimeFormats>();

const useDatetimeFormats = (): Readonly<Ref<DatetimeFormats | undefined>> => {
  const {
    data: { config },
  } = useRemoteData();

  watchEffect(() => {
    datetimeFormats.value = {
      shortTime: new Intl.DateTimeFormat(undefined, {
        timeStyle: "short",
        timeZone: config.value?.timezone,
      }),
      shortDate: new Intl.DateTimeFormat(undefined, {
        dateStyle: "short",
        timeZone: config.value?.timezone,
      }),
      shortWeekday: new Intl.DateTimeFormat(undefined, {
        weekday: "short",
        timeZone: config.value?.timezone,
      }),
      longWeekday: new Intl.DateTimeFormat(undefined, {
        weekday: "long",
        timeZone: config.value?.timezone,
      }),
    };
  });

  return datetimeFormats;
};

export default useDatetimeFormats;
