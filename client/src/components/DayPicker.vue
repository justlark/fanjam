<script setup lang="ts">
import IconButton from "./IconButton.vue";

const currentDayIndex = defineModel<number>("day", { required: true });

const props = defineProps<{
  dayNames: Array<string>;
  todayIndex: number | undefined;
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
  <nav class="flex items-center justify-between gap-4">
    <span class="text-2xl font-bold">{{ props.dayNames[currentDayIndex] }}</span>
    <span class="flex items-center">
      <IconButton
        icon="calendar-event"
        label="Today"
        :show-label="true"
        size="sm"
        :active="currentDayIndex === props.todayIndex"
        v-if="props.todayIndex !== undefined"
        @click="selectToday"
      />
      <IconButton
        icon="chevron-left"
        label="Previous"
        :disabled="currentDayIndex === 0"
        @click="selectPrevDay"
      />
      <IconButton
        icon="chevron-right"
        label="Next"
        :disabled="currentDayIndex === props.dayNames.length - 1"
        @click="selectNextDay"
      />
    </span>
  </nav>
</template>
