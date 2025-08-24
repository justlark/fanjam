<script setup lang="ts">
import { ref, watchEffect, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import SiteNav from "@/components/SiteNav.vue";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import EventDetails from "@/components/EventDetails.vue";

const route = useRoute();
const router = useRouter();
const {
  data: { events },
} = useRemoteData();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref(0);

const allCategories = getSortedCategories(events.value);

const thisEvent = computed(() => events.value.find((event) => event.id === eventId.value));

watchEffect(async () => {
  // If the event does not (or no longer) exists, redirect to the schedule view.
  if (!thisEvent.value) {
    await router.replace({
      name: "schedule",
    });
  }
});
</script>

<template>
  <SiteNav>
    <div class="flex h-full">
      <div class="hidden lg:flex justify-between basis-1/2 grow-0 shrink-0">
        <ScheduleTimeline class="p-6 grow" v-model:day="currentDayIndex" />
        <Divider layout="vertical" />
      </div>
      <div class="flex basis-1/2 grow lg:grow-0 shrink-0">
        <EventDetails
          class="grow"
          v-if="thisEvent"
          :event="thisEvent"
          :day="currentDayIndex"
          :all-categories="allCategories"
        />
      </div>
    </div>
  </SiteNav>
</template>
