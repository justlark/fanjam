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

const isInactive = props.inactive;

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
      [fgColor200NotHoverDark]: isInactive,
      outline: isInactive,
      'not-hover:!bg-transparent': isInactive,
      [outlineColor700]: isInactive,
      [outlineColor200Dark]: isInactive,
    }"
  >
    <slot />
  </Tag>
</template>
