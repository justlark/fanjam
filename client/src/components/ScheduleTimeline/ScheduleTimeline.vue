<script setup lang="ts">
import { ref, computed, watchEffect } from "vue";
import { datesToDayNames, dateIsBetween, groupByTime } from "@/utils/time";
import { useRoute, useRouter } from "vue-router";
import { type Event } from "@/utils/api";
import DayPicker from "./DayPicker.vue";
import ScheduleTimeSlot, { type EventSummary } from "./ScheduleTimeSlot.vue";
import ScheduleHeader from "./ScheduleHeader.vue";

const route = useRoute();
const router = useRouter();

interface TimeSlot {
  localizedTime: string;
  events: Array<EventSummary>;
}

interface Day {
  dayName: string;
  timeSlots: Array<TimeSlot>;
}

const props = defineProps<{
  events: Array<Event>;
}>();

const currentDayIndex = defineModel("day", {
  type: Number,
  default: 0,
});

const days = ref<Array<Day>>([]);
const dayIndexByEventId = ref(new Map<string, number>());
const searchResultEventIds = ref<Array<string>>();

const allCategories = computed(() =>
  days.value.reduce<Array<string>>((set, day) => {
    day.timeSlots.forEach((timeSlot) => {
      timeSlot.events.forEach((event) => {
        if (!set.includes(event.category)) {
          set.push(event.category);
        }
      });
    });
    return set;
  }, []),
);

const dayNames = computed(() => days.value.map((day) => day.dayName));

watchEffect(() => {
  dayIndexByEventId.value.clear();

  const allDates = props.events.reduce((set, event) => {
    if (event.startTime) {
      set.add(event.startTime);
    }

    if (event.endTime) {
      set.add(event.endTime);
    }

    return set;
  }, new Set<Date>());

  const namedDays = datesToDayNames(allDates);

  days.value = [...namedDays.entries()].map(([dayIndex, { dayName, dayStart, dayEnd }]) => {
    const eventsThisDay = props.events.filter(
      (event) => event.startTime && dateIsBetween(event.startTime, dayStart, dayEnd),
    );

    const groupedEvents = groupByTime(eventsThisDay, (event) => event.startTime);

    for (const event of eventsThisDay) {
      dayIndexByEventId.value.set(event.id, dayIndex);
    }

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

watchEffect(async () => {
  if (route.name !== "schedule") {
    return;
  }

  await router.push({
    name: "schedule",
    params: { dayIndex: currentDayIndex.value },
  });
});

watchEffect(() => {
  if (route.name === "schedule") {
    currentDayIndex.value = route.params.dayIndex
      ? parseInt(route.params.dayIndex as string, 10)
      : 0;
  } else if (route.name === "event") {
    currentDayIndex.value = route.params.eventId
      ? (dayIndexByEventId.value.get(route.params.eventId as string) ?? 0)
      : 0;
  }
});
</script>

<template>
  <div class="flex flex-col gap-4">
    <ScheduleHeader
      v-model:ids="searchResultEventIds"
      :events="props.events"
      :all-categories="allCategories"
    />
    <DayPicker v-model:day="currentDayIndex" :day-names="dayNames" />
    <div v-if="days.length > 0" class="flex flex-col gap-8">
      <ScheduleTimeSlot
        v-for="(timeSlot, index) in days[currentDayIndex].timeSlots"
        v-model:ids="searchResultEventIds"
        :key="index"
        :localized-time="timeSlot.localizedTime"
        :events="timeSlot.events"
        :all-categories="allCategories"
      />
    </div>
  </div>
</template>
