<script setup lang="ts">
import { computed } from "vue";
import {
  useRoute,
  type RouteLocationAsRelativeGeneric,
  type RouteLocationAsPathGeneric,
  type RouteLocationNormalizedLoadedGeneric,
} from "vue-router";
import OverlayBadge from "primevue/overlaybadge";
import Button from "primevue/button";
import { RouterLink } from "vue-router";

const props = defineProps<{
  to: string | RouteLocationAsRelativeGeneric | RouteLocationAsPathGeneric;
  label: string;
  icon: string;
  isActive: (route: RouteLocationNormalizedLoadedGeneric) => boolean;
  badge?: string;
}>();

const route = useRoute();

const isRouteActive = computed(() => props.isActive(route));
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
      :to="props.to"
      :icon="props.icon"
      :label="props.label"
      :variant="isRouteActive ? undefined : 'outlined'"
      size="large"
    />
  </component>
</template>
