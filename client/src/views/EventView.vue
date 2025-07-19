<script setup lang="ts">
import { ref, computed } from "vue";
import { useRoute } from "vue-router";
import useEvents from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline";
import EventDetails from "@/components/EventDetails";

const route = useRoute();
const events = useEvents();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref(0);

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
    <div class="lg:flex h-full">
      <ScheduleTimeline
        class="p-6 hidden lg:block basis-1/2 grow-0 shrink-0"
        v-model:day="currentDayIndex"
        :events="events"
      />
      <div class="hidden lg:flex basis-1/2 grow-0 shrink-0">
        <Divider layout="vertical" />
        <EventDetails
          v-if="thisEvent"
          :event="thisEvent"
          :day="currentDayIndex"
          :all-categories="allCategories"
        />
      </div>
    </div>
  </SiteNav>
</template>
