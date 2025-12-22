<script setup lang="ts">
import { computed, ref } from "vue";
import VirtualScroller from "primevue/virtualscroller";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import useFilterQuery from "@/composables/useFilterQuery";
import SimpleIcon from "./SimpleIcon.vue";
import EventProgramDescription from "./EventProgramDescription.vue";
import ScheduleHeader from "./ScheduleHeader.vue";

const {
  data: { events },
} = useRemoteData();

const props = defineProps<{
  focusedEventId?: string;
}>();

const filterCriteria = useFilterQuery();
const filteredEventIds = ref<Array<string>>();

const allCategories = computed(() => getSortedCategories(events.value));

const filteredEventIdsSet = computed(() =>
  filteredEventIds.value !== undefined ? new Set(filteredEventIds.value) : undefined,
);

const filteredEvents = computed(() => {
  const filtered = events.value.filter((event) => filteredEventIdsSet.value?.has(event.id) ?? true);

  filtered.sort((a, b) =>
    a.endTime === undefined || b.endTime === undefined
      ? 0
      : a.endTime.valueOf() - b.endTime.valueOf(),
  );

  filtered.sort((a, b) => a.startTime.valueOf() - b.startTime.valueOf());

  return filtered;
});

const isFilteringPastEvents = computed(() => {
  return filterCriteria.hidePastEvents && filteredEvents.value.length < events.value.length;
});
</script>

<template>
  <div class="flex flex-col gap-4 h-full">
    <ScheduleHeader v-model:ids="filteredEventIds" />
    <span
      class="text-muted-color flex gap-2 justify-center"
      v-if="isFilteringPastEvents"
      data-testid="program-past-events-hidden-notice"
    >
      <SimpleIcon class="text-lg" icon="eye-slash-fill" />
      <span class="italic">past events hidden</span>
    </span>
    <VirtualScroller :items="events" :item-size="50" scroll-height="100%">
      <template v-slot:item="{ item, options }">
        <EventProgramDescription
          :key="item.id"
          :event="item"
          :expand="item.id === props.focusedEventId"
          :all-categories="allCategories"
          :class="{ 'mb-4': options.index < options.count - 1 }"
        />
      </template>
    </VirtualScroller>
  </div>
</template>
