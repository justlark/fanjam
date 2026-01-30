<script setup lang="ts">
import { useId, type DeepReadonly } from "vue";
import Divider from "primevue/divider";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import CategoryLabel from "./CategoryLabel.vue";
import { RouterLink } from "vue-router";
import useStarredEvents from "@/composables/useStarredEvents";

const filterCriteria = useFilterQuery();

const focusedEventId = defineModel<string | undefined>("focused");

const props = defineProps<{
  localizedTime: string;
  events: Array<DeepReadonly<Event>>;
  isCurrentTimeSlot: boolean;
  allCategories: Array<string>;
  viewType: "daily" | "all";
}>();

const starredEvents = useStarredEvents();

const sectionHeadingId = useId();

const isStarred = (eventId: string) => starredEvents.value.has(eventId);
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex items-center gap-3">
      <h2
        :id="sectionHeadingId"
        :class="{
          'text-xl': true,
          'font-bold': props.isCurrentTimeSlot,
        }"
      >
        {{ props.localizedTime }}
      </h2>
      <small v-if="props.isCurrentTimeSlot" class="text-muted-color">now</small>
    </div>
    <Divider
      :class="{
        '!mt-1': true,
        'before:!border-primary': props.isCurrentTimeSlot,
        'before:!border-1': props.isCurrentTimeSlot,
      }"
    />
    <ul class="flex flex-wrap gap-3">
      <li v-for="event in props.events" :key="event.id" :id="`event-${event.id}`">
        <RouterLink
          class="hidden lg:inline"
          :to="{
            name: 'event',
            params: { eventId: event.id },
            query: toFilterQueryParams(filterCriteria),
            state: { fromViewType: viewType },
          }"
          :aria-label="isStarred(event.id) ? `Starred: ${event.name}` : event.name"
          data-testid="schedule-event-link"
        >
          <CategoryLabel
            :title="event.name"
            :icon="isStarred(event.id) ? 'star-fill' : undefined"
            display="active"
            :category="event.category"
            :all-categories="props.allCategories"
          />
        </RouterLink>
        <button
          @click="focusedEventId = event.id"
          class="lg:hidden cursor-pointer text-left"
          :aria-label="isStarred(event.id) ? `Starred: ${event.name}` : event.name"
          data-testid="schedule-event-link"
        >
          <CategoryLabel
            :title="event.name"
            :icon="isStarred(event.id) ? 'star-fill' : undefined"
            display="active"
            :category="event.category"
            :all-categories="props.allCategories"
          />
        </button>
      </li>
    </ul>
  </section>
</template>
