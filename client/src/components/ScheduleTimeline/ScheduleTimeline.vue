<script setup lang="ts">
import { ref, toRef, computed, watch, watchEffect } from "vue";
import { datesToDayNames, dateIsBetween, groupByTime } from "@/utils/time";
import { useRemoteEvents } from "@/composables/useRemoteData";
import { useRoute, useRouter } from "vue-router";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { getSortedCategories } from "@/utils/tags";
import DayPicker from "./DayPicker.vue";
import SimpleIcon from "@/components/system/SimpleIcon.vue";
import ScheduleTimeSlot from "./ScheduleTimeSlot.vue";
import ScheduleHeader from "./ScheduleHeader.vue";

const route = useRoute();
const router = useRouter();
const { value: events } = useRemoteEvents();
const filterCriteria = useFilterQuery();

interface TimeSlot {
  localizedTime: string;
  events: Array<Event>;
}

interface Day {
  dayName: string;
  timeSlots: Array<TimeSlot>;
}

const currentDayIndex = defineModel<number>("day", {
  default: 0,
});

const days = ref<Array<Day>>([]);
const dayIndexByEventId = ref<Record<string, number>>({});
const searchResultEventIds = ref<Array<string>>();

const currentDayTimeSlots = computed(() =>
  days.value.length === 0 ? [] : days.value[currentDayIndex.value].timeSlots,
);

const dayNames = computed(() => days.value.map((day) => day.dayName));
const allCategories = computed(() => getSortedCategories(events.value));

const allDates = computed(() =>
  events.value.reduce((set, event) => {
    set.add(event.startTime);

    if (event.endTime) {
      set.add(event.endTime);
    }

    return set;
  }, new Set<Date>()),
);

const namedDays = computed(() => datesToDayNames(allDates.value));

watchEffect(() => {
  dayIndexByEventId.value = {};

  days.value = [...namedDays.value.entries()].map(([dayIndex, { dayName, dayStart, dayEnd }]) => {
    const eventsThisDay = events.value.filter((event) =>
      dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    const groupedEvents = groupByTime(eventsThisDay, (event) => event.startTime);

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

const currentDayStart = computed(() => namedDays.value[currentDayIndex.value]?.dayStart);
const isDayFiteringPastEvents = computed(
  () => filterCriteria.hidePastEvents && currentDayStart.value < new Date(),
);

watchEffect(async () => {
  if (route.name !== "schedule") {
    return;
  }

  await router.push({
    params: { dayIndex: currentDayIndex.value },
    query: toFilterQueryParams(filterCriteria),
  });
});

// Do not fire when the query params change. Otherwise, if the user is viewing
// an event, the schedule view will reset to that event's day each time they
// change the filters, which is disruptive.
watch(
  [toRef(route, "path"), dayIndexByEventId],
  () => {
    if (route.name === "schedule") {
      currentDayIndex.value = route.params.dayIndex
        ? parseInt(route.params.dayIndex as string, 10)
        : 0;
    } else if (route.name === "event") {
      currentDayIndex.value = route.params.eventId
        ? (dayIndexByEventId.value[route.params.eventId as string] ?? 0)
        : 0;
    }
  },
  { immediate: true },
);
</script>

<template>
  <div class="flex flex-col gap-4 h-full">
    <ScheduleHeader v-model:ids="searchResultEventIds" />
    <DayPicker v-model:day="currentDayIndex" :day-names="dayNames" />
    <span class="text-muted-color flex gap-2 justify-center" v-if="isDayFiteringPastEvents">
      <SimpleIcon class="text-lg" icon="eye-slash-fill" />
      <span class="italic">past events hidden</span>
    </span>
    <div v-if="filteredTimeSlots.length > 0" class="flex flex-col gap-6">
      <ScheduleTimeSlot
        v-for="(timeSlot, index) in filteredTimeSlots"
        :key="index"
        :localized-time="timeSlot.localizedTime"
        :events="timeSlot.events"
        :all-categories="allCategories"
      />
    </div>
    <div v-else class="text-center text-lg italic text-slate-500 dark:text-zinc-400 mt-8">
      No events
    </div>
  </div>
</template>
