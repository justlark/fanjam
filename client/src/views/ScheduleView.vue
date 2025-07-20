<script setup lang="ts">
import useEvents from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";

const { events } = useEvents();
</script>

<template>
  <SiteNav title="My Con">
    <ScheduleTimeline class="p-6" :events="events" />
  </SiteNav>
</template>
