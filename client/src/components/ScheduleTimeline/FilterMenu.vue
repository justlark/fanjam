<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { useRouter, useRoute } from "vue-router";
import { QueryParam, rawQueryParamToArray } from "@/utils/query";
import CategoryLabel from "@/components/system/CategoryLabel.vue";

const route = useRoute();
const router = useRouter();

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

watchEffect(() => {
  const categorieFromQuery = rawQueryParamToArray(route, QueryParam.categories);
  if (categorieFromQuery !== undefined) {
    criteria.value.categories = categorieFromQuery;
  }

  const tagsFromQuery = rawQueryParamToArray(route, QueryParam.tags);
  if (tagsFromQuery !== undefined) {
    criteria.value.tags = tagsFromQuery;
  }
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

watchEffect(async () => {
  await router.replace({
    query: {
      category:
        criteria.value.categories.length > 0 ? criteria.value.categories.join(",") : undefined,
      tag: criteria.value.tags.length > 0 ? criteria.value.tags.join(",") : undefined,
    },
  });
});

watchEffect(() => {
  filtered.value = criteria.value.categories.length > 0 || criteria.value.tags.length > 0;
});
</script>

<template>
  <div>
    <div class="flex flex-col lg:flex-row flex-wrap gap-6">
      <div class="flex flex-col gap-2 lg:basis-1/2">
        <span>Filter by Category</span>
        <ul class="flex flex-wrap gap-3">
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
              />
            </button>
          </li>
        </ul>
      </div>
      <div class="flex flex-col gap-2">
        <span>Filter by Tags</span>
        <ul class="flex flex-wrap gap-3">
          <li v-for="(tag, index) in props.tags" :key="index">
            <button
              class="cursor-pointer"
              @click="toggleTag(tag)"
              :aria-pressed="isTagSelected(tag)"
            >
              <CategoryLabel :title="tag" :inactive="!isTagSelected(tag)" />
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
