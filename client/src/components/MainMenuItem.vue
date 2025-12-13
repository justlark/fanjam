<script setup lang="ts">
import { computed } from "vue";
import { useRoute, type RouteLocationNormalizedLoadedGeneric } from "vue-router";
import OverlayBadge from "primevue/overlaybadge";
import Button from "primevue/button";
import { RouterLink } from "vue-router";

const props = defineProps<{
  to: string;
  label: string;
  icon: string;
  isActive?: (route: RouteLocationNormalizedLoadedGeneric) => boolean;
  badge?: string;
}>();

const route = useRoute();

const isRouteActive = computed(() => {
  if (props.isActive) {
    return props.isActive(route);
  } else {
    return route.name === props.to;
  }
});
</script>

<template>
  <component
    :is="badge === undefined ? 'div' : OverlayBadge"
    :value="props.badge"
    severity="danger"
  >
    <Button
      pt:root="!justify-start w-full"
      :as="RouterLink"
      :to="{ name: props.to }"
      :icon="props.icon"
      :label="props.label"
      :variant="isRouteActive ? undefined : 'outlined'"
      size="large"
    />
  </component>
</template>
