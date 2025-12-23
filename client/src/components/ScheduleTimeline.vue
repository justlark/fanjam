<script setup lang="ts">
import {
  ref,
  type DeepReadonly,
  onMounted,
  onUnmounted,
  toRef,
  computed,
  watch,
  watchEffect,
} from "vue";
import { datesToDayNames, dateIsBetween, groupByTime, isSameDay } from "@/utils/time";
import useRemoteData from "@/composables/useRemoteData";
import useIncremental from "@/composables/useIncremental";
import { useRoute, useRouter } from "vue-router";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import { type Event } from "@/utils/api";
import { getSortedCategories } from "@/utils/tags";
import DayPicker from "./DayPicker.vue";
import SimpleIcon from "./SimpleIcon.vue";
import ScheduleTimeSlot from "./ScheduleTimeSlot.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import ProgressSpinner from "primevue/progressspinner";
import EventSummaryDrawer from "./EventSummaryDrawer.vue";

const route = useRoute();
const router = useRouter();
const {
  data: { events },
  status: { events: eventsStatus },
} = useRemoteData();
const datetimeFormats = useDatetimeFormats();
const filterCriteria = useFilterQuery();

const focusedEventId = defineModel<string | undefined>("focused");
const focusedEvent = computed(() =>
  focusedEventId.value
    ? events.value.find((event) => event.id === focusedEventId.value)
    : undefined,
);

onMounted(() => {
  if (history.state.focusedEventId !== undefined) {
    focusedEventId.value = history.state.focusedEventId;
  }
});

const eventSummaryIsVisible = ref(false);

watch(focusedEventId, (newEventId, oldEventId) => {
  if (oldEventId === undefined && newEventId !== undefined) {
    eventSummaryIsVisible.value = true;
  }
});

watch(eventSummaryIsVisible, (newIsVisible, oldIsVisible) => {
  if (oldIsVisible && !newIsVisible) {
    focusedEventId.value = undefined;
  }
});

interface TimeSlot {
  localizedTime: string;
  events: Array<DeepReadonly<Event>>;
}

interface Day {
  dayName: string;
  timeSlots: Array<TimeSlot>;
}

const currentDayIndex = defineModel<number>("day");

const days = ref<Array<Day>>([]);
const dayIndexByEventId = ref<Record<string, number>>({});
const searchResultEventIds = ref<Array<string>>();

const currentDayTimeSlots = computed(() => {
  if (currentDayIndex.value === undefined) {
    return [];
  }

  return days.value[currentDayIndex.value]?.timeSlots ?? [];
});

const dayNames = computed(() => days.value.map((day) => day.dayName));
const allCategories = computed(() => getSortedCategories(events.value));

const allDates = computed(() =>
  events.value.reduce((set, event) => {
    set.add(event.startTime);
    return set;
  }, new Set<Date>()),
);

const namedDays = computed(() =>
  datetimeFormats.value === undefined
    ? undefined
    : datesToDayNames(datetimeFormats.value, allDates.value),
);

const todayIndex = computed(() => {
  if (namedDays.value === undefined) return undefined;

  const today = new Date();
  const index = namedDays.value.findIndex(({ dayStart }) => isSameDay(dayStart, today));

  if (index === -1) {
    // There are no events today.
    return undefined;
  }

  return index;
});

watchEffect(() => {
  dayIndexByEventId.value = {};

  if (datetimeFormats.value === undefined || namedDays.value === undefined) return;

  // Until all events have loaded, continue using the previous `days`.
  // Otherwise, calculations for things like whether we should enable the Next
  // Day button would have to wait until all the days have loaded.
  if (eventsStatus.value !== "success") return;

  // The type narrowing won't carry into the closure body.
  const datetimeFormatsValue = datetimeFormats.value;

  days.value = [...namedDays.value.entries()].map(([dayIndex, { dayName, dayStart, dayEnd }]) => {
    const eventsThisDay = events.value.filter((event) =>
      dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    const groupedEvents = groupByTime(
      datetimeFormatsValue,
      eventsThisDay,
      (event) => event.startTime,
    );

    for (const event of eventsThisDay) {
      dayIndexByEventId.value[event.id] = dayIndex;
    }

    return {
      dayName,
      timeSlots: [...groupedEvents.entries()].map(([localizedTime, eventsInThisTimeSlot]) => ({
        localizedTime,
        events: eventsInThisTimeSlot,
      })),
    };
  });
});

const filteredEventIdsSet = computed(() =>
  searchResultEventIds.value !== undefined ? new Set(searchResultEventIds.value) : undefined,
);

const filteredTimeSlots = computed(() =>
  currentDayTimeSlots.value
    .map((timeSlot) => ({
      events: timeSlot.events.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true),
      localizedTime: timeSlot.localizedTime,
    }))
    .filter((timeSlot) => timeSlot.events.length > 0),
);

const incrementalFilteredTimeSlots = useIncremental(filteredTimeSlots);

const firstEventEndTime = computed(() => currentDayTimeSlots.value[0]?.events[0]?.endTime);

const isDayFilteringPastEvents = computed(() => {
  if (firstEventEndTime.value === undefined) {
    return false;
  }

  return filterCriteria.hidePastEvents && firstEventEndTime.value < new Date();
});

const getCurrentTimeSlotIndices = () =>
  [...filteredTimeSlots.value.entries()]
    .filter(([, timeSlot]) => {
      const firstEventStartTime = timeSlot.events[0].startTime;
      const lastEvent = timeSlot.events[timeSlot.events.length - 1];
      const lastEventEndTime = lastEvent.endTime ?? lastEvent.startTime;
      const now = new Date();

      return dateIsBetween(now, firstEventStartTime, lastEventEndTime);
    })
    .map(([index]) => index);

const currentTimeSlotIndex = ref<number>();

watchEffect(() => {
  const currentIndices = getCurrentTimeSlotIndices();
  if (currentIndices.length === 0) return;
  currentTimeSlotIndex.value = currentIndices[currentIndices.length - 1];
});

const REFRESH_NOW_TIME_INTERVAL_MILLIS = 1000 * 60 * 1;

const refreshNowTimeIntervalId = ref<number>();

onMounted(() => {
  refreshNowTimeIntervalId.value = setInterval(() => {
    const currentIndices = getCurrentTimeSlotIndices();
    if (currentIndices.length === 0) return;
    currentTimeSlotIndex.value = currentIndices[currentIndices.length - 1];
  }, REFRESH_NOW_TIME_INTERVAL_MILLIS);
});

onUnmounted(() => {
  if (refreshNowTimeIntervalId.value !== undefined) {
    clearInterval(refreshNowTimeIntervalId.value);
  }
});

// Do not fire when the query params change. Otherwise, if the user is viewing
// an event, the schedule view will reset to that event's day each time they
// change the filters, which is disruptive.
watch(
  [toRef(route, "path"), dayIndexByEventId, eventsStatus, todayIndex],
  () => {
    if (route.name === "schedule") {
      if (route.params.dayIndex) {
        if (eventsStatus.value !== "success") {
          // We cannot validate the page number until we know the number of
          // days in the schedule.
          return;
        }

        // Handle the page number in the path being out of range or not a number.
        const parsed = parseInt(route.params.dayIndex as string, 10);
        currentDayIndex.value =
          isNaN(parsed) || parsed < 0 || parsed >= days.value.length + 1 ? undefined : parsed - 1;
      } else if (currentDayIndex.value === undefined && todayIndex.value !== undefined) {
        currentDayIndex.value = todayIndex.value;
      }
    } else if (route.name === "event") {
      currentDayIndex.value = route.params.eventId
        ? dayIndexByEventId.value[route.params.eventId as string]
        : undefined;
    }
  },
  { immediate: true },
);

watchEffect(async () => {
  if (route.name !== "schedule" || currentDayIndex.value === undefined) {
    return;
  }

  // The day number we show in the URL 1-based, whereas our internal index is
  // 0-based.
  await router.push({
    params: { dayIndex: currentDayIndex.value + 1 },
    query: toFilterQueryParams(filterCriteria),
  });
});

watchEffect(() => {
  // This day does not exist.
  if (
    route.name === "schedule" &&
    eventsStatus.value === "success" &&
    currentDayTimeSlots.value.length === 0
  ) {
    currentDayIndex.value = 0;
  }
});
</script>

<template>
  <div class="flex flex-col gap-4 h-full">
    <ScheduleHeader v-model:ids="searchResultEventIds" />
    <DayPicker
      v-if="currentDayIndex !== undefined && days.length > 0"
      v-model:day="currentDayIndex"
      :day-names="dayNames"
      :today-index="todayIndex"
    />
    <span
      class="text-muted-color flex gap-2 justify-center"
      v-if="isDayFilteringPastEvents"
      data-testid="schedule-past-events-hidden-notice"
    >
      <SimpleIcon class="text-lg" icon="eye-slash-fill" />
      <span class="italic">past events hidden</span>
    </span>
    <div
      v-if="filteredTimeSlots.length > 0"
      :class="{ flex: true, 'flex-col': true, 'gap-6': true, 'mb-[15rem]': eventSummaryIsVisible }"
    >
      <ScheduleTimeSlot
        v-for="(timeSlot, index) in incrementalFilteredTimeSlots"
        v-model:focused="focusedEventId"
        :key="index"
        :localized-time="timeSlot.localizedTime"
        :events="timeSlot.events"
        :all-categories="allCategories"
        :is-current-time-slot="index === currentTimeSlotIndex"
        data-testid="schedule-time-slot"
      />
    </div>
    <div class="m-auto" v-else-if="eventsStatus === 'pending'">
      <ProgressSpinner />
    </div>
    <div v-else class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8">
      No events
    </div>
    <EventSummaryDrawer
      class="lg:!hidden"
      v-if="currentDayIndex !== undefined"
      v-model:visible="eventSummaryIsVisible"
      :event="focusedEvent"
      :day="currentDayIndex"
      :all-categories="allCategories"
    />
  </div>
</template>
