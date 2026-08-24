<script setup lang="ts">
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useNow from "@/composables/useNow";
import type { Event } from "@/utils/api";
import { dateIsBetween, earliest, groupByTime, latest, startOfScheduleDay } from "@/utils/time";
import { computed, type DeepReadonly } from "vue";
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

  // The weekday and date come from the schedule day the event belongs to, so
  // that late-night events are labeled consistently with the By Day view. The
  // time is always the event's actual wall-clock start time.
  const scheduleDay = (time: Date) =>
    startOfScheduleDay(time, formats.timezone, formats.dayCutoffMinutes);

  const groupedEvents = groupByTime(
    props.events,
    (event) => event.startTime,
    (time) => `${scheduleDay(time).toISOString()} ${formats.shortTime.format(time)}`,
    (time) => {
      const dayStart = scheduleDay(time);

      return {
        weekday: formats.longWeekday.format(dayStart),
        date: formats.mediumDate.format(dayStart),
        time: formats.shortTime.format(time),
      };
    },
  );
  return groupedEvents.map(([{ weekday, date, time }, eventsInThisTimeSlot]) => ({
    weekday,
    date,
    time,
    events: eventsInThisTimeSlot,
  }));
});

const now = useNow();

const currentTimeSlotIndex = computed(() => {
  return timeSlots.value.reduce<number | undefined>((prev, thisSlot, index) => {
    const thisSlotStartTimes = thisSlot.events.map((event) => event.startTime);
    const startTime = earliest(...thisSlotStartTimes);

    const thisSlotEndTimes = thisSlot.events.map((event) => event.endTime);
    const endTime = latest(...thisSlotEndTimes);

    // Skipping the entry for now, but hitting this condition probably means something is very weird with the data -
    // A timeslot either has no events, or no events with defined end times.
    if (startTime === undefined || endTime === undefined) return prev;
    return dateIsBetween(now.value, startTime, endTime) ? index : prev;
  }, undefined);
});

const incrementalTimeSlots = useIncremental(timeSlots);
</script>

<template>
  <div v-if="incrementalTimeSlots.length > 0" :class="['flex flex-col gap-6']">
    <ScheduleTimeSlot
      v-for="(timeSlot, index) in timeSlots"
      v-model:focused="focusedEventId"
      :key="index"
      :day-name="timeSlot"
      :events="timeSlot.events"
      :all-categories="allCategories"
      :is-current-time-slot="index === currentTimeSlotIndex"
      :view-type="viewType"
      data-testid="schedule-time-slot"
    />
  </div>
  <div
    v-else
    class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8"
    data-testid="schedule-no-events"
  >
    No events
  </div>
</template>
