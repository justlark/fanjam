<script setup lang="ts">
import { computed, useId } from "vue";
import { useRoute } from "vue-router";
import Divider from "primevue/divider";
import { type Event } from "@/utils/api";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import { QueryParam } from "@/utils/query";
import { RouterLink } from "vue-router";

const route = useRoute();

const eventIdAllowList = defineModel<Array<string>>("ids");

const props = defineProps<{
  localizedTime: string;
  events: Array<Event>;
  allCategories: Array<string>;
}>();

const eventIdAllowSet = computed(() =>
  eventIdAllowList.value !== undefined ? new Set(eventIdAllowList.value) : undefined,
);

const filteredEvents = computed(() => {
  return props.events.filter((event) => eventIdAllowSet.value?.has(event.id) ?? true);
});

const sectionHeadingId = useId();
</script>

<template>
  <section :aria-labelledby="sectionHeadingId" v-if="filteredEvents.length > 0">
    <h2 :id="sectionHeadingId" class="text-xl">{{ props.localizedTime }}</h2>
    <Divider pt:root="!mt-1" />
    <ul class="flex flex-wrap gap-3">
      <li v-for="event in filteredEvents" :key="event.id">
        <RouterLink
          :to="{
            name: 'event',
            params: { eventId: event.id },
            query: {
              [QueryParam.categories]: route.query[QueryParam.categories],
              [QueryParam.tags]: route.query[QueryParam.tags],
            },
          }"
        >
          <CategoryLabel
            :title="event.name"
            :category="event.category"
            :all-categories="props.allCategories"
          />
        </RouterLink>
      </li>
    </ul>
  </section>
</template>
