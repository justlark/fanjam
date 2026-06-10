<script setup lang="ts">
import { onMounted, watchEffect } from "vue";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useToast } from "primevue/usetoast";
import useEnvId from "@/composables/useEnvId";
import useRemoteData from "@/composables/useRemoteData";
import usePushNotifications from "@/composables/usePushNotifications";

// Delay before showing the toast so it doesn't fire as the first thing a
// visitor sees on cold load — gives the schedule/announcement views a
// moment to render first.
const PROMPT_DELAY_MS = 4000;

const toast = useToast();
const envId = useEnvId();
const {
  data: { config },
} = useRemoteData();
const { state, requestAndSubscribe } = usePushNotifications();

const dismissedKey = () => `push-prompt-dismissed:${envId.value}`;

const isFeatureOn = () => config.value?.usePushNotifications ?? true;
const isPromptable = () => state.value === "default" && isFeatureOn();

let timer: ReturnType<typeof setTimeout> | undefined;

onMounted(() => {
  if (!isPromptable()) return;
  if (localStorage.getItem(dismissedKey()) === "true") return;

  timer = setTimeout(() => {
    // Re-check at fire time — the user may have granted/denied or navigated
    // away in the meantime.
    if (!isPromptable()) return;
    toast.add({
      severity: "secondary",
      summary: "Get notified when there's a new announcement?",
      group: "push-prompt",
    });
  }, PROMPT_DELAY_MS);
});

watchEffect(() => {
  if (state.value !== "default") {
    clearTimeout(timer);
    toast.removeGroup("push-prompt");
  }
});

const enable = async () => {
  toast.removeGroup("push-prompt");
  await requestAndSubscribe();
};

const dismiss = () => {
  toast.removeGroup("push-prompt");
  localStorage.setItem(dismissedKey(), "true");
};
</script>

<template>
  <Toast position="bottom-center" group="push-prompt">
    <template #message="slotProps">
      <div class="flex items-center justify-between gap-4">
        <div>{{ slotProps.message.summary }}</div>
        <div class="flex gap-2">
          <Button
            class="break-normal"
            size="small"
            label="Not now"
            severity="secondary"
            text
            @click="dismiss()"
          />
          <Button
            class="break-normal"
            size="small"
            label="Enable"
            severity="primary"
            @click="enable()"
          />
        </div>
      </div>
    </template>
  </Toast>
</template>
