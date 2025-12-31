<script setup lang="ts">
import AppRoot from "@/components/AppRoot.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
</script>

<template>
  <AppRoot>
    <div class="flex justify-center h-full lg:contain-strict lg:overflow-y-auto">
      <ScheduleTimeline class="p-6 grow max-w-240" />
    </div>
  </AppRoot>
</template>
