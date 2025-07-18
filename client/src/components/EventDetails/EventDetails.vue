<script setup lang="ts">
import { useRouter } from "vue-router";
import Button from "primevue/button";

export interface Event {
  title: string;
  description?: string;
  location?: string;
  startTime?: Date;
  endTime?: Date;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

const router = useRouter();

const props = defineProps<{
  event: Event;
}>();
</script>

<template>
  <div class="flex justify-start items-center gap-2 p-4">
    <Button
      icon="bi bi-chevron-left"
      pt:icon="!text-2xl"
      variant="text"
      size="large"
      aria-label="Back"
      rounded
      @click="router.back()"
    />
    <h1 class="text-2xl">{{ props.event.title }}</h1>
  </div>
</template>
