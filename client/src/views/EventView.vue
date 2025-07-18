<script setup lang="ts">
import { ref } from "vue";

import SiteNav from "@/components/SiteNav";
import EventDetails from "@/components/EventDetails";

const event = ref({
  title: "My Event",
  description: "An event about stuff and things",
  location: "Thoreau Room",
  startTime: new Date("2025-07-18T09:00:00"),
  endTime: new Date("2025-07-18T10:30:00"),
  people: ["Lark", "Anon"],
  category: "Entertainment",
  tags: ["Tag 1", "Tag 2", "Tag 3"],
});
</script>

<template>
  <SiteNav title="My Con">
    <EventDetails :event="event" />
  </SiteNav>
</template>
