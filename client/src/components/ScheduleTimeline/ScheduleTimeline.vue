<script setup lang="ts">
import { ref, toRef, computed, watch, watchEffect } from "vue";
import { datesToDayNames, dateIsBetween, groupByTime } from "@/utils/time";
import { useRoute, useRouter } from "vue-router";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { getSortedCategories } from "@/utils/tags";
import DayPicker from "./DayPicker.vue";
import ScheduleTimeSlot from "./ScheduleTimeSlot.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import ProgressSpinner from "primevue/progressspinner";

const route = useRoute();
const router = useRouter();
const filterCriteria = useFilterQuery();

interface TimeSlot {
  localizedTime: string;
  events: Array<Event>;
}

interface Day {
  dayName: string;
  timeSlots: Array<TimeSlot>;
}

const props = defineProps<{
  events: Array<Event>;
}>();

const currentDayIndex = defineModel<number>("day", {
  default: 0,
});

const days = ref<Array<Day>>([]);
const dayIndexByEventId = ref<Record<string, number>>({});
const searchResultEventIds = ref<Array<string>>();

const dayNames = computed(() => days.value.map((day) => day.dayName));
const allCategories = computed(() => getSortedCategories(props.events));

watchEffect(() => {
  dayIndexByEventId.value = {};

  const allDates = props.events.reduce((set, event) => {
    set.add(event.startTime);

    if (event.endTime) {
      set.add(event.endTime);
    }

    return set;
  }, new Set<Date>());

  const namedDays = datesToDayNames(allDates);

  days.value = [...namedDays.entries()].map(([dayIndex, { dayName, dayStart, dayEnd }]) => {
    const eventsThisDay = props.events.filter((event) =>
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

const eventIdAllowSet = computed(() =>
  searchResultEventIds.value !== undefined ? new Set(searchResultEventIds.value) : undefined,
);

const filteredTimeSlots = computed(() =>
  days.value.length === 0
    ? []
    : days.value[currentDayIndex.value].timeSlots
        .map((timeSlot) => ({
          events: timeSlot.events.filter((event) => eventIdAllowSet.value?.has(event.id) ?? true),
          localizedTime: timeSlot.localizedTime,
        }))
        .filter((timeSlot) => timeSlot.events.length > 0),
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
    <ScheduleHeader v-model:ids="searchResultEventIds" :events="props.events" />
    <DayPicker v-if="days.length > 0" v-model:day="currentDayIndex" :day-names="dayNames" />
    <div v-if="days.length === 0" class="flex justify-center items-center h-full">
      <div class="flex flex-col items-center gap-4">
        <ProgressSpinner />
        <div>Loading the schedule…</div>
      </div>
    </div>
    <div v-else-if="filteredTimeSlots.length > 0" class="flex flex-col gap-8">
      <ScheduleTimeSlot
        v-for="(timeSlot, index) in filteredTimeSlots"
        :key="index"
        :localized-time="timeSlot.localizedTime"
        :events="timeSlot.events"
        :all-categories="allCategories"
      />
    </div>
    <div
      v-else
      class="text-center italic text-slate-500 dark:text-zinc-400 mt-8 flex flex-col gap-2"
    >
      <span>No events for this day</span>
      <span>(or they've been filtered out)</span>
    </div>
  </div>
</template>
