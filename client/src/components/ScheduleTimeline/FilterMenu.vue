<script setup lang="ts">
import { ref } from "vue";
import useFilterQuery from "@/composables/useFilterQuery";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

const criteria = useFilterQuery();

const props = defineProps<{
  categories: Array<string>;
  tags: Array<string>;
}>();

const selectedCategories = ref<Set<string>>(new Set(criteria.categories));
const selectedTags = ref<Set<string>>(new Set(criteria.tags));

const toggleCategory = (category: string) => {
  if (selectedCategories.value.has(category)) {
    selectedCategories.value.delete(category);
  } else {
    selectedCategories.value.add(category);
  }

  criteria.categories = Array.from(selectedCategories.value);
};

const toggleTag = (tag: string) => {
  if (selectedTags.value.has(tag)) {
    selectedTags.value.delete(tag);
  } else {
    selectedTags.value.add(tag);
  }

  criteria.tags = Array.from(selectedTags.value);
};

const isCategorySelected = (category: string) => selectedCategories.value.has(category);
const isTagSelected = (tag: string) => selectedTags.value.has(tag);
</script>

<template>
  <div>
    <div class="flex flex-col lg:flex-row flex-wrap gap-6">
      <div class="flex flex-col gap-2 lg:basis-1/2">
        <span>Filter by Category</span>
        <ul class="ms-2 flex flex-wrap gap-3">
          <li v-for="(category, index) in props.categories" :key="index">
            <button
              class="cursor-pointer"
              @click="toggleCategory(category)"
              :aria-pressed="isCategorySelected(category)"
            >
              <CategoryLabel
                :title="category"
                :all-categories="props.categories"
                :inactive="!isCategorySelected(category)"
                :display="isCategorySelected(category) ? 'active' : 'hover'"
              />
            </button>
          </li>
        </ul>
      </div>
      <div class="flex flex-col gap-2">
        <span>Filter by Tags</span>
        <ul class="ms-2 flex flex-wrap gap-3">
          <li v-for="(tag, index) in props.tags" :key="index">
            <button
              class="cursor-pointer"
              @click="toggleTag(tag)"
              :aria-pressed="isTagSelected(tag)"
            >
              <CategoryLabel :title="tag" :display="isTagSelected(tag) ? 'active' : 'hover'" />
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
