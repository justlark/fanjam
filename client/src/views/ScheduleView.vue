<script setup lang="ts">
import { ref } from "vue";

import SiteNav from "@/components/SiteNav.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";

const days = ref([
  {
    dayName: "Friday",
    timeSlots: [
      {
        localizedTime: "9:00 pm",
        events: [
          {
            title: "An event which has a quite a long title indeed",
            category: "Games",
          },
          {
            title: "An event",
            category: "Social",
          },
        ],
      },
    ],
  },
  {
    dayName: "Saturday",
    timeSlots: [
      {
        localizedTime: "10:00 am",
        events: [
          {
            title: "An event",
            category: "Competition",
          },
          {
            title: "An event with a longer title",
            category: "Social",
          },
          {
            title: "Another event for you to join",
            category: "Entertainment",
          },
        ],
      },
      {
        localizedTime: "11:30 am",
        events: [
          {
            title: "An event which has a quite a long title indeed",
            category: "Entertainment",
          },
          {
            title: "An event",
            category: "Games",
          },
        ],
      },
    ],
  },
]);
</script>

<template>
  <SiteNav title="My Con">
    <ScheduleTimeline class="p-6" :days="days" />
  </SiteNav>
</template>
