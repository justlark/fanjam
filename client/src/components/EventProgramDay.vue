<script setup lang="ts">
import { ref } from "vue";
import EventProgramDescription from "./EventProgramDescription.vue";
import IconButton from "./IconButton.vue";
import { type Event } from "@/utils/api";

const expanded = ref(false);

const props = defineProps<{
  dayName: string;
  dayIndex: number;
  events: Array<Event>;
  allCategories: Array<string>;
}>();
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between">
      <h2 class="text-2xl font-bold">{{ props.dayName }}</h2>
      <IconButton
        size="sm"
        :label="expanded ? 'Collapse All' : 'Expand All'"
        :show-label="true"
        :icon="expanded ? 'arrows-collapse' : 'arrows-expand'"
        @click="expanded = !expanded"
      />
    </div>
    <div class="flex flex-col gap-4">
      <div v-for="event in props.events" :key="event.id">
        <EventProgramDescription
          :event="event"
          :day-index="props.dayIndex"
          :expand="expanded"
          :all-categories="allCategories"
        />
      </div>
    </div>
  </div>
</template>
