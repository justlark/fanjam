<script setup lang="ts">
import Button from "primevue/button";
import { RouterLink } from "vue-router";

const props = defineProps<{
  to: string;
  label: string;
  icon: string;
}>();
</script>

<template>
  <Button
    pt:root="!justify-start"
    :as="RouterLink"
    :to="props.to"
    :icon="props.icon"
    :label="props.label"
    variant="outlined"
    size="large"
  />
</template>
