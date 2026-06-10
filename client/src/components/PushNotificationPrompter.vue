<script setup lang="ts">
import { onMounted, watchEffect } from "vue";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useToast } from "primevue/usetoast";
import useEnvId from "@/composables/useEnvId";
import useRemoteData from "@/composables/useRemoteData";
import usePushNotifications from "@/composables/usePushNotifications";
import { TOAST_TTL_LONG } from "@/utils/toast";

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

const isFeatureOn = () =>
  (config.value?.usePushNotifications ?? true) && config.value?.hideAnnouncements !== true;
const isPromptable = () => state.value === "default" && isFeatureOn();

let timer: ReturnType<typeof setTimeout> | undefined;

onMounted(() => {
  if (!isPromptable()) return;
  if (localStorage.getItem(dismissedKey()) === "true") return;

  timer = setTimeout(() => {
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
  const result = await requestAndSubscribe();

  if (result === "granted-subscribed") {
    toast.add({
      severity: "success",
      summary: "Notifications enabled! You'll get updates when there's a new announcement.",
      life: TOAST_TTL_LONG,
    });
  } else if (result === "granted-unsubscribed") {
    // Permission was granted, but we could not send the subscription. This
    // likely means the user is offline.
    toast.add({
      severity: "error",
      summary: "Could not set up notifications. Are you offline?",
      life: TOAST_TTL_LONG,
    });
  }
};

const dismiss = () => {
  toast.removeGroup("push-prompt");
  localStorage.setItem(dismissedKey(), "true");

  toast.add({
    severity: "info",
    summary:
      "You won't receive notifications. You can always enable them from the Announcements page.",
    life: TOAST_TTL_LONG,
  });
};
</script>

<template>
  <Toast position="bottom-center" group="push-prompt">
    <template #message="slotProps">
      <div class="flex flex-col justify-between gap-4">
        <div>{{ slotProps.message.summary }}</div>
        <div class="flex gap-4">
          <Button
            class="grow break-normal"
            size="small"
            label="Not now"
            severity="secondary"
            outlined
            @click="dismiss()"
          />
          <Button
            class="grow break-normal"
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
