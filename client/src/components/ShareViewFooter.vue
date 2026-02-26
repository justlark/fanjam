<script setup lang="ts">
import IconButton from "./IconButton.vue";
</script>

<template>
  <div class="p-4">
    <div
      class="flex gap-2 items-center justify-between rounded-2xl bg-surface-0 dark:bg-surface-800 shadow-xl py-4 px-6"
    >
      <span class="text-sm font-bold">You are viewing someone else's schedule.</span>
      <IconButton icon="x-lg" label="Exit" size="md" />
    </div>
  </div>
</template>
