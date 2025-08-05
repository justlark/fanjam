<script setup lang="ts">
import { computed } from "vue";
import { type FilterCriteria } from "@/composables/useFilterQuery";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const props = defineProps<{
  allCategories: Array<string>;
}>();

const criteria = defineModel<FilterCriteria>("criteria", { required: true });

const onlyStarred = computed(() => criteria.value.hideNotStarred);
const hasCategories = computed(() => criteria.value.categories.length > 0);
const hasTags = computed(() => criteria.value.tags.length > 0);
const hasSearch = computed(() => criteria.value.search.length > 0);
const hasMultipleCategories = computed(() => criteria.value.categories.length > 1);
const hasMultipleTags = computed(() => criteria.value.tags.length > 1);

const clearCategory = (category: string) => {
  const index = criteria.value.categories.indexOf(category);
  if (index !== -1) {
    criteria.value.categories.splice(index, 1);
  }
};

const clearTag = (tag: string) => {
  const index = criteria.value.tags.indexOf(tag);
  if (index !== -1) {
    criteria.value.tags.splice(index, 1);
  }
};
</script>

<template>
  <span>
    <span class="font-bold me-2 block md:inline">Only showing:</span>
    <span v-if="onlyStarred">
      <SimpleIcon icon="star-fill" label="Star" />
    </span>
    <span class="mx-2 italic" v-if="onlyStarred && (hasCategories || hasTags || hasSearch)"
      >and</span
    >
    <span v-if="hasMultipleCategories && (onlyStarred || hasTags || hasSearch)" class="me-1"
      >(</span
    >
    <template v-for="(category, index) in criteria.categories" :key="index">
      <span class="mx-2 italic" v-if="index != 0">or</span>
      <button @click="clearCategory(category)">
        <CategoryLabel
          class="my-1"
          size="xs"
          icon="x-lg"
          :title="category"
          :all-categories="props.allCategories"
          display="active"
        />
      </button>
    </template>
    <span v-if="hasMultipleCategories && (onlyStarred || hasTags || hasSearch)" class="ms-1"
      >)</span
    >
    <span class="mx-2 italic" v-if="hasCategories && hasTags">and</span>
    <span v-if="hasMultipleTags && (onlyStarred || hasCategories || hasSearch)" class="me-1"
      >(</span
    >
    <template v-for="(tag, index) in criteria.tags" :key="index">
      <span class="mx-2 italic" v-if="index != 0">or</span>
      <button @click="clearTag(tag)">
        <CategoryLabel class="my-1" size="xs" :title="tag" display="active" />
      </button>
    </template>
    <span v-if="hasMultipleTags && (onlyStarred || hasCategories || hasSearch)" class="ms-1"
      >)</span
    >
    <span class="mx-2 italic" v-if="(hasCategories || hasTags) && hasSearch">and</span>
    <span class="my-1 text-sm" v-if="hasSearch">"{{ criteria.search }}"</span>
  </span>
</template>
