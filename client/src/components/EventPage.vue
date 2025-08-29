<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import ProgressSpinner from "primevue/progressspinner";
import { useRoute } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import EventProgram from "@/components/EventProgram.vue";
import EventDetails from "@/components/EventDetails.vue";

const route = useRoute();
const {
  data: { events },
} = useRemoteData();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref<number>();

const allCategories = computed(() => getSortedCategories(events));

const thisEvent = computed(() => events.find((event) => event.id === eventId.value));

const from = ref<"schedule" | "program">();

onMounted(() => {
  if (history.state.from === undefined) {
    from.value = "schedule";
  } else {
    from.value = history.state.from;
  }
});
</script>

<template>
  <div class="flex h-full">
    <div v-if="thisEvent" class="flex w-full">
      <div class="hidden lg:flex justify-between basis-1/2 grow-0 shrink-0">
        <ScheduleTimeline
          v-if="from === 'schedule'"
          class="p-6 grow"
          v-model:day="currentDayIndex"
        />
        <EventProgram v-if="from === 'program'" class="p-6 grow max-w-240" />
        <Divider layout="vertical" />
      </div>
      <div class="flex basis-1/2 grow lg:grow-0 shrink-0">
        <EventDetails
          v-if="currentDayIndex"
          class="grow"
          :event="thisEvent"
          :day="currentDayIndex"
          :all-categories="allCategories"
          :from="from"
        />
      </div>
    </div>
    <div v-else class="m-auto">
      <ProgressSpinner />
    </div>
  </div>
</template>
