<script setup lang="ts">
import { ref, type DeepReadonly, onMounted, toRef, computed, watch, watchEffect } from "vue";
import { datesToDayNames, dateIsInRange, startOfScheduleDay, earliest } from "@/utils/time";
import useRemoteData from "@/composables/useRemoteData";
import { useRoute, useRouter } from "vue-router";
import useFilterQuery from "@/composables/useFilterQuery";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import { type Event } from "@/utils/api";
import { getSortedCategories } from "@/utils/tags";
import DayPicker from "./DayPicker.vue";
import SimpleIcon from "./SimpleIcon.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import ProgressSpinner from "primevue/progressspinner";
import EventSummaryDrawer from "./EventSummaryDrawer.vue";
import ScheduleList from "./ScheduleList.vue";

// TODO: Break up the logic in this component. This component has *way* too
// much going on.

const route = useRoute();
const router = useRouter();
const {
  data: { events },
  status: { events: eventsStatus, config: configStatus },
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

export interface DayName {
  dayName: string;
  dateName: string;
}

type Day = {
  events: Array<DeepReadonly<Event>>;
} & DayName;

const currentDayIndex = defineModel<number>("day");

const searchResultEventIds = ref<Array<string>>();
const viewType = ref<"daily" | "all">();

const allCategories = computed(() => getSortedCategories(events.value));

const allDates = computed(() => events.value.map((event) => event.startTime));

// Which days the schedule has depends on the configured timezone and day
// cutoff, so until the config has loaded we do not know how to divide the
// events up. Grouping them provisionally would be worse than waiting: the day
// we settle on gets written into the URL, and correcting it once the config
// arrives would leave the user on a day they never picked.
const namedDays = computed(() =>
  datetimeFormats.value === undefined || configStatus.value === "pending"
    ? undefined
    : datesToDayNames(datetimeFormats.value, allDates.value),
);

// The schedule cannot be paginated until we know both which events there are
// and how they divide into days.
const scheduleIsLoaded = computed(
  () => eventsStatus.value === "success" && namedDays.value !== undefined,
);

const todayIndex = computed(() => {
  const formats = datetimeFormats.value;
  if (namedDays.value === undefined || formats === undefined) return undefined;

  // Which day is "today" depends on the rollover time, so in the small hours
  // this is still the previous calendar day.
  const todayStart = startOfScheduleDay(new Date(), formats.timezone, formats.dayCutoffMinutes);
  const index = namedDays.value.findIndex(
    ({ dayStart }) => dayStart.valueOf() === todayStart.valueOf(),
  );

  if (index === -1) {
    // There are no events today.
    return undefined;
  }

  return index;
});

// Derived rather than assigned from a watcher, so that `days` is never out of
// step with `scheduleIsLoaded`. When the two could disagree, an effect reading
// the flag could see an empty `days` and conclude the selected day had no
// events.
const days = computed<Array<Day>>(() => {
  if (!scheduleIsLoaded.value || namedDays.value === undefined) {
    return [];
  }

  return namedDays.value.map(({ dayName, dateName, dayStart, dayEnd }) => ({
    dayName,
    dateName,
    events: events.value.filter((event) => dateIsInRange(event.startTime, dayStart, dayEnd)),
  }));
});

const dayIndexByEventId = computed(() => {
  const dayIndexes: Record<string, number> = {};

  for (const [dayIndex, day] of days.value.entries()) {
    for (const event of day.events) {
      dayIndexes[event.id] = dayIndex;
    }
  }

  return dayIndexes;
});

const currentDayEvents = computed(() => {
  if (currentDayIndex.value === undefined) {
    return [];
  }

  return days.value[currentDayIndex.value]?.events ?? [];
});

const filteredEventIdsSet = computed(() =>
  searchResultEventIds.value !== undefined ? new Set(searchResultEventIds.value) : undefined,
);

const filteredEventsForCurrentDay = computed(() =>
  currentDayEvents.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true),
);

const filteredEventsForAllDays = computed(() =>
  events.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true),
);

const filteredEvents = computed(() => {
  if (viewType.value === "daily") {
    return filteredEventsForCurrentDay.value;
  }

  if (viewType.value === "all") {
    return filteredEventsForAllDays.value;
  }

  return [];
});

watch(viewType, async (newViewType, oldViewType) => {
  if (oldViewType === undefined || route.name !== "schedule") {
    return;
  }

  await router.push({
    name: route.name,
    query: route.query,
    params: {
      dayIndex: newViewType === "all" ? "all" : (currentDayIndex.value ?? 0) + 1,
    },
    replace: true,
  });
});

const firstEventEndTime = computed(() =>
  earliest(...currentDayEvents.value.map((event) => event.endTime)),
);

const isDayFilteringPastEvents = computed(() => {
  if (firstEventEndTime.value === undefined) {
    return false;
  }

  return filterCriteria.hidePastEvents && firstEventEndTime.value < new Date();
});

// Do not fire when the query params change. Otherwise, if the user is viewing
// an event, the schedule view will reset to that event's day each time they
// change the filters, which is disruptive.
watch(
  [toRef(route, "path"), dayIndexByEventId, scheduleIsLoaded, todayIndex],
  () => {
    if (route.name === "schedule") {
      if (route.params.dayIndex === "all") {
        viewType.value = "all";
        currentDayIndex.value = undefined;
        return;
      }

      viewType.value = "daily";

      if (!scheduleIsLoaded.value) {
        // We do not know how many days the schedule has yet, so we can neither
        // validate a day from the path nor work out which day is today.
        return;
      }

      if (route.params.dayIndex) {
        // The day number in the path is 1-based, and may be out of range or
        // not a number at all. Fall back to the first day rather than leaving
        // no day selected.
        const requestedDayIndex = parseInt(route.params.dayIndex as string, 10) - 1;
        currentDayIndex.value =
          isNaN(requestedDayIndex) ||
          requestedDayIndex < 0 ||
          requestedDayIndex >= days.value.length
            ? 0
            : requestedDayIndex;
      } else if (currentDayIndex.value === undefined) {
        // Open on today when the schedule is running, and on the first day
        // otherwise.
        currentDayIndex.value = todayIndex.value ?? 0;
      }
    } else if (route.name === "event") {
      const currentFromViewType = history.state.fromViewType as "daily" | "all" | undefined;
      viewType.value = currentFromViewType ?? "daily";

      currentDayIndex.value = route.params.eventId
        ? dayIndexByEventId.value[route.params.eventId as string]
        : undefined;
    }
  },
  { immediate: true },
);

watchEffect(async () => {
  if (
    route.name !== "schedule" ||
    viewType.value === "all" ||
    currentDayIndex.value === undefined
  ) {
    return;
  }

  // The day number we show in the URL 1-based, whereas our internal index is
  // 0-based.
  await router.push({
    params: { dayIndex: currentDayIndex.value + 1 },
    query: route.query,
  });
});

watchEffect(() => {
  // The selected day has gone away — the schedule can shrink under us when
  // events are refetched. An unselected day is not the same thing, and is the
  // route watcher's job to fill in.
  if (
    route.name === "schedule" &&
    scheduleIsLoaded.value &&
    currentDayIndex.value !== undefined &&
    currentDayEvents.value.length === 0
  ) {
    currentDayIndex.value = 0;
  }
});
</script>

<template>
  <div>
    <div class="flex flex-col gap-4">
      <ScheduleHeader v-if="viewType" v-model:view="viewType" v-model:ids="searchResultEventIds" />
      <DayPicker
        v-if="viewType === 'daily' && currentDayIndex !== undefined && days.length > 0"
        v-model:day="currentDayIndex"
        :day-names="days"
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
      <div v-if="viewType !== undefined" :class="[{ 'mb-[15rem] lg:mb-0': eventSummaryIsVisible }]">
        <ScheduleList
          v-model:focused="focusedEventId"
          :events="filteredEvents"
          :allCategories="allCategories"
          :viewType="viewType"
        />
      </div>
      <div class="m-auto" v-else-if="eventsStatus === 'pending'">
        <ProgressSpinner />
      </div>
      <EventSummaryDrawer
        class="lg:!hidden"
        v-if="currentDayIndex !== undefined && viewType !== undefined"
        v-model:visible="eventSummaryIsVisible"
        :event="focusedEvent"
        :day="currentDayIndex"
        :all-categories="allCategories"
        :view-type="viewType"
      />
    </div>
  </div>
</template>
