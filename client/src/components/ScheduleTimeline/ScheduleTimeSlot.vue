<script setup lang="ts">
import Divider from "primevue/divider";
import CategoryLabel from "@/components/CategoryLabel.vue";
import { RouterLink } from "vue-router";

export interface EventSummary {
  id: string;
  title: string;
  category: string;
}

const props = defineProps<{
  localizedTime: string;
  events: Array<EventSummary>;
  allCategories: Array<string>;
}>();
</script>

<template>
  <section>
    <h2 class="text-xl">{{ props.localizedTime }}</h2>
    <Divider pt:root="!mt-1" />
    <div class="flex flex-wrap gap-3">
      <RouterLink
        v-for="event in props.events"
        :key="event.id"
        :to="{ name: 'event', params: { eventId: event.id } }"
      >
        <CategoryLabel
          :title="event.title"
          :category="event.category"
          :all-categories="props.allCategories"
        />
      </RouterLink>
    </div>
  </section>
</template>
