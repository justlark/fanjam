<script setup lang="ts">
import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { dateRangeToDayNames, localizeTime } from "@/utils/time";

import api, { type Event } from "@/utils/api";
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";
import { type Day } from "@/components/ScheduleTimeline/ScheduleTimeline.vue";

const route = useRoute();
const envId = computed(() => route.params.envId as string);

const events = ref<Array<Event>>([]);
const days = ref<Array<Day>>([]);

watchEffect(async () => {
  const result = await api.getEvents(envId.value);

  // TODO: Handle a 404 from this endpoint and serve the 404 page.

  if (!result.ok) {
    events.value = [];
    return;
  }

  events.value = result.value;
});

// TODO: Rewrite and optimize.
watchEffect(async () => {
  days.value = [];

  const allDates = events.value.reduce((set, event) => {
    set.add(event.startTime);
    set.add(event.endTime);
    return set;
  }, new Set<Date>());

  const namedDays = dateRangeToDayNames(allDates);

  for (const { dayName, times } of namedDays) {
    days.value.push({
      dayName,
      timeSlots: events.value
        .filter(
          (event) =>
            event.startTime &&
            (times.has(event.startTime) || (event.endTime && times.has(event.endTime))),
        )
        .map((event) => ({
          localizedTime: localizeTime(event.startTime),
          events: [
            {
              id: event.id,
              title: event.name,
              category: event.category,
            },
          ],
        })),
    });
  }
});
</script>

<template>
  <SiteNav title="My Con">
    <ScheduleTimeline class="p-6" :days="days" />
  </SiteNav>
</template>
