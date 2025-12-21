<script setup lang="ts">
import { computed, type DeepReadonly, watchEffect, ref } from "vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import useFilterQuery from "@/composables/useFilterQuery";
import SimpleIcon from "./SimpleIcon.vue";
import EventProgramDay from "./EventProgramDay.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import { datesToDayNames, dateIsBetween } from "@/utils/time";
import { type Event } from "@/utils/api";

const {
  data: { events },
} = useRemoteData();

interface Day {
  dayName: string;
  events: Array<DeepReadonly<Event>>;
}

const props = defineProps<{
  focusedEventId?: string;
}>();

const days = ref<Array<Day>>([]);
const filterCriteria = useFilterQuery();
const filteredEventIds = ref<Array<string>>();
const datetimeFormats = useDatetimeFormats();

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

const namedDays = computed(() =>
  datetimeFormats.value === undefined
    ? undefined
    : datesToDayNames(datetimeFormats.value, allDates.value),
);

const filteredEventIdsSet = computed(() =>
  filteredEventIds.value !== undefined ? new Set(filteredEventIds.value) : undefined,
);

const filteredEvents = computed(() => {
  const filtered = events.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true);

  filtered.sort((a, b) =>
    a.endTime === undefined || b.endTime === undefined
      ? 0
      : a.endTime.valueOf() - b.endTime.valueOf(),
  );

  filtered.sort((a, b) => a.startTime.valueOf() - b.startTime.valueOf());

  return filtered;
});

const isFilteringPastEvents = computed(() => {
  return filterCriteria.hidePastEvents && filteredEvents.value.length < events.value.length;
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
    <span
      class="text-muted-color flex gap-2 justify-center"
      v-if="isFilteringPastEvents"
      data-testid="program-past-events-hidden-notice"
    >
      <SimpleIcon class="text-lg" icon="eye-slash-fill" />
      <span class="italic">past events hidden</span>
    </span>
    <div class="flex flex-col gap-8">
      <EventProgramDay
        v-for="(day, index) in days"
        :key="index"
        :day-name="day.dayName"
        :events="day.events"
        :focused-event-id="props.focusedEventId"
        :day-index="index"
        :all-categories="allCategories"
      />
    </div>
  </div>
</template>
