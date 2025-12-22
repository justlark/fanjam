<script setup lang="ts">
import { type DeepReadonly } from "vue";
import VirtualScroller from "primevue/virtualscroller";
import EventProgramDescription from "./EventProgramDescription.vue";
import { type Event } from "@/utils/api";

const props = defineProps<{
  focusedEventId?: string;
  filteredEvents: ReadonlyArray<DeepReadonly<Event>>;
  allCategories: ReadonlyArray<string>;
}>();
</script>

<template>
  <VirtualScroller :items="[...props.filteredEvents]" :item-size="50" scroll-height="100%">
    <template #item="{ item, options }">
      <EventProgramDescription
        :key="item.id"
        :event="item"
        :expand="item.id === props.focusedEventId"
        :all-categories="props.allCategories"
        :class="{ 'mb-4': options.index < options.count - 1 }"
      />
    </template>
  </VirtualScroller>
</template>
