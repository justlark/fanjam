<script setup lang="ts">
import { ref } from "vue";

import SiteNav from "@/components/SiteNav";
import ScheduleTimeline from "@/components/ScheduleTimeline";

const days = ref([
  {
    dayName: "Friday",
    timeSlots: [
      {
        localizedTime: "9:00 pm",
        events: [
          {
            id: "1",
            title: "An event which has a quite a long title indeed",
            category: "Games",
          },
          {
            id: "2",
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
            id: "3",
            title: "An event",
            category: "Competition",
          },
          {
            id: "4",
            title: "An event with a longer title",
            category: "Social",
          },
          {
            id: "5",
            title: "Another event for you to join",
            category: "Entertainment",
          },
        ],
      },
      {
        localizedTime: "11:30 am",
        events: [
          {
            id: "6",
            title: "An event which has a quite a long title indeed",
            category: "Entertainment",
          },
          {
            id: "7",
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
