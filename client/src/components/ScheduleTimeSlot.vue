<script setup lang="ts">
import Divider from "primevue/divider";
import CategoryLabel from "@/components/CategoryLabel.vue";

export interface EventSummary {
  title: string;
  category: string;
}

const props = defineProps<{
  time: string;
  events: Array<EventSummary>;
}>();
</script>

<template>
  <div>
    <h2 class="text-xl">{{ props.time }}</h2>
    <Divider pt:root="!mt-1" />
    <div class="flex flex-wrap gap-3">
      <CategoryLabel
        v-for="(event, index) in props.events"
        :key="index"
        :title="event.title"
        :category="event.category"
      />
    </div>
  </div>
</template>
