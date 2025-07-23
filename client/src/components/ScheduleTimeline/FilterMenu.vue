<script setup lang="ts">
import { ref, useId } from "vue";
import useFilterQuery from "@/composables/useFilterQuery";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import ToggleSwitch from "primevue/toggleswitch";

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

const hidePastEventsToggleId = useId();
</script>

<template>
  <div class="flex flex-col gap-6">
    <span class="flex items-center gap-4">
      <ToggleSwitch :id="hidePastEventsToggleId" v-model="criteria.hidePastEvents" />
      <label :for="hidePastEventsToggleId">Hide past events</label>
    </span>
    <div class="flex flex-col md:flex-row gap-x-12 gap-y-6">
      <div class="flex flex-col gap-2">
        <span>Only show these categories:</span>
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
      <div class="flex md:hidden justify-center items-center">
        <div class="border-b w-8 mr-4"></div>
        <span>and</span>
        <div class="border-b w-8 ml-4"></div>
      </div>
      <div class="hidden md:flex flex-col justify-center items-center">
        <div class="border-l h-4 mb-2"></div>
        <span>and</span>
        <div class="border-l h-4 mt-2"></div>
      </div>
      <div class="flex flex-col gap-2">
        <span>Only show these tags:</span>
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
