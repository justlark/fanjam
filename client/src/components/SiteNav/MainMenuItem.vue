<script setup lang="ts">
import { useRoute } from "vue-router";

import Button from "primevue/button";
import { RouterLink } from "vue-router";

const props = defineProps<{
  to: string;
  activeRoutes: Array<string>;
  label: string;
  icon: string;
}>();

const route = useRoute();
</script>

<template>
  <Button
    pt:root="!justify-start"
    :as="RouterLink"
    :to="{ name: props.to }"
    :icon="props.icon"
    :label="props.label"
    :variant="activeRoutes.includes(route.name) ? undefined : 'outlined'"
    size="large"
  />
</template>
