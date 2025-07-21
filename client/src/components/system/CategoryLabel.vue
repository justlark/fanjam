<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { newBgColor, newFgColor } from "@/utils/colors";

const props = defineProps<{
  title: string;
  category?: string;
  allCategories: Array<string>;
}>();

const bgColor = computed(() => newBgColor(props.category ?? props.title, props.allCategories, 100));
const fgColor = computed(() => newFgColor(props.category ?? props.title, props.allCategories, 700));
</script>

<template>
  <Tag :value="props.title" :class="`!${fgColor} !${bgColor}`" />
</template>
