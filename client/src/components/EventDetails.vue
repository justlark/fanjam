<script setup lang="ts">
import { computed, useId } from "vue";
import { RouterLink } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useStarredEvents from "@/composables/useStarredEvents";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import * as commonmark from "commonmark";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import TagBar from "./TagBar.vue";

const starredEvents = useStarredEvents();
const datetimeFormats = useDatetimeFormats();
const filterCriteria = useFilterQuery();

const props = defineProps<{
  event: Event;
  day: number;
  allCategories: Array<string>;
  from: "schedule" | "program" | undefined;
}>();

const event = computed(() => props.event);

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  const parsed = mdReader.parse(event.value.description);
  return mdWriter.render(parsed);
});

const isStarred = computed(() => starredEvents.value.has(event.value.id));

const toggleStarred = () => {
  if (isStarred.value) {
    starredEvents.value.delete(event.value.id);
  } else {
    starredEvents.value.add(event.value.id);
  }
};

const sectionHeadingId = useId();
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <span class="flex items-center">
        <RouterLink
          v-if="props.from !== undefined"
          :to="{
            name: props.from,
            params: props.from === 'schedule' ? { dayIndex: props.day } : {},
            query: toFilterQueryParams(filterCriteria),
          }"
        >
          <IconButton class="lg:!hidden" icon="chevron-left" label="Back" />
          <IconButton class="!hidden lg:!block" icon="x-lg" label="Close" />
        </RouterLink>
        <IconButton
          class="hidden lg:block"
          label="Star"
          :icon="isStarred ? 'star-fill' : 'star'"
          @click="toggleStarred()"
        />
      </span>
      <h2 :id="sectionHeadingId" class="text-xl font-bold">{{ event.name }}</h2>
    </div>
    <div class="px-6">
      <div class="flex justify-between items-end">
        <dl class="flex flex-col items-start gap-2">
          <EventDetail v-if="datetimeFormats && event.startTime" icon="clock" icon-label="Time">
            {{ localizeTimeSpan(datetimeFormats, event.startTime, event.endTime) }}
          </EventDetail>
          <EventDetail v-if="event.people.length > 0" icon="person-circle" icon-label="Hosts">
            Hosted by {{ event.people.join(", ") }}
          </EventDetail>
          <EventDetail v-if="event.location" icon="geo-alt-fill" icon-label="Location">
            {{ event.location }}
          </EventDetail>
        </dl>
        <IconButton
          class="lg:hidden"
          label="Star"
          :icon="isStarred ? 'star-fill' : 'star'"
          :active="isStarred"
          inactive-variant="filled"
          :button-props="{
            raised: true,
          }"
          @click="toggleStarred()"
        />
      </div>
      <TagBar
        class="mt-4"
        :category="event.category"
        :tags="event.tags"
        :all-categories="props.allCategories"
      />
      <Divider />
      <article
        id="document"
        v-if="descriptionHtml"
        v-html="descriptionHtml"
        :aria-labelledby="sectionHeadingId"
        class="my-4"
      ></article>
      <div v-else class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8">
        <span>No description</span>
      </div>
    </div>
  </section>
</template>
