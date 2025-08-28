<script setup lang="ts">
import { ref, type DeepReadonly, useId } from "vue";
import EventProgramDescription from "./EventProgramDescription.vue";
import IconButton from "./IconButton.vue";
import { type Event } from "@/utils/api";

const expanded = ref(false);

const props = defineProps<{
  dayName: string;
  dayIndex: number;
  events: ReadonlyArray<DeepReadonly<Event>>;
  allCategories: Array<string>;
}>();

const sectionHeading = useId();
</script>

<template>
  <section class="flex flex-col gap-2" :aria-labelledby="sectionHeading">
    <div class="flex items-center justify-between">
      <h2 :id="sectionHeading" class="text-2xl font-bold">{{ props.dayName }}</h2>
      <IconButton
        v-if="events.length > 0"
        size="sm"
        :label="expanded ? 'Collapse All' : 'Expand All'"
        :show-label="true"
        :icon="expanded ? 'arrows-collapse' : 'arrows-expand'"
        @click="expanded = !expanded"
      />
    </div>
    <div v-if="events.length > 0" class="flex flex-col gap-4">
      <div v-for="event in props.events" :key="event.id">
        <EventProgramDescription
          :event="event"
          :day-index="props.dayIndex"
          :expand="expanded"
          :all-categories="allCategories"
        />
      </div>
    </div>
    <div v-else class="text-center text-lg italic text-surface-500 dark:text-surface-400">
      No events
    </div>
  </section>
</template>
