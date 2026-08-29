<script setup lang="ts">
import { watchEffect } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useRegisterSW } from "virtual:pwa-register/vue";
import { useToast } from "primevue/usetoast";

const toast = useToast();
const { needRefresh, updateServiceWorker } = useRegisterSW();
const { invalidate } = useRemoteData();

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
  // Mark the local cache as stale. Wiping the local cache would be bad,
  // because when the page refreshes, it will only refetch the data relevant to
  // the current page. If the user then goes offline, they will be missing data
  // from their offline cache.
  invalidate();

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
