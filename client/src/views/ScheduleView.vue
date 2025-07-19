<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { datesToDayNames, dateIsBetween, groupByTime } from "@/utils/time";
import useEvents from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";
import { type Day } from "@/components/ScheduleTimeline/ScheduleTimeline.vue";

const events = useEvents();
const days = ref<Array<Day>>([]);

watchEffect(async () => {
  const allDates = events.value.reduce((set, event) => {
    if (event.startTime) {
      set.add(event.startTime);
    }

    if (event.endTime) {
      set.add(event.endTime);
    }

    return set;
  }, new Set<Date>());

  const namedDays = datesToDayNames(allDates);

  days.value = namedDays.map(({ dayName, dayStart, dayEnd }) => {
    const eventsThisDay = events.value.filter(
      (event) => event.startTime && dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    const groupedEvents = groupByTime(eventsThisDay, (event) => event.startTime);

    return {
      dayName,
      timeSlots: [...groupedEvents.entries()].map(([localizedTime, eventsInThisTimeSlot]) => ({
        localizedTime,
        events: eventsInThisTimeSlot.map((event) => ({
          id: event.id,
          name: event.name,
          category: event.category,
        })),
      })),
    };
  });
});
</script>

<template>
  <SiteNav title="My Con">
    <ScheduleTimeline class="p-6" :days="days" />
  </SiteNav>
</template>
