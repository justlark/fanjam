<script setup lang="ts">
import SiteNav from "@/components/SiteNav.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
</script>

<template>
  <SiteNav>
    <div class="flex justify-center h-full">
      <ScheduleTimeline class="p-6 grow max-w-240" />
    </div>
  </SiteNav>
</template>
