<script setup lang="ts">
import { RouterLink, type RouteLocationAsRelativeGeneric } from "vue-router";
import CategoryLabel from "./CategoryLabel.vue";
import { toFilterQueryParams } from "@/composables/useFilterQuery";

const props = defineProps<{
  size?: "xs" | "sm" | "md" | "lg";
  category?: string;
  tags: ReadonlyArray<string>;
  allCategories: ReadonlyArray<string>;
  to?: RouteLocationAsRelativeGeneric;
}>();
</script>

<template>
  <div v-if="props.category || props.tags.length > 0" class="flex flex-wrap gap-3">
    <RouterLink
      v-if="props.category"
      :to="{
        ...props.to,
        query: toFilterQueryParams({
          categories: [props.category],
        }),
      }"
      data-testid="tagbar-category-link"
    >
      <CategoryLabel
        :title="props.category"
        :size="props.size"
        :all-categories="props.allCategories"
        display="active"
      />
    </RouterLink>
    <RouterLink
      v-for="tag in props.tags"
      :key="tag"
      :to="{
        ...props.to,
        query: toFilterQueryParams({
          tags: [tag],
        }),
      }"
      data-testid="tagbar-tag-link"
    >
      <CategoryLabel :title="tag" :size="props.size" display="active" />
    </RouterLink>
  </div>
</template>
