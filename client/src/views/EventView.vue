<script setup lang="ts">
import { ref, computed } from "vue";
import { useRoute } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import SiteNav from "@/components/SiteNav";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline";
import EventDetails from "@/components/EventDetails";

const route = useRoute();
const {
  data: { events },
} = useRemoteData();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref(0);

const allCategories = computed(() =>
  events.value.reduce<Array<string>>((set, event) => {
    if (event.category && !set.includes(event.category)) {
      set.push(event.category);
    }

    return set;
  }, []),
);

const thisEvent = computed(() => events.value.find((event) => event.id === eventId.value));
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
