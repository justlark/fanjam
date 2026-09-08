<script setup lang="ts">
const props = defineProps<{
  icon: string;
  label?: string;
}>();
</script>

<template>
  <span>
    <i :class="`bi bi-${props.icon}`" aria-hidden="true"></i>
    <span v-if="props.label" class="sr-only">{{ props.label }}</span>
  </span>
</template>
