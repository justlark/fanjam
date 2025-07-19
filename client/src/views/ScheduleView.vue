<script setup lang="ts">
import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { datesToDayNames, dateIsBetween, groupByTime } from "@/utils/time";

import api, { type Event } from "@/utils/api";
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";
import { type Day } from "@/components/ScheduleTimeline/ScheduleTimeline.vue";

const route = useRoute();
const envId = computed(() => route.params.envId as string);

const events = ref<Array<Event>>([]);
const days = ref<Array<Day>>([]);

// TODO: Eventually, we'll want to persist this data to the local storage so
// the app can work offline.
watchEffect(async () => {
  const result = await api.getEvents(envId.value);

  // TODO: Handle a 404 from this endpoint and serve the 404 page.

  if (!result.ok) {
    events.value = [];
    return;
  }

  events.value = result.value;
});

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
          title: event.name,
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
