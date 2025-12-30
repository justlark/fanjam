<script setup lang="ts">
import { computed, type DeepReadonly } from "vue";
import { type Event } from "@/utils/api";
import TagBar from "./TagBar.vue";
import Drawer from "primevue/drawer";
import IconButton from "./IconButton.vue";
import SimpleIcon from "./SimpleIcon.vue";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import useIsEventStarred from "@/composables/useIsEventStarred";
import { renderMarkdown } from "@/utils/markdown";

const isVisible = defineModel<boolean>("visible", {
  required: true,
});

const props = defineProps<{
  event?: DeepReadonly<Event>;
  day: number;
  allCategories: Array<string>;
}>();

const filterCriteria = useFilterQuery();
const isStarred = useIsEventStarred(computed(() => props.event?.id));

const descriptionHtml = computed(() => {
  if (!props.event?.description) return undefined;
  return renderMarkdown(props.event.description);
});
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
      <div class="flex flex-col mx-4 mt-4 h-full" data-testid="event-summary-drawer">
        <div class="sticky top-0 pb-2 bg-white dark:bg-surface-900 flex gap-2 items-start">
          <h2 class="text-lg font-bold my-auto me-auto" v-if="props.event">
            {{ props.event.name }}
          </h2>
          <div class="flex">
            <IconButton
              label="Star"
              :icon="isStarred ? 'star-fill' : 'star'"
              :button-props="{
                'aria-pressed': isStarred,
                'data-testid': 'event-summary-star-button',
              }"
              @click="isStarred = !isStarred"
            />
            <IconButton
              size="md"
              icon="x-lg"
              label="Close"
              @click="closeCallback"
              :button-props="{ 'data-testid': 'event-summary-close-button' }"
            />
          </div>
        </div>
        <div class="overflow-hidden h-full">
          <TagBar
            class="mb-2"
            v-if="props.event"
            size="sm"
            :category="props.event.category"
            :tags="props.event.tags"
            :all-categories="props.allCategories"
          />
          <div class="mb-12">
            <div v-if="props.event?.summary">
              {{ props.event.summary }}
            </div>
            <div v-else-if="descriptionHtml" v-html="descriptionHtml" v-plaintext />
          </div>
        </div>
        <div
          class="absolute bottom-0 h-14 inset-x-0 bg-linear-to-b from-transparent dark:via-surface-900/80 via-white/80 via-40% dark:to-surface-900 to-white"
        />
        <div class="absolute bottom-0 left-0 w-full flex items-center justify-center">
          <RouterLink
            v-if="props.event"
            :to="{
              name: 'event',
              params: { eventId: props.event.id },
              query: toFilterQueryParams(filterCriteria),
              state: { from: 'schedule' },
            }"
            data-testid="event-summary-show-more-button"
          >
            <div class="mb-2 flex gap-1">
              <SimpleIcon class="text-lg" icon="caret-down-fill" />
              <span>Show more</span>
            </div>
          </RouterLink>
        </div>
      </div>
    </template>
  </Drawer>
</template>
