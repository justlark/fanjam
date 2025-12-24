<script setup lang="ts">
import { ref, toRef, watch, computed, type Ref, type DeepReadonly } from "vue";
import EventProgramDescription from "./EventProgramDescription.vue";
import { type Event } from "@/utils/api";
import ScrollTop from "primevue/scrolltop";
import { useVirtualizer } from "@tanstack/vue-virtual";

const props = defineProps<{
  focusedEventId?: string;
  filteredEvents: ReadonlyArray<DeepReadonly<Event>>;
  allCategories: ReadonlyArray<string>;
}>();

const eventIndexById = computed(() => {
  const map = new Map<string, number>();

  for (const [index, event] of props.filteredEvents.entries()) {
    map.set(event.id, index);
  }

  return map;
});

const scrollerRef = ref<HTMLElement | null>(null);

const virtualizerOptions = computed(() => ({
  count: props.filteredEvents.length,
  getScrollElement: () => scrollerRef.value,
  // Keep this in sync with `EventProgramDescription`. Use the browser dev
  // tools to measure if needed. Make sure you include the margin.
  estimateSize: () => 114,
  getItemKey: (index: number) => props.filteredEvents[index].id,
  overscan: 5,
}));

const rowVirtualizer = useVirtualizer(virtualizerOptions);

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
const totalSize = computed(() => rowVirtualizer.value.getTotalSize());

const measureElement = (element?: Ref<HTMLElement>) => {
  if (!element) {
    return;
  }

  rowVirtualizer.value.measureElement(element.value);

  return undefined;
};

const isScrollElementLoaded = computed(() => rowVirtualizer.value.scrollElement !== null);

watch(
  [toRef(props, "focusedEventId"), isScrollElementLoaded],
  () => {
    if (!props.focusedEventId) {
      return;
    }

    const index = eventIndexById.value.get(props.focusedEventId);

    if (index === undefined) {
      return;
    }

    rowVirtualizer.value.scrollToIndex(index, {
      align: "start",
    });
  },
  { immediate: true },
);
</script>

<template>
  <div ref="scrollerRef" class="h-full overflow-y-auto" :style="{ contain: 'strict' }">
    <div class="relative" :style="{ height: `${totalSize}px` }">
      <div
        class="absolute top-0 left-0 w-full"
        :style="{ transform: `translateY(${virtualRows[0]?.start ?? 0}px)` }"
      >
        <EventProgramDescription
          v-for="row in virtualRows"
          :key="row.key.toString()"
          :data-index="row.index"
          :ref="measureElement"
          :event="props.filteredEvents[row.index]"
          :expand="props.filteredEvents[row.index].id === props.focusedEventId"
          :all-categories="props.allCategories"
          class="mb-[20px]"
        />
      </div>
    </div>
    <ScrollTop target="parent" />
  </div>
</template>
