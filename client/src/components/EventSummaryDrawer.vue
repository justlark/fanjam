<script setup lang="ts">
import { type Event } from "@/utils/api";
import TagBar from "./TagBar.vue";
import Drawer from "primevue/drawer";

const isVisible = defineModel<boolean>("visible", {
  required: true,
});

const props = defineProps<{
  event: Event;
  day: number;
  allCategories: Array<string>;
}>();
</script>

<template>
  <Drawer v-model:visible="isVisible" :dismissable="false" :modal="false" position="bottom">
    <template #header>
      <TagBar
        :day="props.day"
        :category="props.event.category"
        :tags="props.event.tags"
        :all-categories="props.allCategories"
      />
    </template>
    <span>This is a placeholder summary!</span>
  </Drawer>
</template>
