<script setup lang="ts">
import Panel from "primevue/panel";
import IconButton from "./IconButton.vue";
import SimpleIcon from "./SimpleIcon.vue";
import TagBar from "./TagBar.vue";
import * as commonmark from "commonmark";
import { RouterLink } from "vue-router";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useStarredEvents from "@/composables/useStarredEvents";
import { type Event } from "@/utils/api";
import { computed, useId } from "vue";
import { localizeTimeSpan } from "@/utils/time";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";

const props = defineProps<{
  expand: boolean;
  event: Event;
  dayIndex: number;
  allCategories: Array<string>;
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

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const descriptionHtml = computed(() => {
  if (!props.event.description) return undefined;
  const parsed = mdReader.parse(props.event.description);
  return mdWriter.render(parsed);
});

const eventNameHeadingId = useId();
</script>

<template>
  <Panel
    :toggleable="event.description !== undefined"
    :collapsed="!props.expand"
    pt:content:class="!pb-2"
    pt:footer:class="!pb-2"
  >
    <template #header>
      <div class="flex flex-col">
        <span>
          <SimpleIcon v-if="isStarred" icon="star-fill" label="Star" class="me-2" />
          <h3 class="inline text-lg font-bold" :id="eventNameHeadingId">{{ props.event.name }}</h3>
        </span>
        <span class="text-muted-color" v-if="datetimeFormats !== undefined">{{
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
      <article
        id="document"
        class="*:first:mt-0 *:last:mb-0"
        v-if="descriptionHtml"
        v-html="descriptionHtml"
        :aria-labelledby="eventNameHeadingId"
      ></article>
    </div>
    <template #footer>
      <div class="flex items-center justify-evenly gap-2">
        <IconButton
          :icon="isStarred ? 'star-fill' : 'star'"
          label="Star"
          :show-label="true"
          size="sm"
          @click="toggleStarred"
        />
        <RouterLink
          :to="{
            name: 'event',
            params: { eventId: props.event.id },
            state: { from: 'program' },
            query: toFilterQueryParams(filterCriteria),
          }"
        >
          <IconButton icon="arrows-angle-expand" label="Open" :show-label="true" size="sm" />
        </RouterLink>
      </div>
    </template>
  </Panel>
</template>
