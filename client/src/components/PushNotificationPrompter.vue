<script lang="ts">
import { ref } from "vue";

// Every view wraps itself in `AppRoot`, so this component unmounts and
// remounts on every navigation. This state lives at module scope so that it
// outlives any single instance: the prompt is scheduled at most once per page
// load, and stays visible across navigations until the user acts on it.

// Whether the prompt should currently be on screen.
const promptVisible = ref(false);

// `undefined` means the prompt hasn't been scheduled yet.
let timer: ReturnType<typeof setTimeout> | undefined;
</script>

<script setup lang="ts">
import { onMounted, watch, watchEffect } from "vue";
import Toast from "primevue/toast";
import Button from "primevue/button";
import { useToast } from "primevue/usetoast";
import useEnvId from "@/composables/useEnvId";
import useRemoteData from "@/composables/useRemoteData";
import usePushNotifications, { promptDismissedKey } from "@/composables/usePushNotifications";
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

const dismissedKey = () => promptDismissedKey(envId.value);

const isFeatureOn = () =>
  (config.value?.usePushNotifications ?? true) && config.value?.hideAnnouncements !== true;
// `granted-unsubscribed` covers the case where the user already granted
// permission for another env on the same origin, but hasn't opted into this
// one yet. Enabling from there doesn't show a browser permission dialog.
const isPromptable = () =>
  (state.value === "default" || state.value === "granted-unsubscribed") && isFeatureOn();

// The toast bus is global, so only the currently mounted instance renders the
// prompt. Clearing the group first guarantees we never stack duplicates.
const show = () => {
  toast.removeGroup("push-prompt");
  if (!isPromptable()) return;

  toast.add({
    severity: "secondary",
    summary: "Get notified when there's a new announcement?",
    group: "push-prompt",
  });
};

// Watchers are stopped on unmount, so only the live instance reacts when the
// timer fires.
watch(promptVisible, (visible) => {
  if (visible) show();
});

onMounted(() => {
  // The previous instance's `<Toast>` took its messages with it when it
  // unmounted, so put the prompt back on this one. This runs in `onMounted`
  // rather than an immediate watcher so the `<Toast>` is listening first.
  if (promptVisible.value) {
    show();
    return;
  }

  if (timer !== undefined) return;
  if (!isPromptable()) return;
  if (localStorage.getItem(dismissedKey()) === "true") return;

  timer = setTimeout(() => {
    promptVisible.value = true;
  }, PROMPT_DELAY_MS);
});

watchEffect(() => {
  if (!isPromptable()) {
    clearTimeout(timer);
    timer = undefined;
    promptVisible.value = false;
    toast.removeGroup("push-prompt");
  }
});

const enable = async () => {
  promptVisible.value = false;
  toast.removeGroup("push-prompt");
  const result = await requestAndSubscribe();

  // TODO: DRY this up.
  if (result === "granted-subscribed") {
    toast.add({
      severity: "success",
      summary: "Notifications enabled",
      detail: "You'll get updates when there's a new announcement.",
      life: TOAST_TTL_LONG,
    });

    umami.track("enable-push", { source: "prompt" });
  } else if (result === "granted-unsubscribed") {
    // Permission was granted, but we could not send the subscription. This
    // likely means the user is offline.
    toast.add({
      severity: "error",
      summary: "Could not set up notifications",
      detail: "Are you offline?",
      life: TOAST_TTL_LONG,
    });
  }
};

const dismiss = () => {
  promptVisible.value = false;
  toast.removeGroup("push-prompt");
  localStorage.setItem(dismissedKey(), "true");

  toast.add({
    severity: "info",
    summary: "You won't receive notifications",
    detail: "You can always enable them from the Announcements page.",
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
