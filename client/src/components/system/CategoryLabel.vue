<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { newBgColor, newFgColor, newOutlineColor } from "@/utils/colors";

const props = defineProps<{
  title: string;
  inactive?: boolean;
  category?: string;
  allCategories?: Array<string>;
}>();

const allCategories = computed(() => props.allCategories ?? []);
const isCategories = computed(() => allCategories.value.includes(props.category ?? props.title));

const fg = (value: number) => newFgColor(props.category ?? props.title, allCategories.value, value);
const bg = (value: number) => newBgColor(props.category ?? props.title, allCategories.value, value);
const outline = (value: number) =>
  newOutlineColor(props.category ?? props.title, allCategories.value, value);

const categoryStyles = computed(() => [
  `!${fg(700)}`,
  `!${bg(100)}`,
  ...(props.inactive
    ? [
        "outline",
        "not-hover:!bg-transparent",
        `not-hover:dark:!${fg(200)}`,
        `!${outline(700)}`,
        `dark:!${outline(200)}`,
      ]
    : []),
]);

const standaloneStyles = computed(() => [
  "!text-slate-600",
  "!bg-slate-200",
  "dark:!text-zinc-300",
  "dark:!bg-zinc-700",
  ...(props.inactive
    ? ["outline", "!outline-slate-400", "dark:!outline-zinc-700", , "not-hover:!bg-transparent"]
    : []),
]);
</script>

<template>
  <Tag :value="props.title" :class="isCategories ? categoryStyles : standaloneStyles" />
</template>
