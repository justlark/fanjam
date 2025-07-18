<script setup lang="ts">
import { ref } from "vue";

import SiteNav from "@/components/SiteNav";
import EventDetails from "@/components/EventDetails";

const event = ref({
  title: "Loving the Unlovable: In Defense of Arthropods",
  description:
    "An event about stuff and things. Much con will be had at this panel, class, or workshop.",
  location: "Thoreau Room",
  startTime: new Date("2025-07-18T09:00:00"),
  endTime: new Date("2025-07-18T10:30:00"),
  people: ["Lark", "Anon"],
  category: "Entertainment",
  tags: ["Tag 1", "Tag 2", "Tag 3"],
});

const allCategories = ["Games", "Social", "Competition", "Entertainment"];
</script>

<template>
  <SiteNav title="My Con">
    <EventDetails :event="event" :all-categories="allCategories" />
  </SiteNav>
</template>
