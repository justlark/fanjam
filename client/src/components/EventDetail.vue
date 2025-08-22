<script setup lang="ts">
import SimpleIcon from "./SimpleIcon.vue";

const props = defineProps<{
  icon: string;
  iconLabel: string;
}>();
</script>

<template>
  <div class="flex gap-3 items-start">
    <dt>
      <SimpleIcon class="text-xl" :icon="props.icon" :label="props.iconLabel" />
    </dt>
    <dd class="text-lg">
      <slot />
    </dd>
  </div>
</template>
