<script setup lang="ts">
import { RouterLink } from "vue-router";
import CategoryLabel from "./CategoryLabel.vue";
import { toFilterQueryParams } from "@/composables/useFilterQuery";

const props = defineProps<{
  day: number;
  category?: string;
  tags: Array<string>;
  allCategories: Array<string>;
}>();
</script>

<template>
  <div v-if="props.category || props.tags.length > 0" class="flex flex-wrap gap-3">
    <RouterLink
      v-if="props.category"
      :to="{
        name: 'schedule',
        params: { dayIndex: props.day },
        query: toFilterQueryParams({
          categories: [props.category],
        }),
      }"
    >
      <CategoryLabel
        :title="props.category"
        :all-categories="props.allCategories"
        display="active"
      />
    </RouterLink>
    <RouterLink
      v-for="tag in props.tags"
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
</template>
