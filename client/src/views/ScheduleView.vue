<script setup lang="ts">
import AppRoot from "@/components/AppRoot.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import ScrollTop from "primevue/scrolltop";
</script>

<template>
  <AppRoot>
    <div class="h-full p-6 lg:contain-strict lg:overflow-y-auto">
      <div class="flex justify-center">
        <ScheduleTimeline class="grow max-w-240" />
      </div>
      <ScrollTop target="parent" />
    </div>
  </AppRoot>
</template>
