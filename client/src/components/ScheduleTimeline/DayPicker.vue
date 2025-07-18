<script setup lang="ts">
import Button from "primevue/button";

const currentDayIndex = defineModel({ type: Number, default: 0 });

const props = defineProps<{
  dayNames: Array<string>;
}>();

const selectNextDay = () => {
  if (currentDayIndex.value < props.dayNames.length - 1) {
    currentDayIndex.value++;
  }
};

const selectPrevDay = () => {
  if (currentDayIndex.value > 0) {
    currentDayIndex.value--;
  }
};
</script>

<template>
  <nav class="flex items-center justify-center gap-4">
    <Button
      icon="bi bi-chevron-left"
      pt:icon="!text-2xl"
      variant="text"
      size="large"
      aria-label="Previous Day"
      rounded
      @click="selectPrevDay"
    />
    <span class="text-xl">{{ props.dayNames[currentDayIndex] }}</span>
    <Button
      icon="bi bi-chevron-right"
      pt:icon="!text-2xl"
      variant="text"
      size="large"
      aria-label="Next Day"
      rounded
      @click="selectNextDay"
    />
  </nav>
</template>
