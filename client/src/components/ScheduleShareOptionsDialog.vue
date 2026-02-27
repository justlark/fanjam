<script setup lang="ts">
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import { exitShareMode } from "@/router";
import { useToast } from "primevue/usetoast";
import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const toast = useToast();
const route = useRoute();

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const exitSharedSchedule = async () => {
  exitShareMode();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { share: _, ...query } = route.query;
  await router.replace({ query });

  toast.add({
    severity: "info",
    summary: "Returning to your schedule",
    detail: "You are no longer viewing someone else's schedule.",
    life: 2000,
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
    header="Options"
  >
    <p>
      You are currently viewing a schedule someone shared with you. Do you want to exit and return
      to your own schedule, or do you want to add these events to your schedule?
    </p>
    <template #footer>
      <div class="flex flex-col gap-2 w-full justify-stretch">
        <Button @click="exitSharedSchedule" label="Return to My Schedule" icon="bi bi-x-lg" />
        <Button label="Add to My Schedule" icon="bi bi-star" />
      </div>
    </template>
  </Dialog>
</template>
