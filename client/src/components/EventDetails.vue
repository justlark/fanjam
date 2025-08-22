<script setup lang="ts">
import { computed, watchEffect, ref, useId } from "vue";
import { useRouter, RouterLink } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useStarredEvents from "@/composables/useStarredEvents";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import * as commonmark from "commonmark";
import CategoryLabel from "./CategoryLabel.vue";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";

const router = useRouter();
const filterCriteria = useFilterQuery();
const starredEvents = useStarredEvents();
const datetimeFormats = useDatetimeFormats();

const props = defineProps<{
  event: Event;
  day: number;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  const parsed = mdReader.parse(event.value.description);
  return mdWriter.render(parsed);
});

const back = async () => {
  await router.push({
    name: "schedule",
    params: { dayIndex: props.day },
    query: toFilterQueryParams(filterCriteria),
  });
};

const starredEventsSet = ref<Set<string>>(new Set(starredEvents.value));

watchEffect(() => {
  starredEvents.value = Array.from(starredEventsSet.value);
});

const isStarred = computed(() => starredEventsSet.value.has(event.value.id));

const toggleStarred = () => {
  if (isStarred.value) {
    starredEventsSet.value.delete(event.value.id);
  } else {
    starredEventsSet.value.add(event.value.id);
  }
};

const sectionHeadingId = useId();
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <span class="flex items-center">
        <IconButton class="lg:!hidden" icon="chevron-left" label="Back" @click="back()" />
        <IconButton class="!hidden lg:!block" icon="x-lg" label="Close" @click="back()" />
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
      <div v-if="event.category || event.tags.length > 0" class="mt-4 flex flex-wrap gap-3">
        <RouterLink
          v-if="event.category"
          :to="{
            name: 'schedule',
            params: { dayIndex: props.day },
            query: toFilterQueryParams({
              categories: [event.category],
            }),
          }"
        >
          <CategoryLabel
            :title="event.category"
            :all-categories="props.allCategories"
            display="active"
          />
        </RouterLink>
        <RouterLink
          v-for="tag in event.tags"
          :key="tag"
          :to="{
            name: 'schedule',
            params: { dayIndex: props.day },
            query: toFilterQueryParams({
              tags: [tag],
            }),
          }"
        >
          <CategoryLabel :title="tag" display="active" />
        </RouterLink>
      </div>
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
