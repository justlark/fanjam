<script setup lang="ts">
import { computed, type DeepReadonly, useId } from "vue";
import { RouterLink } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useStarredEvents from "@/composables/useStarredEvents";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { renderMarkdown } from "@/utils/markdown";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import TagBar from "./TagBar.vue";

const starredEvents = useStarredEvents();
const datetimeFormats = useDatetimeFormats();
const filterCriteria = useFilterQuery();

const props = defineProps<{
  event: DeepReadonly<Event>;
  day: number;
  allCategories: Array<string>;
  from: "schedule" | "program" | undefined;
}>();

const event = computed(() => props.event);

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  return renderMarkdown(event.value.description);
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
            params: props.from === 'schedule' ? { dayIndex: props.day + 1 } : {},
            query: toFilterQueryParams(filterCriteria),
          }"
        >
          <IconButton
            class="lg:!hidden"
            icon="chevron-left"
            label="Back"
            :button-props="{
              'data-testid': 'event-details-back-button',
            }"
          />
          <IconButton
            class="!hidden lg:!block"
            icon="x-lg"
            label="Close"
            :button-props="{
              'data-testid': 'event-details-back-button',
            }"
          />
        </RouterLink>
        <IconButton
          class="hidden lg:block"
          label="Star"
          :icon="isStarred ? 'star-fill' : 'star'"
          :button-props="{
            'aria-pressed': isStarred,
            'data-testid': 'event-details-star-button',
          }"
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
            <span>Hosted by </span>
            <span v-for="(person, index) in event.people" :key="index">
              <RouterLink
                class="text-primary underline underline-offset-2 hover:decoration-2"
                data-testid="event-details-person-link"
                :to="{
                  name: props.from,
                  params: props.from === 'schedule' ? { dayIndex: props.day + 1 } : {},
                  query: toFilterQueryParams({ search: person }),
                }"
              >
                {{ person }}
              </RouterLink>
              <span v-if="index < event.people.length - 1">, </span>
            </span>
          </EventDetail>
          <EventDetail v-if="event.location" icon="geo-alt-fill" icon-label="Location">
            <RouterLink
              class="text-primary underline underline-offset-2 hover:decoration-2"
              data-testid="event-details-location-link"
              :to="{
                name: props.from,
                params: props.from === 'schedule' ? { dayIndex: props.day + 1 } : {},
                query: toFilterQueryParams({ search: event.location }),
              }"
            >
              {{ event.location }}
            </RouterLink>
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
            'aria-pressed': isStarred,
            'data-testid': 'event-details-star-button',
          }"
          @click="toggleStarred()"
        />
      </div>
      <TagBar
        class="mt-4"
        :category="event.category"
        :tags="event.tags"
        :all-categories="props.allCategories"
        :to="{
          name: props.from,
          params: props.from === 'schedule' ? { dayIndex: props.day + 1 } : {},
        }"
      />
      <Divider />
      <article id="document" :aria-labelledby="sectionHeadingId" class="my-4">
        <p v-if="event.summary">{{ event.summary }}</p>
        <div v-if="descriptionHtml" v-html="descriptionHtml" />
      </article>
      <div
        v-if="!event.summary && !descriptionHtml"
        class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8"
      >
        <span>No description</span>
      </div>
    </div>
  </section>
</template>
