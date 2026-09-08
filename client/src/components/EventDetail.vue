<script setup lang="ts">
import { computed } from "vue";
import SimpleIcon from "./SimpleIcon.vue";

const props = defineProps<{
  icon: string;
  iconLabel?: string;
  size?: "sm" | "lg";
}>();

const size = computed(() => props.size ?? "lg");
</script>

<template>
  <div class="flex gap-3 items-start">
    <dt>
      <SimpleIcon
        :class="{ 'text-xl': size === 'lg', 'text-lg': size === 'sm' }"
        :icon="props.icon"
        :label="props.iconLabel"
      />
    </dt>
    <dd :class="{ 'text-lg': size === 'lg', 'text-md': size === 'sm' }">
      <slot />
    </dd>
  </div>
</template>
