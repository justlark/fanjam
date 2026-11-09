<script setup lang="ts">
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import { exitShareMode } from "@/router";
import { useToast } from "primevue/usetoast";
import { useRoute, useRouter } from "vue-router";
import { TOAST_TTL_LONG } from "@/utils/toast";
import useStarredEvents from "@/composables/useStarredEvents";
import useScheduleSync from "@/composables/useScheduleSync";
import { nextTick } from "vue";

const router = useRouter();
const toast = useToast();
const route = useRoute();
const starredEvents = useStarredEvents();
const { addStars } = useScheduleSync();

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const clearSharedSchedule = async () => {
  exitShareMode();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { share: _, ...query } = route.query;
  await router.replace({ query });
};

const exitSharedSchedule = async () => {
  await clearSharedSchedule();

  toast.add({
    severity: "info",
    summary: "Returning to your schedule",
    detail: "You are no longer viewing someone else's schedule.",
    life: TOAST_TTL_LONG,
  });

  visible.value = false;
};

const addToSchedule = async () => {
  const sharedSchedule = new Set([...starredEvents.value]);
  await clearSharedSchedule();

  // Leaving share mode switches `starredEvents` back to the user's own stars in a watcher. Wait for
  // that to land so we merge into their schedule rather than the shared one.
  await nextTick();

  // Going through sync (rather than assigning `starredEvents` directly) makes sure the merge is
  // pushed on top of the server's schedule, even if this load hasn't pulled it yet.
  const eventsAdded = addStars(sharedSchedule);

  toast.add({
    severity: "info",
    summary: "Added to your schedule",
    detail:
      eventsAdded.size === 0
        ? `You already have all these events in your schedule.`
        : `This schedule has been merged with yours. Added ${eventsAdded.size.toString()} events.`,
    life: TOAST_TTL_LONG,
  });

  visible.value = false;
};
</script>

<template>
  <Dialog
    class="max-w-100 m-4"
    v-model:visible="visible"
    modal
    dismissable-mask
    block-scroll
    :draggable="false"
    header="Schedule Options"
    data-testid="schedule-share-options-dialog"
  >
    <p>
      You are currently viewing a schedule someone shared with you. Do you want to exit and return
      to your own schedule, or do you want to add these events to your schedule?
    </p>
    <template #footer>
      <div class="flex flex-col gap-2 w-full justify-stretch">
        <Button
          @click="exitSharedSchedule"
          label="Return to My Schedule"
          icon="bi bi-arrow-90deg-left"
          data-testid="schedule-share-options-return-button"
        />
        <Button
          @click="addToSchedule"
          label="Add to My Schedule"
          icon="bi bi-plus-circle"
          data-testid="schedule-share-options-add-button"
        />
      </div>
    </template>
  </Dialog>
</template>
