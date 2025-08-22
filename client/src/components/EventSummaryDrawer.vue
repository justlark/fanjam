<script setup lang="ts">
import { type Event } from "@/utils/api";
import TagBar from "./TagBar.vue";
import Drawer from "primevue/drawer";

const isVisible = defineModel<boolean>("visible", {
  required: true,
});

const props = defineProps<{
  event?: Event;
  day: number;
  allCategories: Array<string>;
}>();
</script>

<template>
  <Drawer
    class="!h-[15rem]"
    v-model:visible="isVisible"
    :dismissable="false"
    :modal="false"
    position="bottom"
  >
    <template #header>
      <TagBar
        v-if="props.event"
        :day="props.day"
        :category="props.event.category"
        :tags="props.event.tags"
        :all-categories="props.allCategories"
      />
      <!-- The header formatting breaks if there's no element here. -->
      <span></span>
    </template>
    <span v-if="props.event?.summary">{{ props.event.summary }}</span>
  </Drawer>
</template>
