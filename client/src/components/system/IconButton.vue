<script setup lang="ts">
import Button from "primevue/button";

const props = defineProps<{
  icon: string;
  label: string;
  active?: boolean;
  disabled?: boolean;
  badge?: boolean;
  buttonProps?: Record<string, unknown>;
}>();

defineEmits(["click"]);
</script>

<template>
  <div>
    <span class="relative">
      <Button
        :icon="`bi bi-${props.icon}`"
        pt:icon="!text-3xl"
        :variant="props.active == true ? undefined : 'text'"
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
