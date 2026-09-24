<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { newBgColor, newFgColor, newOutlineColor } from "@/utils/colors";

const props = defineProps<{
  title: string;
  display: "active" | "inactive" | "hover";
  icon?: string;
  size?: "xs" | "sm" | "md" | "lg";
  category?: string;
  allCategories?: ReadonlyArray<string>;
}>();

const allCategories = computed(() => props.allCategories ?? []);
const isCategories = computed(() => allCategories.value.includes(props.category ?? props.title));

const textSize = computed(() => {
  switch (props.size) {
    case "xs":
      return "text-xs";
    case "sm":
      return "text-sm";
    case "md":
      return "text-base";
    case "lg":
      return "text-lg";
    default:
      return "text-base";
  }
});

const fg = (value: number) => newFgColor(props.category ?? props.title, allCategories.value, value);
const bg = (value: number) => newBgColor(props.category ?? props.title, allCategories.value, value);
const outline = (value: number) =>
  newOutlineColor(props.category ?? props.title, allCategories.value, value);

// The "hover" display is styled like "inactive", but looks like "active" on
// hover.
const categoryStyles = computed(() => [
  `!${textSize.value}`,
  `!${fg(700)}`,
  ...(props.display == "active" ? [`!${bg(100)}`] : []),
  ...(props.display == "hover" || props.display == "inactive"
    ? [
        "outline",
        `!${outline(700)}`,
        `dark:!${outline(200)}`,
        "!bg-transparent",
        `dark:!${fg(200)}`,
      ]
    : []),
  ...(props.display == "hover" ? [`hover:!${bg(100)}`, `hover:dark:!${fg(700)}`] : []),
]);

const standaloneStyles = computed(() => [
  `!${textSize.value}`,
  "!text-surface-600",
  "dark:!text-surface-300",
  ...(props.display == "active" ? ["!bg-surface-200", "dark:!bg-surface-700"] : []),
  ...(props.display == "hover" || props.display == "inactive"
    ? ["outline", "!outline-surface-400", "dark:!outline-surface-700", "!bg-transparent"]
    : []),
  ...(props.display == "hover" ? ["hover:!bg-surface-200", "hover:dark:!bg-surface-700"] : []),
]);
</script>

<template>
  <Tag
    :icon="props.icon ? `bi bi-${props.icon}` : undefined"
    :value="props.title"
    :class="isCategories ? categoryStyles : standaloneStyles"
    :pt:icon:class="`!${textSize} flex items-center justify-center mr-1`"
  />
</template>
