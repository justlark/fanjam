<script setup lang="ts">
import { ref, watchEffect } from "vue";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

export interface FilterCriteria {
  categories: Array<string>;
}

const criteria = defineModel<FilterCriteria>("criteria", {
  default: {
    categories: [],
  },
});

const filtered = defineModel<boolean>("filtered", {
  default: false,
});

const props = defineProps<{
  allCategories: Array<string>;
}>();

const selectedCategories = ref<Set<string>>(new Set(criteria.value.categories));

const toggleCategory = (category: string) => {
  if (selectedCategories.value.has(category)) {
    selectedCategories.value.delete(category);
  } else {
    selectedCategories.value.add(category);
  }

  criteria.value.categories = Array.from(selectedCategories.value);
};

const isCategorySelected = (category: string) => selectedCategories.value.has(category);

watchEffect(() => {
  filtered.value = criteria.value.categories.length > 0;
});
</script>

<template>
  <div>
    <div class="flex flex-col gap-2">
      <span>Filter by Category</span>
      <ul class="flex flex-wrap gap-3">
        <li v-for="(category, index) in props.allCategories" :key="index">
          <CategoryLabel
            :title="category"
            :all-categories="props.allCategories"
            :inactive="!isCategorySelected(category)"
          >
            <button class="cursor-pointer" @click="toggleCategory(category)">{{ category }}</button>
          </CategoryLabel>
        </li>
      </ul>
    </div>
  </div>
</template>
