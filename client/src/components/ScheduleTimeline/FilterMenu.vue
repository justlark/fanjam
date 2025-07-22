<script setup lang="ts">
import { ref, watchEffect } from "vue";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

export interface FilterCriteria {
  categories: Array<string>;
  tags: Array<string>;
}

const criteria = defineModel<FilterCriteria>("criteria", {
  default: {
    categories: [],
    tags: [],
  },
});

const filtered = defineModel<boolean>("filtered", {
  default: false,
});

const props = defineProps<{
  categories: Array<string>;
  tags: Array<string>;
}>();

const selectedCategories = ref<Set<string>>(new Set(criteria.value.categories));
const selectedTags = ref<Set<string>>(new Set(criteria.value.tags));

const toggleCategory = (category: string) => {
  if (selectedCategories.value.has(category)) {
    selectedCategories.value.delete(category);
  } else {
    selectedCategories.value.add(category);
  }

  criteria.value.categories = Array.from(selectedCategories.value);
};

const toggleTag = (tag: string) => {
  if (selectedTags.value.has(tag)) {
    selectedTags.value.delete(tag);
  } else {
    selectedTags.value.add(tag);
  }

  criteria.value.tags = Array.from(selectedTags.value);
};

const isCategorySelected = (category: string) => selectedCategories.value.has(category);
const isTagSelected = (tag: string) => selectedTags.value.has(tag);

watchEffect(() => {
  filtered.value = criteria.value.categories.length > 0 || criteria.value.tags.length > 0;
});
</script>

<template>
  <div>
    <div class="flex flex-col lg:flex-row gap-y-4 gap-x-12">
      <div class="flex flex-col gap-2 lg:basis-1/2">
        <span>Filter by Category</span>
        <ul class="flex flex-wrap gap-3">
          <li v-for="(category, index) in props.categories" :key="index">
            <CategoryLabel
              :title="category"
              :all-categories="props.categories"
              :inactive="!isCategorySelected(category)"
            >
              <button class="cursor-pointer" @click="toggleCategory(category)">
                {{ category }}
              </button>
            </CategoryLabel>
          </li>
        </ul>
      </div>
      <div class="flex flex-col gap-2 lg:basis-1/2">
        <span>Filter by Tags</span>
        <ul class="flex flex-wrap gap-3">
          <li v-for="(tag, index) in props.tags" :key="index">
            <CategoryLabel :title="tag" :inactive="!isTagSelected(tag)">
              <button class="cursor-pointer" @click="toggleTag(tag)">{{ tag }}</button>
            </CategoryLabel>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
