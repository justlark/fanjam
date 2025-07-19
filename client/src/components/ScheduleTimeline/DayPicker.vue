<script setup lang="ts">
import IconButton from "@/components/system/IconButton.vue";

const currentDayIndex = defineModel("day", {
  type: Number,
  default: 0,
});

const props = defineProps<{
  dayNames: Array<string>;
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
</script>

<template>
  <nav class="flex items-center justify-center gap-4">
    <IconButton
      icon="chevron-left"
      label="Previous Day"
      :disabled="currentDayIndex === 0"
      @click="selectPrevDay"
    />
    <span class="text-xl">{{ props.dayNames[currentDayIndex] }}</span>
    <IconButton
      icon="chevron-right"
      label="Next Day"
      :disabled="currentDayIndex === props.dayNames.length - 1"
      @click="selectNextDay"
    />
  </nav>
</template>
