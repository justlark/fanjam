<script setup lang="ts">
import IconButton from "./IconButton.vue";
defineEmits(["click"]);
</script>

<template>
  <div data-testid="share-view-footer" class="p-4">
    <div
      class="flex gap-2 items-center justify-between rounded-2xl bg-surface-0 dark:bg-surface-800 shadow-xl py-4 px-6"
    >
      <span class="font-bold">You are viewing someone else's schedule. Click for options.</span>
      <IconButton
        @click="$emit('click')"
        icon="sliders"
        label="Exit"
        size="md"
        :button-props="{ 'data-testid': 'share-view-footer-exit-button' }"
      />
    </div>
  </div>
</template>
