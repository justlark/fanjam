<script setup lang="ts">
import ScheduleTimeSlot, { type EventSummary } from "@/components/ScheduleTimeSlot.vue";
import ScheduleHeader from "@/components/ScheduleHeader.vue";

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
    <ScheduleHeader />
    <div class="flex flex-col gap-8">
      <section v-for="(event, index) in props.events" :key="index">
        <ScheduleTimeSlot :time="event.time" :events="event.events" />
      </section>
    </div>
  </div>
</template>
