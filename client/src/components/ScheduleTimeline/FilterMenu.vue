<script setup lang="ts">
import CategoryLabel from "@/components/system/CategoryLabel.vue";

const props = defineProps<{
  allCategories: Array<string>;
}>();
</script>

<template>
  <div>
    <div class="flex flex-col gap-2">
      <span>Categories</span>
      <ul class="flex flex-wrap gap-3">
        <li v-for="(category, index) in props.allCategories" :key="index">
          <CategoryLabel :title="category" :all-categories="props.allCategories" :inactive="true" />
        </li>
      </ul>
    </div>
  </div>
</template>
