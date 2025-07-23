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
</script>

<template>
  <div class="flex flex-col lg:flex-row flex-wrap gap-6">
    <div v-if="hasCategories" class="flex flex-col gap-2 lg:basis-1/2">
      <span>Only showing events in:</span>
      <span v-if="hasCategories" class="ms-2 flex flex-wrap gap-3">
        <span v-for="(category, index) in props.criteria.categories" :key="index">
          <CategoryLabel :title="category" :all-categories="props.allCategories" display="active" />
        </span>
      </span>
    </div>
    <div v-if="hasTags" class="flex flex-col gap-2">
      <span v-if="hasCategories">And:</span>
      <span v-else>Only showing events in:</span>
      <span class="ms-2 flex flex-wrap gap-3">
        <span v-for="(tag, index) in props.criteria.tags" :key="index">
          <CategoryLabel :title="tag" display="active" />
        </span>
      </span>
    </div>
  </div>
</template>
