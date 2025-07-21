<script setup lang="ts">
import Button from "primevue/button";

const props = defineProps<{
  icon: string;
  label: string;
  active?: boolean;
  disabled?: boolean;
}>();

defineEmits(["click"]);
</script>

<template>
  <Button
    :icon="`bi bi-${props.icon}`"
    pt:icon="!text-3xl"
    :variant="props.active == true ? undefined : 'text'"
    size="large"
    :aria-label="props.label"
    rounded
    :disabled="props.disabled ?? false"
    @click="$emit('click')"
  />
</template>
