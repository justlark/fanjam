<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import useStarredEvents from "@/composables/useStarredEvents";
import { encodeBase64url } from "@/utils/encoding";
import LinkShareDialog from "@/components/LinkShareDialog.vue";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const route = useRoute();
const starredEvents = useStarredEvents();
const {
  data: { config },
} = useRemoteData();

const envId = computed(() => route.params.envId as string);
const appUrl = computed(() => `${window.location.origin}/app/${envId.value}`);
const scheduleSharingEnabled = computed(() => config.value?.useScheduleSharing ?? false);

const scheduleShareUrl = computed(() => {
  const starredEventIds = [...starredEvents.value];
  starredEventIds.sort();
  return `${window.location.origin}/app/${envId.value}/schedule/all/?star=true&share=${encodeBase64url(starredEventIds.join(","))}`;
});

const links = computed(() => [
  {
    title: "This App",
    link: appUrl.value,
    message: "Send someone a link to this app.",
    toastMessage: "A link to this app has been copied to your clipboard.",
  },
  ...(scheduleSharingEnabled.value
    ? [
        {
          title: "My Schedule",
          link: scheduleShareUrl.value,
          message:
            "Use this link to share your schedule with a friend or move it to another device.",
          toastMessage: "Share this URL to share your schedule.",
        },
      ]
    : []),
]);
</script>

<template>
  <LinkShareDialog v-model:visible="visible" title="Share" :links="links" />
</template>
