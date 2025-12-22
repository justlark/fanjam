<script setup lang="ts">
import { computed, defineAsyncComponent, readonly, ref } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import useFilterQuery from "@/composables/useFilterQuery";
import SimpleIcon from "./SimpleIcon.vue";
import ScheduleHeader from "./ScheduleHeader.vue";
import ProgressSpinner from "primevue/progressspinner";

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

const EventProgramScroller = defineAsyncComponent(() => import("./EventProgramScroller.vue"));
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
    <Suspense>
      <template #default>
        <EventProgramScroller
          :focused-event-id="props.focusedEventId"
          :filtered-events="readonly(filteredEvents)"
          :all-categories="allCategories"
        />
      </template>
      <template #fallback>
        <div class="flex items-center h-full">
          <ProgressSpinner />
        </div>
      </template>
    </Suspense>
  </div>
</template>
