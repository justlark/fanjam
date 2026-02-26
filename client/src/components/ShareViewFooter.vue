<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";
import { exitShareMode } from "@/router";
import IconButton from "./IconButton.vue";

const route = useRoute();
const router = useRouter();

const exitSharedSchedule = async () => {
  exitShareMode();
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { share: _, ...query } = route.query;
  await router.replace({ query });
};
</script>

<template>
  <div class="p-4">
    <div
      class="flex gap-2 items-center justify-between rounded-2xl bg-surface-0 dark:bg-surface-800 shadow-xl py-4 px-6"
    >
      <span class="text-sm font-bold">You are viewing someone else's schedule.</span>
      <IconButton @click="exitSharedSchedule" icon="x-lg" label="Exit" size="md" />
    </div>
  </div>
</template>
