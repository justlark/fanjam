<script setup lang="ts">
import { useId } from "vue";
import Divider from "primevue/divider";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import { RouterLink } from "vue-router";

const filterCriteria = useFilterQuery();

const props = defineProps<{
  localizedTime: string;
  events: Array<Event>;
  allCategories: Array<string>;
}>();

const sectionHeadingId = useId();
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <h2 :id="sectionHeadingId" class="text-xl">{{ props.localizedTime }}</h2>
    <Divider pt:root="!mt-1" />
    <ul class="flex flex-wrap gap-3">
      <li v-for="event in props.events" :key="event.id">
        <RouterLink
          :to="{
            name: 'event',
            params: { eventId: event.id },
            query: toFilterQueryParams(filterCriteria),
          }"
        >
          <CategoryLabel
            :title="event.name"
            display="active"
            :category="event.category"
            :all-categories="props.allCategories"
          />
        </RouterLink>
      </li>
    </ul>
  </section>
</template>
