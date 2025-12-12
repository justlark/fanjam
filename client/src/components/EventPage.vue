<script setup lang="ts">
import { ref, watchEffect, onMounted, computed } from "vue";
import ProgressSpinner from "primevue/progressspinner";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import EventProgram from "@/components/EventProgram.vue";
import EventDetails from "@/components/EventDetails.vue";

const route = useRoute();
const router = useRouter();
const {
  data: { events },
  status: { events: eventsStatus },
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

  watchEffect(async () => {
    if (eventsStatus.value === "success" && thisEvent.value === undefined) {
      await router.push({ name: from.value });
    }
  });
});
</script>

<template>
  <div class="flex h-full overflow-hidden">
    <div v-if="thisEvent" class="flex w-full h-full min-h-0">
      <div
        class="hidden lg:flex justify-between basis-1/2 grow-0 overflow-y-auto overflow-x-hidden min-h-0"
      >
        <div class="grow">
          <ScheduleTimeline v-if="from === 'schedule'" class="p-6" v-model:day="currentDayIndex" />
        </div>
        <div>
          <EventProgram
            v-if="from === 'program'"
            class="p-6 max-w-240"
            :focused-event-id="eventId"
          />
        </div>
      </div>
      <Divider class="!hidden lg:!block" layout="vertical" />
      <div class="flex basis-1/2 grow lg:grow-0 shrink-0 overflow-y-auto overflow-x-hidden min-h-0">
        <EventDetails
          class="grow"
          :event="thisEvent"
          :day="currentDayIndex ?? 0"
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
