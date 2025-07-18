<script setup lang="ts">
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const props = defineProps<{
  icon: string;
  iconLabel?: string;
}>();
</script>

<template>
  <div class="flex gap-3 items-center">
    <SimpleIcon class="text-xl" :icon="props.icon" :label="props.iconLabel" />
    <span class="text-lg">
      <slot />
    </span>
  </div>
</template>
