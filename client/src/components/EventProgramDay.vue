<script setup lang="ts">
import { ref, toRef, watchEffect, type DeepReadonly, useId } from "vue";
import useIncremental from "@/composables/useIncremental";
import EventProgramDescription from "./EventProgramDescription.vue";
import IconButton from "./IconButton.vue";
import VirtualScroller from "primevue/virtualscroller";
import { type Event } from "@/utils/api";

const expanded = ref(false);

const props = defineProps<{
  dayName: string;
  dayIndex: number;
  events: ReadonlyArray<DeepReadonly<Event>>;
  focusedEventId?: string;
  allCategories: Array<string>;
}>();

const sectionHeading = useId();

const incrementalEvents = useIncremental(toRef(props, "events"));
</script>

<template>
  <section class="flex flex-col gap-2" :aria-labelledby="sectionHeading">
    <div class="flex items-center justify-between">
      <h2 :id="sectionHeading" class="text-2xl font-bold">{{ props.dayName }}</h2>
      <IconButton
        v-if="events.length > 0"
        size="sm"
        :label="expanded ? 'Collapse' : 'Expand'"
        :show-label="true"
        :icon="expanded ? 'arrows-collapse' : 'arrows-expand'"
        @click="expanded = !expanded"
        :button-props="{ 'data-testid': 'program-day-expand-button' }"
      />
    </div>
    <div v-if="events.length > 0">
      <VirtualScroller
        :items="[...(incrementalEvents.value ?? [])]"
        :itemSize="1"
        scrollHeight="80vh"
      >
        <template v-slot:item="{ item, options }">
          <EventProgramDescription
            :event="item"
            :day-index="props.dayIndex"
            :expand="item.id === props.focusedEventId"
            :all-categories="allCategories"
            :class="{
              'mb-4': !options.last,
            }"
          />
        </template>
      </VirtualScroller>
      <div
        v-if="!expanded"
        class="text-center text-lg italic text-surface-500 dark:text-surface-400"
      >
        {{ events.length }} events
      </div>
    </div>
    <div v-else class="text-center text-lg italic text-surface-500 dark:text-surface-400">
      No events
    </div>
  </section>
</template>
