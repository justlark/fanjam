<script setup lang="ts">
import { computed, watchEffect, ref } from "vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import useFilterQuery from "@/composables/useFilterQuery";
import ProgressSpinner from "primevue/progressspinner";
import SimpleIcon from "./SimpleIcon.vue";
import EventProgramDay from "./EventProgramDay.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import { datesToDayNames, dateIsBetween } from "@/utils/time";
import { type Event } from "@/utils/api";

const {
  data: { events },
  result: { events: eventsResult },
} = useRemoteData();

interface Day {
  dayName: string;
  events: Array<Event>;
}

const days = ref<Array<Day>>([]);
const filterCriteria = useFilterQuery();
const filteredEventIds = ref<Array<string>>();
const datetimeFormats = useDatetimeFormats();

const allCategories = computed(() => getSortedCategories(events));

const allDates = computed(() =>
  events.reduce((set, event) => {
    set.add(event.startTime);

    if (event.endTime) {
      set.add(event.endTime);
    }

    return set;
  }, new Set<Date>()),
);

const namedDays = computed(() =>
  datetimeFormats.value === undefined
    ? undefined
    : datesToDayNames(datetimeFormats.value, allDates.value),
);

const filteredEventIdsSet = computed(() =>
  filteredEventIds.value !== undefined ? new Set(filteredEventIds.value) : undefined,
);

const filteredEvents = computed(() =>
  events.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true),
);

const isFilteringPastEvents = computed(() => {
  return filterCriteria.hidePastEvents && filteredEvents.value.length < events.length;
});

watchEffect(() => {
  if (datetimeFormats.value === undefined || namedDays.value === undefined) return;

  days.value = namedDays.value.map(({ dayName, dayStart, dayEnd }) => {
    const eventsThisDay = filteredEvents.value.filter((event) =>
      dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    return {
      dayName,
      events: eventsThisDay,
    };
  });
});
</script>

<template>
  <div class="flex flex-col gap-4 h-full">
    <ScheduleHeader v-model:ids="filteredEventIds" />
    <span class="text-muted-color flex gap-2 justify-center" v-if="isFilteringPastEvents">
      <SimpleIcon class="text-lg" icon="eye-slash-fill" />
      <span class="italic">past events hidden</span>
    </span>
    <div class="m-auto" v-if="eventsResult.status === 'pending'">
      <ProgressSpinner />
    </div>
    <div class="flex flex-col gap-8" v-else>
      <EventProgramDay
        v-for="(day, index) in days"
        :key="index"
        :day-name="day.dayName"
        :events="day.events"
        :day-index="index"
        :all-categories="allCategories"
      />
    </div>
  </div>
</template>
