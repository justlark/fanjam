<script setup lang="ts">
import ScheduleTimeSlot, { EventSummary } from "@/components/ScheduleTimeSlot.vue";

export interface TimeSlot {
  time: string;
  events: Array<EventSummary>;
}

const props = defineProps<{
  events: Array<TimeSlot>;
}>();
</script>

<template>
  <div class="flex flex-col gap-8">
    <section v-for="(event, index) in props.events" :key="index">
      <ScheduleTimeSlot :time="event.time" :events="event.events" />
    </section>
  </div>
</template>
