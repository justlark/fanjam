<script setup lang="ts">
import { ref, computed } from "vue";
import ProgressSpinner from "primevue/progressspinner";
import { useRoute } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import EventDetails from "@/components/EventDetails.vue";
import ScrollTop from "primevue/scrolltop";

const route = useRoute();
const {
  data: { events },
} = useRemoteData();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref<number>();

const allCategories = computed(() => getSortedCategories(events.value));

const thisEvent = computed(() => events.value.find((event) => event.id === eventId.value));
</script>

<template>
  <div class="flex h-full">
    <div v-if="thisEvent" class="flex w-full">
      <div class="hidden lg:flex justify-between basis-1/2 grow-0 shrink-0">
        <div class="p-6 grow lg:contain-strict lg:overflow-y-auto">
          <ScheduleTimeline v-model:day="currentDayIndex" />
          <ScrollTop target="parent" />
        </div>
        <Divider layout="vertical" />
      </div>
      <div class="flex basis-1/2 grow lg:grow-0 shrink-0">
        <EventDetails
          class="grow lg:contain-strict lg:overflow-y-auto"
          :event="thisEvent"
          :day="currentDayIndex ?? 0"
          :all-categories="allCategories"
        />
      </div>
    </div>
    <div v-else class="m-auto">
      <ProgressSpinner />
    </div>
  </div>
</template>
