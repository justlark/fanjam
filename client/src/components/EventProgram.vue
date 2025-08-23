<script setup lang="ts">
import { computed, watchEffect, ref } from "vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useRemoteData from "@/composables/useRemoteData";
import ProgressSpinner from "primevue/progressspinner";
import EventProgramDay from "./EventProgramDay.vue";
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
const datetimeFormats = useDatetimeFormats();

const allCategories = computed(() =>
  events.value.reduce<Array<string>>((set, event) => {
    if (event.category && !set.includes(event.category)) {
      set.push(event.category);
    }

    return set;
  }, []),
);

const allDates = computed(() =>
  events.value.reduce((set, event) => {
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

watchEffect(() => {
  if (datetimeFormats.value === undefined || namedDays.value === undefined) return;

  days.value = namedDays.value.map(({ dayName, dayStart, dayEnd }) => {
    const eventsThisDay = events.value.filter((event) =>
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
