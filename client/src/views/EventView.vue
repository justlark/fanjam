<script setup lang="ts">
import { ref, watchEffect, onMounted, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { getSortedCategories } from "@/utils/tags";
import PageRoot from "@/components/PageRoot.vue";
import Divider from "primevue/divider";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import EventProgram from "@/components/EventProgram.vue";
import EventDetails from "@/components/EventDetails.vue";

const route = useRoute();
const router = useRouter();
const {
  data: { events },
} = useRemoteData();

const eventId = computed(() => route.params.eventId as string);
const currentDayIndex = ref(0);

const allCategories = computed(() => getSortedCategories(events));

const thisEvent = computed(() => events.find((event) => event.id === eventId.value));

const from = ref<"schedule" | "program">();

watchEffect(async () => {
  // If the event does not (or no longer) exists, redirect to the schedule view.
  if (!thisEvent.value) {
    await router.replace({
      name: "schedule",
    });
  }
});

onMounted(() => {
  if (history.state.from !== undefined) {
    from.value = history.state.from;
  }
});
</script>

<template>
  <PageRoot>
    <div class="flex h-full">
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
          class="grow"
          v-if="thisEvent"
          :event="thisEvent"
          :day="currentDayIndex"
          :all-categories="allCategories"
          :from="from"
        />
      </div>
    </div>
  </PageRoot>
</template>
