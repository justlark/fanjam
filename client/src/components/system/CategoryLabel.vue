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
const hasColor = computed(() => allCategories.value.includes(props.category ?? props.title));

const fgColor = computed(() =>
  hasColor.value
    ? `!${newFgColor(props.category ?? props.title, allCategories.value, 700)}`
    : "!text-gray-700",
);
const bgColor = computed(() =>
  hasColor.value
    ? `!${newBgColor(props.category ?? props.title, allCategories.value, 100)}`
    : "!bg-gray-100",
);
const fgColorNotHoverDark = computed(() =>
  hasColor.value
    ? `not-hover:dark:!${newFgColor(props.category ?? props.title, allCategories.value, 200)}`
    : "not-hover:dark:!text-gray-400",
);
const outlineColorLight = computed(() =>
  hasColor.value
    ? `!${newOutlineColor(props.category ?? props.title, allCategories.value, 700)}`
    : "!outline-gray-700",
);
const outlineColorDark = computed(() =>
  hasColor.value
    ? `dark:!${newOutlineColor(props.category ?? props.title, allCategories.value, 200)}`
    : "dark:!outline-gray-400",
);
</script>

<template>
  <Tag
    :value="props.title"
    :class="{
      outline: props.inactive,
      'not-hover:!bg-transparent': props.inactive,
      [fgColor]: true,
      [bgColor]: true,
      [fgColorNotHoverDark]: props.inactive,
      [outlineColorLight]: props.inactive,
      [outlineColorDark]: props.inactive,
    }"
  >
    <slot />
  </Tag>
</template>
