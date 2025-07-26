<script setup lang="ts">
import { computed } from "vue";
import { type FilterCriteria } from "@/composables/useFilterQuery";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

const props = defineProps<{
  criteria: FilterCriteria;
  allCategories: Array<string>;
}>();

const hasCategories = computed(() => props.criteria.categories.length > 0);
const hasTags = computed(() => props.criteria.tags.length > 0);
const hasSearch = computed(() => props.criteria.search.length > 0);
const hasMultipleCategories = computed(() => props.criteria.categories.length > 1);
const hasMultipleTags = computed(() => props.criteria.tags.length > 1);
</script>

<template>
  <span>
    <span class="font-bold me-2 block md:inline">Only showing:</span>
    <span v-if="hasMultipleCategories && (hasTags || hasSearch)" class="me-1">(</span>
    <template v-for="(category, index) in props.criteria.categories" :key="index">
      <span class="mx-2 italic" v-if="index != 0">or</span>
      <CategoryLabel
        class="my-1"
        size="xs"
        :title="category"
        :all-categories="props.allCategories"
        display="active"
      />
    </template>
    <span v-if="hasMultipleCategories && (hasTags || hasSearch)" class="ms-1">)</span>
    <span class="mx-2 italic" v-if="hasCategories && hasTags">and</span>
    <span v-if="hasMultipleTags && (hasCategories || hasSearch)" class="me-1">(</span>
    <template v-for="(tag, index) in props.criteria.tags" :key="index">
      <span class="mx-2 italic" v-if="index != 0">or</span>
      <CategoryLabel class="my-1" size="xs" :title="tag" display="active" />
    </template>
    <span v-if="hasMultipleTags && (hasCategories || hasSearch)" class="ms-1">)</span>
    <span class="mx-2 italic" v-if="(hasCategories || hasTags) && hasSearch">and</span>
    <span class="my-1 text-sm" v-if="hasSearch">"{{ props.criteria.search }}"</span>
  </span>
</template>
