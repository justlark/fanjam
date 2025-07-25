<script setup lang="ts">
import { watchEffect } from "vue";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useRegisterSW } from "virtual:pwa-register/vue";
import { useToast } from "primevue/usetoast";

const toast = useToast();
const { needRefresh, updateServiceWorker } = useRegisterSW();

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
  await updateServiceWorker();
  toast.removeGroup("app-update");
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
