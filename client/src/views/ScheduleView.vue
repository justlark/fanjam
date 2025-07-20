<script setup lang="ts">
import useEvents from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";

const { events } = useEvents();
</script>

<template>
  <SiteNav title="My Con">
    <div class="flex justify-center">
      <ScheduleTimeline class="p-6 grow max-w-240" :events="events" />
    </div>
  </SiteNav>
</template>
