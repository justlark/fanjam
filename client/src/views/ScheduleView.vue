<script setup lang="ts">
import AppRoot from "@/components/AppRoot.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import ScrollTop from "primevue/scrolltop";
</script>

<template>
  <AppRoot>
    <div class="flex justify-center h-full">
      <ScheduleTimeline class="p-6 grow max-w-240" />
      <ScrollTop />
    </div>
  </AppRoot>
</template>
