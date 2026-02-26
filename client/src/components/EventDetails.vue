<script setup lang="ts">
import { computed, ref, useId, type DeepReadonly, onMounted } from "vue";
import { useRoute, useRouter, RouterLink } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useIsEventStarred from "@/composables/useIsEventStarred";
import { toFilterQueryParams } from "@/composables/useFilterQuery";
import { renderMarkdown } from "@/utils/markdown";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import TagBar from "./TagBar.vue";

const datetimeFormats = useDatetimeFormats();
const route = useRoute();
const router = useRouter();

const props = defineProps<{
  event: DeepReadonly<Event>;
  day: number;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);
const isStarred = useIsEventStarred(computed(() => event.value.id));

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  return renderMarkdown(event.value.description);
});

const sectionHeadingId = useId();

const fromViewType = ref<"daily" | "all">();

const back = async () => {
  if (!fromViewType.value) {
    return;
  }

  await router.push({
    name: "schedule",
    params: {
      dayIndex: fromViewType.value === "daily" ? props.day + 1 : "all",
    },
    query: route.query,
  });
};

onMounted(() => {
  fromViewType.value = history.state.fromViewType;
});
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <span class="flex items-center">
        <IconButton
          class="lg:!hidden"
          icon="chevron-left"
          label="Back"
          :button-props="{ 'data-testid': 'event-details-back-button' }"
          @click="back()"
        />
        <IconButton
          class="!hidden lg:!block"
          icon="x-lg"
          label="Close"
          :button-props="{ 'data-testid': 'event-details-back-button' }"
          @click="back()"
        />
        <IconButton
          class="hidden lg:block"
          label="Star"
          :icon="isStarred ? 'star-fill' : 'star'"
          :button-props="{
            'aria-pressed': isStarred,
            'data-testid': 'event-details-star-button',
          }"
          @click="isStarred = !isStarred"
        />
      </span>
      <h2 :id="sectionHeadingId" class="text-xl font-bold" data-testid="event-details-name">
        {{ event.name }}
      </h2>
    </div>
    <div class="px-6">
      <div class="flex justify-between items-end">
        <dl class="flex flex-col items-start gap-2">
          <EventDetail
            v-if="datetimeFormats && event.startTime"
            icon="clock"
            icon-label="Time"
            data-testid="event-details-time"
          >
            {{ localizeTimeSpan(datetimeFormats, event.startTime, event.endTime) }}
          </EventDetail>
          <EventDetail
            v-if="event.people.length > 0"
            icon="person-circle"
            icon-label="Hosts"
            data-testid="event-details-hosts"
          >
            <span>Hosted by </span>
            <span v-for="(person, index) in event.people" :key="index">
              <RouterLink
                class="text-link-sm"
                data-testid="event-details-person-link"
                :to="{
                  name: 'schedule',
                  params: { dayIndex: props.day + 1 },
                  query: toFilterQueryParams({ search: person }),
                }"
              >
                {{ person }}
              </RouterLink>
              <span v-if="index < event.people.length - 1">, </span>
            </span>
          </EventDetail>
          <EventDetail
            v-if="event.location"
            icon="geo-alt-fill"
            icon-label="Location"
            data-testid="event-details-location"
          >
            <RouterLink
              class="text-link-sm"
              data-testid="event-details-location-link"
              :to="{
                name: 'schedule',
                params: { dayIndex: props.day + 1 },
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
          :button-props="{
            'aria-pressed': isStarred,
            'data-testid': 'event-details-star-button',
          }"
          @click="isStarred = !isStarred"
        />
      </div>
      <TagBar
        class="mt-4"
        :category="event.category"
        :tags="event.tags"
        :all-categories="props.allCategories"
        :to="{
          name: 'schedule',
          params: {
            dayIndex: fromViewType === 'all' ? 'all' : props.day + 1,
          },
        }"
      />
      <Divider />
      <article
        id="document"
        :aria-labelledby="sectionHeadingId"
        class="my-4"
        data-testid="event-details-content"
      >
        <p v-if="event.summary" data-testid="event-details-summary">{{ event.summary }}</p>
        <div
          v-if="descriptionHtml"
          v-html="descriptionHtml"
          data-testid="event-details-description"
        />
      </article>
      <div
        v-if="!event.summary && !descriptionHtml"
        class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8"
        data-testid="event-details-no-description"
      >
        <span>No description</span>
      </div>
    </div>
  </section>
</template>
