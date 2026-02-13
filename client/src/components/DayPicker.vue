<script setup lang="ts">
import IconButton from "./IconButton.vue";

const currentDayIndex = defineModel<number>("day", { required: true });

const props = defineProps<{
  dayNames: Array<string>;
  todayIndex: number | undefined;
  dayDate: Date | undefined;
  viewType: "daily" | "all";
}>();

const selectNextDay = () => {
  if (currentDayIndex.value < props.dayNames.length - 1) {
    currentDayIndex.value += 1;
  }
};

const selectPrevDay = () => {
  if (currentDayIndex.value > 0) {
    currentDayIndex.value -= 1;
  }
};

const selectToday = () => {
  if (props.todayIndex !== undefined) {
    currentDayIndex.value = props.todayIndex;
  }
};
</script>

<template>
  <nav
    class="flex items-center justify-between gap-4 min-h-12 mt-6"
    :class="{ 'mb-6': viewType === 'daily' }"
  >
    <span class="text-2xl font-bold" data-testid="schedule-day-name">
      {{ props.dayNames[currentDayIndex] }}
      <span
        class="text-sm font-bold text-muted-color text-nowrap"
        v-if="props.dayDate !== undefined"
        data-testid="schedule-day-name"
      >
        {{
          props.dayDate.toLocaleDateString(undefined, {
            month: "short",
            day: "numeric",
            year: "numeric",
          })
        }}
      </span>
    </span>
    <span v-if="viewType === 'daily'" class="flex items-center">
      <IconButton
        icon="calendar-event"
        label="Today"
        :show-label="true"
        size="sm"
        :disabled="currentDayIndex === props.todayIndex"
        v-if="props.todayIndex !== undefined"
        @click="selectToday"
        :button-props="{
          'data-testid': 'schedule-today-button',
        }"
      />
      <IconButton
        icon="chevron-left"
        label="Previous"
        :disabled="currentDayIndex === 0"
        @click="selectPrevDay"
        :button-props="{
          'data-testid': 'schedule-prev-day-button',
        }"
      />
      <IconButton
        icon="chevron-right"
        label="Next"
        :disabled="currentDayIndex >= props.dayNames.length - 1"
        @click="selectNextDay"
        :button-props="{
          'data-testid': 'schedule-next-day-button',
        }"
      />
    </span>
  </nav>
</template>
