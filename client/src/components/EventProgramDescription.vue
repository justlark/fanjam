<script setup lang="ts">
import Panel from "primevue/panel";
import IconButton from "./IconButton.vue";
import SimpleIcon from "./SimpleIcon.vue";
import TagBar from "./TagBar.vue";
import { RouterLink } from "vue-router";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useStarredEvents from "@/composables/useStarredEvents";
import { renderMarkdown } from "@/utils/markdown";
import { type Event } from "@/utils/api";
import { ref, computed, type DeepReadonly, useId } from "vue";
import { localizeTimeSpan } from "@/utils/time";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";

// This component deliberately has a fixed height when collapsed so it can be
// used with virtual scrollers.

const props = defineProps<{
  expand: boolean;
  event: DeepReadonly<Event>;
  allCategories: ReadonlyArray<string>;
}>();

const datetimeFormats = useDatetimeFormats();
const starredEvents = useStarredEvents();
const filterCriteria = useFilterQuery();

const isStarred = computed(() => starredEvents.value.has(props.event.id));

const toggleStarred = () => {
  if (isStarred.value) {
    starredEvents.value.delete(props.event.id);
  } else {
    starredEvents.value.add(props.event.id);
  }
};

const descriptionHtml = computed(() => {
  if (!props.event.description) return undefined;
  return renderMarkdown(props.event.description);
});

const collapsed = ref(!props.expand);

const onUpdateCollapsed = (value: boolean) => {
  collapsed.value = value;
};

const eventNameHeadingId = useId();
</script>

<template>
  <Panel
    :toggleable="true"
    :collapsed="!props.expand"
    @update:collapsed="onUpdateCollapsed"
    pt:content="!pb-2"
    pt:footer="!pb-2"
    pt:root:data-testid="program-event"
    pt:pc-toggle-button:root:data-testid="program-event-expand-button"
  >
    <template #header>
      <div
        :class="[
          'flex flex-col justify-between text-ellipsis break-all',
          { 'h-[80px]': collapsed },
        ]"
      >
        <div class="flex flex-col justify-center grow">
          <span
            data-testid="program-event-name"
            :class="['overflow-hidden', { 'line-clamp-2': collapsed }]"
          >
            <SimpleIcon v-if="isStarred" icon="star-fill" label="Starred:" class="me-2" />
            <h3 class="inline text-lg font-bold" :id="eventNameHeadingId">
              {{ props.event.name }}
            </h3>
          </span>
        </div>
        <span class="text-muted-color line-clamp-1" v-if="datetimeFormats !== undefined">{{
          localizeTimeSpan(datetimeFormats, props.event.startTime, props.event.endTime)
        }}</span>
      </div>
    </template>
    <div class="flex flex-col gap-4 mt-2">
      <TagBar
        size="sm"
        :category="props.event.category"
        :tags="props.event.tags"
        :all-categories="props.allCategories"
      />
      <article id="document" :aria-labelledby="eventNameHeadingId">
        <p v-if="event.summary" class="mt-0">{{ event.summary }}</p>
        <div v-if="descriptionHtml" v-html="descriptionHtml" class="*:last:mb-0"></div>
      </article>
    </div>
    <template #footer>
      <div class="flex items-center justify-evenly gap-2">
        <IconButton
          :icon="isStarred ? 'star-fill' : 'star'"
          label="Star"
          :show-label="true"
          size="sm"
          @click="toggleStarred"
          :button-props="{
            'aria-pressed': isStarred ? 'true' : 'false',
            'data-testid': 'program-event-star-button',
          }"
        />
        <RouterLink
          :to="{
            name: 'event',
            params: { eventId: props.event.id },
            state: { from: 'program' },
            query: toFilterQueryParams(filterCriteria),
          }"
        >
          <IconButton
            icon="arrows-angle-expand"
            label="Open"
            :show-label="true"
            size="sm"
            :button-props="{
              'data-testid': 'program-event-details-button',
            }"
          />
        </RouterLink>
      </div>
    </template>
  </Panel>
</template>
