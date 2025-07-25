<script setup lang="ts">
import { watchEffect } from "vue";
import { useRemoteInfo, useRemoteEvents } from "@/composables/useRemoteData";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useRegisterSW } from "virtual:pwa-register/vue";
import { useToast } from "primevue/usetoast";

const toast = useToast();
const { needRefresh, updateServiceWorker } = useRegisterSW();
const { clear: clearInfo } = useRemoteInfo();
const { clear: clearEvents } = useRemoteEvents();

watchEffect(() => {
  if (needRefresh.value) {
    toast.add({
      severity: "secondary",
      summary: "A new version of the app is available",
      group: "app-update",
    });
  }
});

const update = async () => {
  // Clear the local storage storage when the app updates.
  clearInfo();
  clearEvents();

  // Update the service worker, otherwise the user would need to close all open
  // FanJam tabs to get the new version.
  await updateServiceWorker();
};
</script>

<template>
  <Toast position="bottom-center" group="app-update">
    <template #message="slotProps">
      <div class="flex items-center justify-between gap-4">
        <div>{{ slotProps.message.summary }}</div>
        <Button
          class="break-normal"
          size="small"
          label="Update"
          severity="primary"
          @click="update()"
        />
      </div>
    </template>
  </Toast>
</template>
