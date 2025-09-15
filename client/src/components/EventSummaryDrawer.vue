<script setup lang="ts">
import { type DeepReadonly } from "vue";
import { type Event } from "@/utils/api";
import TagBar from "./TagBar.vue";
import Drawer from "primevue/drawer";
import IconButton from "./IconButton.vue";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";

const isVisible = defineModel<boolean>("visible", {
  required: true,
});

const filterCriteria = useFilterQuery();

const props = defineProps<{
  event?: DeepReadonly<Event>;
  day: number;
  allCategories: Array<string>;
}>();
</script>

<template>
  <Drawer
    class="!h-[15rem]"
    v-model:visible="isVisible"
    :dismissable="false"
    :modal="false"
    position="bottom"
  >
    <template #container="{ closeCallback }">
      <div class="flex flex-col mx-4 mt-4 overflow-auto" data-testid="event-summary-drawer">
        <div class="sticky top-0 pb-2 bg-white dark:bg-surface-900 flex gap-2 items-center">
          <h2 class="text-lg font-bold me-auto" v-if="props.event">
            {{ props.event.name }}
          </h2>
          <RouterLink
            v-if="props.event"
            :to="{
              name: 'event',
              params: { eventId: props.event.id },
              query: toFilterQueryParams(filterCriteria),
              state: { from: 'schedule' },
            }"
          >
            <IconButton size="md" icon="arrows-angle-expand" label="Expand" />
          </RouterLink>
          <IconButton size="md" icon="x-lg" label="Close" @click="closeCallback" />
        </div>
        <TagBar
          class="mb-2"
          v-if="props.event"
          size="sm"
          :category="props.event.category"
          :tags="props.event.tags"
          :all-categories="props.allCategories"
        />
        <div v-if="props.event?.summary">
          {{ props.event.summary }}
        </div>
      </div>
    </template>
  </Drawer>
</template>
