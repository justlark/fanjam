<script setup lang="ts">
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";
</script>

<template>
  <SiteNav title="My Con">
    <div class="flex justify-center h-full">
      <ScheduleTimeline class="p-6 grow max-w-240" />
    </div>
  </SiteNav>
</template>
