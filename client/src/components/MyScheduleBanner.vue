<script setup lang="ts">
import { ref, computed } from "vue";
import { useRoute } from "vue-router";
import { encodeBase64url } from "@/utils/encoding";
import useStarredEvents from "@/composables/useStarredEvents";
import useRemoteData from "@/composables/useRemoteData";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import LinkShareDialog from "./LinkShareDialog.vue";

const route = useRoute();
const starredEvents = useStarredEvents();
const envId = computed(() => route.params.envId as string);
const {
  data: { config },
} = useRemoteData();

const scheduleSharingEnabled = computed(() => config.value?.useScheduleSharing ?? false);
const scheduleShareUrl = computed(() => {
  const starredEventIds = [...starredEvents.value];
  starredEventIds.sort();
  return `${window.location.origin}/app/${envId.value}/share/?s=${encodeBase64url(starredEventIds.join(","))}`;
});

const shareDialogVisible = ref(false);
</script>

<template>
  <div class="sticky top-16 lg:top-0 z-2 bg-color-light dark:bg-color-dark">
    <div class="lg:hidden">
      <Divider class="!my-0" />
    </div>
    <div class="pl-5 pr-3 lg:pr-5 h-16 flex gap-2 items-center justify-between lg:justify-start">
      <span class="text-xl lg:text-2xl">My Schedule</span>
      <IconButton
        v-if="scheduleSharingEnabled"
        icon="share-fill"
        size="md"
        label="Share"
        @click="shareDialogVisible = true"
        :button-props="{ 'data-testid': 'schedule-share-button' }"
      />
    </div>
    <Divider class="!my-0" />
    <LinkShareDialog
      v-model:visible="shareDialogVisible"
      title="Share Your Schedule"
      :link="scheduleShareUrl"
      message="Use this link to share your schedule with a friend or move it to another device."
      toast-message="Share this URL to share your schedule."
    />
  </div>
</template>
