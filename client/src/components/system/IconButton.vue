<script setup lang="ts">
import { computed } from "vue";
import Button from "primevue/button";

const props = defineProps<{
  icon: string;
  label: string;
  active?: boolean;
  disabled?: boolean;
  badge?: boolean;
  inactiveVariant?: "outlined" | "empty" | "filled";
  size?: "xs" | "sm" | "md" | "lg";
  buttonProps?: Record<string, unknown>;
}>();

defineEmits(["click"]);

const sizeClass = computed(() => {
  switch (props.size) {
    case "xs":
      return "text-lg";
    case "sm":
      return "text-xl";
    case "md":
      return "text-2xl";
    case "lg":
      return "text-3xl";
    default:
      return "text-3xl";
  }
});

const variant = computed(() => {
  if (props.active || props.inactiveVariant === "filled") {
    return undefined;
  }

  if (props.inactiveVariant === "outlined") {
    return "outlined";
  }

  return "text";
});
</script>

<template>
  <div>
    <span class="relative">
      <Button
        :icon="`bi bi-${props.icon}`"
        :pt:icon="`!${sizeClass}`"
        :variant="variant"
        :aria-pressed="props.active ?? false"
        size="large"
        :aria-label="props.label"
        rounded
        :disabled="props.disabled ?? false"
        @click="$emit('click')"
        v-bind="props.buttonProps"
      />
      <span
        v-if="props.badge && !props.active"
        class="absolute -top-2 right-3 border-4 border-red-400 dark:border-red-300 current rounded-full outline-3 outline-surface-50 dark:outline-surface-900"
      >
      </span>
    </span>
  </div>
</template>
