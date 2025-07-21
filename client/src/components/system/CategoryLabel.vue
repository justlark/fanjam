<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { newBgColor, newFgColor, newOutlineColor } from "@/utils/colors";

const props = defineProps<{
  title: string;
  inactive?: boolean;
  category?: string;
  allCategories: Array<string>;
}>();

const fgColor700 = computed(
  () => `!${newFgColor(props.category ?? props.title, props.allCategories, 700)}`,
);
const fgColor200NotHoverDark = computed(
  () => `not-hover:dark:!${newFgColor(props.category ?? props.title, props.allCategories, 200)}`,
);
const bgColor100 = computed(
  () => `!${newBgColor(props.category ?? props.title, props.allCategories, 100)}`,
);
const outlineColor700 = computed(
  () => `!${newOutlineColor(props.category ?? props.title, props.allCategories, 700)}`,
);
const outlineColor200Dark = computed(
  () => `dark:!${newOutlineColor(props.category ?? props.title, props.allCategories, 200)}`,
);
</script>

<template>
  <Tag
    :value="props.title"
    :class="{
      [fgColor700]: true,
      [bgColor100]: true,
      [fgColor200NotHoverDark]: props.inactive,
      outline: props.inactive,
      'not-hover:!bg-transparent': props.inactive,
      [outlineColor700]: props.inactive,
      [outlineColor200Dark]: props.inactive,
    }"
  >
    <slot />
  </Tag>
</template>
