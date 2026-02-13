<script setup lang="ts">
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useInterval from "@/composables/useInterval";
import type { Event } from "@/utils/api";
import { dateIsBetween, earliest, groupByTime, latest } from "@/utils/time";
import { computed, ref, type DeepReadonly } from "vue";
import ScheduleTimeSlot from "./ScheduleTimeSlot.vue";
import useIncremental from "@/composables/useIncremental";

const focusedEventId = defineModel<string | undefined>("focused");

const props = defineProps<{
  events: DeepReadonly<Event>[];
  allCategories: Array<string>;
  viewType: "daily" | "all";
}>();

const datetimeFormats = useDatetimeFormats();

const timeSlots = computed(() => {
  if (datetimeFormats.value === undefined) return [];
  const formats = datetimeFormats.value;

  const groupedEvents = groupByTime(
    props.events,
    (event) => event.startTime,
    (time) => {
      const timeString = formats.shortTime.format(time);
      const dayName = formats.shortWeekday.format(time);
      return props.viewType === "daily" ? timeString : `${dayName} ${timeString}`;
    },
  );
  return [...groupedEvents.entries()].map(([localizedTime, eventsInThisTimeSlot]) => ({
    localizedTime,
    events: eventsInThisTimeSlot,
  }));
});

const now = ref(new Date());

const currentTimeSlotIndex = computed(() => {
  const index = timeSlots.value.findIndex((thisSlot, i, allSlots) => {
    const thisSlotStartTimes = thisSlot.events.map((event) => event.startTime);
    const startTime = earliest(...thisSlotStartTimes);

    const thisSlotEndTimes = thisSlot.events.map((event) => event.endTime);
    const thisEndTime = latest(...thisSlotEndTimes);

    const nextSlotStartTimes = allSlots[i + 1]?.events.map((event) => event.startTime) ?? [];
    const nextStartTime = earliest(...nextSlotStartTimes);

    const endTime = earliest(thisEndTime, nextStartTime);
    // Returning false for now, but hitting this condition probably means something is very weird with the data -
    // A timeslot either has no events, or no events with defined end times.
    if (startTime === undefined || endTime === undefined) return false;
    return dateIsBetween(now.value, startTime, endTime);
  });
  return index >= 0 ? index : undefined;
});

const REFRESH_NOW_TIME_INTERVAL_MILLIS = 1000 * 60 * 1;
useInterval(() => (now.value = new Date()), REFRESH_NOW_TIME_INTERVAL_MILLIS);

const incrementalTimeSlots = useIncremental(timeSlots);
</script>

<template>
  <div v-if="incrementalTimeSlots.length > 0" :class="['flex flex-col gap-6']">
    <ScheduleTimeSlot
      v-for="(timeSlot, index) in timeSlots"
      v-model:focused="focusedEventId"
      :key="index"
      :localized-time="timeSlot.localizedTime"
      :events="timeSlot.events"
      :all-categories="allCategories"
      :is-current-time-slot="index === currentTimeSlotIndex"
      :view-type="viewType"
      data-testid="schedule-time-slot"
    />
  </div>
</template>
