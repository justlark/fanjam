<script setup lang="ts">
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import Button from "primevue/button";
import QrcodeVue from "qrcode.vue";
import Dialog from "primevue/dialog";
import { TOAST_TTL_MEDIUM } from "@/utils/toast";
import { useToast } from "primevue/usetoast";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const props = defineProps<{
  link: string;
  syncEmojis: string;
}>();

const emit = defineEmits<{
  stop: [];
}>();

const toast = useToast();

const copySyncUrl = async () => {
  await navigator.clipboard.writeText(props.link);

  toast.add({
    severity: "info",
    summary: "Link copied",
    detail: "Open this link on your other devices to sync your schedule.",
    life: TOAST_TTL_MEDIUM,
  });
};

const stopSyncing = () => {
  emit("stop");

  visible.value = false;

  toast.add({
    severity: "info",
    summary: "Stopped syncing",
    detail: "This device is no longer syncing your schedule.",
    life: TOAST_TTL_MEDIUM,
  });
};
</script>

<template>
  <Dialog
    class="w-100 m-4"
    v-model:visible="visible"
    modal
    dismissable-mask
    block-scroll
    :draggable="false"
    header="Sync Your Schedule"
  >
    <div>
      <div class="my-auto mb-6">
        <div class="rounded-3xl overflow-hidden mx-auto w-fit shadow-md">
          <QrcodeVue
            :value="props.link"
            :margin="3"
            :size="220"
            foreground="#18181b"
            background="#f1f5f9"
            render-as="svg"
          />
        </div>
      </div>
      <div class="flex flex-col gap-2 justify-end">
        <p data-testid="schedule-sync-dialog-description">
          Open this link on your other devices to keep your schedule in sync between them
          automatically.
        </p>
        <p>
          Your sync code is
          <span data-testid="schedule-sync-dialog-emojis">{{ props.syncEmojis }}</span
          >. Compare this with your other devices to check if they're in sync.
        </p>
      </div>
      <InputGroup class="mt-3">
        <InputGroupAddon>
          <Button
            data-testid="schedule-sync-dialog-copy-button"
            @click="copySyncUrl"
            label="Copy"
            icon="bi bi-clipboard"
          />
        </InputGroupAddon>
        <InputText data-testid="schedule-sync-dialog-url" :value="props.link" disabled />
      </InputGroup>
    </div>
    <template #footer>
      <Button
        fluid
        data-testid="schedule-stop-sync-button"
        @click="stopSyncing"
        label="Stop Syncing"
        icon="bi bi-x-lg"
        severity="danger"
        outlined
      />
    </template>
  </Dialog>
</template>
