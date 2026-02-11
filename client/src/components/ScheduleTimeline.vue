<script setup lang="ts">
import {
  ref,
  type DeepReadonly,
  onMounted,
  toRef,
  computed,
  watch,
  watchEffect,
} from "vue";
import { datesToDayNames, dateIsBetween, isSameDay, earliest } from "@/utils/time";
import useRemoteData from "@/composables/useRemoteData";
import { useRoute, useRouter } from "vue-router";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
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

interface Day {
  dayName: string;
  events: Array<DeepReadonly<Event>>;
}

const currentDayIndex = defineModel<number>("day");

const days = ref<Array<Day>>([]);
const dayIndexByEventId = ref<Record<string, number>>({});
const searchResultEventIds = ref<Array<string>>();
const viewType = ref<"daily" | "all">();

const currentDayEvents = computed(() => {
  if (currentDayIndex.value === undefined) {
    return [];
  }

  return days.value[currentDayIndex.value]?.events ?? [];
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
  const timezone = datetimeFormats.value?.timezone;
  if (namedDays.value === undefined || timezone === undefined) return undefined;

  const today = new Date();
  const index = namedDays.value.findIndex(({ dayStart }) => isSameDay(dayStart, today, timezone));

  if (index === -1) {
    // There are no events today.
    return undefined;
  }

  return index;
});

watchEffect(() => {
  dayIndexByEventId.value = {};

  if (namedDays.value === undefined) return;

  // Until all events have loaded, continue using the previous `days`.
  // Otherwise, calculations for things like whether we should enable the Next
  // Day button would have to wait until all the days have loaded.
  if (eventsStatus.value !== "success") return;

  days.value = [...namedDays.value.entries()].map(([dayIndex, { dayName, dayStart, dayEnd }]) => {
    const eventsThisDay = events.value.filter((event) =>
      dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    for (const event of eventsThisDay) {
      dayIndexByEventId.value[event.id] = dayIndex;
    }

    return {
      dayName,
      events: eventsThisDay,
    };
  });
});

const filteredEventIdsSet = computed(() =>
  searchResultEventIds.value !== undefined ? new Set(searchResultEventIds.value) : undefined,
);

const filteredEventsForCurrentDay = computed(() =>
  currentDayEvents.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true)
);

const filteredEventsForAllDays = computed(() =>
  events.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true)
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

const firstEventEndTime = computed(() => earliest(...currentDayEvents.value.map(event => event.endTime)));

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
  [toRef(route, "path"), dayIndexByEventId, eventsStatus, todayIndex],
  () => {
    if (route.name === "schedule") {
      if (route.params.dayIndex === "all") {
        viewType.value = "all";
        currentDayIndex.value = undefined;
        return;
      }

      viewType.value = "daily";

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
    query: toFilterQueryParams(filterCriteria),
  });
});

watchEffect(() => {
  // This day does not exist.
  if (
    route.name === "schedule" &&
    eventsStatus.value === "success" &&
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
        v-if="viewType !== undefined"
        :class="[{ 'mb-[15rem] lg:mb-0': eventSummaryIsVisible }]"
      >
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
      <div
        v-else
        class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8"
        data-testid="schedule-no-events"
      >
        No events
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
