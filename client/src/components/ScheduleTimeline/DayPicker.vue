<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import IconButton from "@/components/system/IconButton.vue";

const router = useRouter();

const route = useRoute();
const currentDayIndex = computed(() =>
  route.params.dayIndex ? parseInt(route.params.dayIndex as string, 10) : 0,
);

const props = defineProps<{
  dayNames: Array<string>;
}>();

const selectNextDay = () => {
  if (currentDayIndex.value < props.dayNames.length - 1) {
    router.push({
      name: "schedule",
      params: { dayIndex: currentDayIndex.value + 1 },
    });
  }
};

const selectPrevDay = () => {
  if (currentDayIndex.value > 0) {
    router.push({
      name: "schedule",
      params: { dayIndex: currentDayIndex.value - 1 },
    });
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
