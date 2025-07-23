<script setup lang="ts">
import { type FilterCriteria } from "@/composables/useFilterQuery";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

const props = defineProps<{
  criteria: FilterCriteria;
  allCategories: Array<string>;
}>();
</script>

<template>
  <div class="flex flex-col lg:flex-row flex-wrap gap-6">
    <div class="flex flex-col gap-2 lg:basis-1/2">
      <span v-if="props.criteria.categories.length > 0 || props.criteria.tags.length > 0">
        Only showing events in:
      </span>
      <span class="ms-2 flex flex-wrap gap-3">
        <span v-for="(category, index) in props.criteria.categories" :key="index">
          <CategoryLabel :title="category" :all-categories="props.allCategories" display="active" />
        </span>
      </span>
    </div>
    <div class="flex flex-col gap-2">
      <span v-if="props.criteria.categories.length > 0 && props.criteria.tags.length > 0">
        And:
      </span>
      <span class="ms-2 flex flex-wrap gap-3">
        <span v-for="(tag, index) in props.criteria.tags" :key="index">
          <CategoryLabel :title="tag" display="active" />
        </span>
      </span>
    </div>
  </div>
</template>
