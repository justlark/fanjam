<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import useEvents from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import EventDetails from "@/components/EventDetails";

const route = useRoute();
const eventId = computed(() => route.params.eventId as string);

const events = useEvents();

const allCategories = computed(() =>
  events.value.reduce((set, event) => {
    if (event.category && !set.includes(event.category)) {
      set.push(event.category);
    }

    return set;
  }, [] as Array<string>),
);

const thisEvent = computed(() => events.value.find((event) => event.id === eventId.value));
</script>

<template>
  <SiteNav title="My Con">
    <EventDetails v-if="thisEvent" :event="thisEvent" :all-categories="allCategories" />
  </SiteNav>
</template>
