<script setup lang="ts">
import { computed, ref, watch } from "vue";
import LinkShareDialog from "./LinkShareDialog.vue";
import ScheduleShareModal from "./ScheduleShareModal.vue";
import Dialog from "primevue/dialog";
import Button from "primevue/button";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const scheduleShareDialogVisible = ref(false);
const appShareDialogVisible = ref(false);

// Do not include the query params or fragment; users likely aren't intending
// to share their current search/filter params.
const appUrl = computed(() => window.location.origin + window.location.pathname);

watch(scheduleShareDialogVisible, (newValue, oldValue) => {
  if (oldValue && !newValue) {
    // If the schedule share dialog is closed, also close the main share dialog.
    visible.value = false;
  }
});

watch(appShareDialogVisible, (newValue, oldValue) => {
  if (oldValue && !newValue) {
    // If the app share dialog is closed, also close the main share dialog.
    visible.value = false;
  }
});
</script>

<template>
  <div>
    <Dialog
      class="max-w-100 m-4"
      v-model:visible="visible"
      modal
      dismissable-mask
      block-scroll
      header="Share"
      :draggable="false"
    >
      <div class="flex flex-col gap-3">
        <p>Share a link to this app.</p>
        <Button
          data-testid="share-dialog-share-app-button"
          size="large"
          label="Share This App"
          icon="bi bi-share-fill"
          variant="outlined"
          @click="appShareDialogVisible = true"
        />
      </div>
      <div class="flex justify-center items-center my-4">
        <div class="border-b w-8 mr-4"></div>
        <span>or</span>
        <div class="border-b w-8 ml-4"></div>
      </div>
      <div class="flex flex-col gap-3">
        <p>Share your schedule with a friend or move it to another device.</p>
        <Button
          data-testid="share-dialog-share-schedule-button"
          size="large"
          label="Share My Schedule"
          icon="bi bi-star"
          variant="outlined"
          @click="scheduleShareDialogVisible = true"
        />
      </div>
    </Dialog>
    <LinkShareDialog
      v-model:visible="appShareDialogVisible"
      title="Share This App"
      :link="appUrl"
      message="Send someone a link to this app."
      toast-message="A link to this app has been copied to your clipboard."
      data-testid="app-share-dialog"
    />
    <ScheduleShareModal v-model:visible="scheduleShareDialogVisible" />
  </div>
</template>
