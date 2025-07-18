<script setup lang="ts">
import { computed } from "vue";

import ScheduleTimeSlot, { type EventSummary } from "@/components/ScheduleTimeSlot.vue";
import ScheduleHeader from "@/components/ScheduleHeader.vue";

export interface TimeSlot {
  time: string;
  events: Array<EventSummary>;
}

const props = defineProps<{
  events: Array<TimeSlot>;
}>();

const allCategories = computed(() =>
  props.events.reduce((set, slot) => {
    slot.events.forEach((event) => {
      if (!set.includes(event.category)) {
        set.push(event.category);
      }
    });

    return set;
  }, []),
);
</script>

<template>
  <div class="flex flex-col gap-8">
    <ScheduleHeader />
    <div class="flex flex-col gap-8">
      <section v-for="(event, index) in props.events" :key="index">
        <ScheduleTimeSlot
          :time="event.time"
          :events="event.events"
          :all-categories="allCategories"
        />
      </section>
    </div>
  </div>
</template>
