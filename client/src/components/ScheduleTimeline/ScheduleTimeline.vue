<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

import DayPicker from "./DayPicker.vue";
import ScheduleTimeSlot, { type EventSummary } from "./ScheduleTimeSlot.vue";
import ScheduleHeader from "./ScheduleHeader.vue";

const route = useRoute();
const currentDayIndex = computed(() =>
  route.params.dayIndex ? parseInt(route.params.dayIndex as string, 10) : 0,
);

export interface TimeSlot {
  localizedTime: string;
  events: Array<EventSummary>;
}

export interface Day {
  dayName: string;
  timeSlots: Array<TimeSlot>;
}

const props = defineProps<{
  days: Array<Day>;
}>();

const allCategories = computed(() =>
  props.days.reduce((set, day) => {
    day.timeSlots.forEach((timeSlot) => {
      timeSlot.events.forEach((event) => {
        if (!set.includes(event.category)) {
          set.push(event.category);
        }
      });
    });
    return set;
  }, [] as Array<string>),
);

const dayNames = computed(() => props.days.map((day) => day.dayName));
</script>

<template>
  <div class="flex flex-col gap-4">
    <ScheduleHeader />
    <DayPicker :day-names="dayNames" />
    <div v-if="days.length > 0" class="flex flex-col gap-8">
      <ScheduleTimeSlot
        v-for="(timeSlot, index) in props.days[currentDayIndex].timeSlots"
        :key="index"
        :localized-time="timeSlot.localizedTime"
        :events="timeSlot.events"
        :all-categories="allCategories"
      />
    </div>
  </div>
</template>
