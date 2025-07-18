<script setup lang="ts">
import { ref } from "vue";

import SiteNav from "@/components/SiteNav.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";

const events = ref([
  {
    time: "10:00 am",
    events: [
      {
        title: "An event",
        category: "Category 1",
      },
      {
        title: "An event with a longer title",
        category: "Category 2",
      },
      {
        title: "Another event for you to join",
        category: "Category 3",
      },
    ],
  },
  {
    time: "11:30 am",
    events: [
      {
        title: "An event which has a quite a long title indeed",
        category: "Category 3",
      },
      {
        title: "An event",
        category: "Category 2",
      },
    ],
  },
]);
</script>

<template>
  <SiteNav title="My Con">
    <ScheduleTimeline class="p-6" :events="events" />
  </SiteNav>
</template>
