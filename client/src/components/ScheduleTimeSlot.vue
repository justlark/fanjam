<script setup lang="ts">
import { useId, computed } from "vue";
import Divider from "primevue/divider";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { dateIsBetween } from "@/utils/time";
import CategoryLabel from "./CategoryLabel.vue";
import { RouterLink } from "vue-router";
import useStarredEvents from "@/composables/useStarredEvents";

const filterCriteria = useFilterQuery();

const focusedEventId = defineModel<string | undefined>("focused");

const props = defineProps<{
  localizedTime: string;
  events: Array<Event>;
  allCategories: Array<string>;
}>();

const starredEvents = useStarredEvents();

const sectionHeadingId = useId();

const isCurrentTimeSlot = computed(() => {
  if (props.events.length === 0) return false;

  const firstEventStartTime = props.events[0].startTime;
  const lastEvent = props.events[props.events.length - 1];
  const lastEventEndTime = lastEvent.endTime ?? lastEvent.startTime;
  const now = new Date();

  return dateIsBetween(now, firstEventStartTime, lastEventEndTime);
});

const isStarred = (eventId: string) => starredEvents.value && starredEvents.value.includes(eventId);
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex items-center gap-3">
      <h2
        :id="sectionHeadingId"
        :class="{
          'text-xl': true,
          'font-bold': isCurrentTimeSlot,
        }"
      >
        {{ props.localizedTime }}
      </h2>
      <small v-if="isCurrentTimeSlot" class="text-muted-color">now</small>
    </div>
    <Divider
      :class="{
        '!mt-1': true,
        'before:!border-primary': isCurrentTimeSlot,
        'before:!border-1': isCurrentTimeSlot,
      }"
    />
    <ul class="flex flex-wrap gap-3">
      <li v-for="event in props.events" :key="event.id">
        <RouterLink
          class="hidden lg:inline"
          :to="{
            name: 'event',
            params: { eventId: event.id },
            query: toFilterQueryParams(filterCriteria),
          }"
          :aria-label="isStarred(event.id) ? `Starred: ${event.name}` : event.name"
        >
          <CategoryLabel
            :title="event.name"
            :icon="isStarred(event.id) ? 'star-fill' : undefined"
            display="active"
            :category="event.category"
            :all-categories="props.allCategories"
          />
        </RouterLink>
        <button @click="focusedEventId = event.id" class="lg:hidden cursor-pointer">
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
