<script setup lang="ts">
import Button from "primevue/button";

const props = defineProps<{
  icon: string;
  label: string;
}>();

defineEmits(["click"]);
</script>

<template>
  <Button
    :icon="`bi bi-${props.icon}`"
    pt:icon="!text-3xl"
    variant="text"
    size="large"
    :aria-label="props.label"
    rounded
    @click="$emit('click')"
  />
</template>
