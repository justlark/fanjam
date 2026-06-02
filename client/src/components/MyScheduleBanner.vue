<script setup lang="ts">
import { ref, computed } from "vue";
import { encodeBase64url } from "@/utils/encoding";
import { downloadStarredEventsIcs } from "@/utils/calendar";
import useStarredEvents from "@/composables/useStarredEvents";
import useRemoteData from "@/composables/useRemoteData";
import useEnvId from "@/composables/useEnvId";
import { useAppUrl } from "@/composables/useAppUrl";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import LinkShareDialog from "./LinkShareDialog.vue";

const starredEvents = useStarredEvents();
const envId = useEnvId();
const appUrl = useAppUrl();
const {
  data: { config, events },
} = useRemoteData();

const calendarExportEnabled = computed(() => config.value?.useCalendarExport ?? true);
const scheduleSharingEnabled = computed(() => config.value?.useScheduleSharing ?? true);
const hasStarredEvents = computed(() => starredEvents.value.size > 0);
const scheduleShareUrl = computed(() => {
  const starredEventIds = [...starredEvents.value];
  starredEventIds.sort();
  return appUrl(`share/?s=${encodeBase64url(starredEventIds.join(","))}`);
});

const shareDialogVisible = ref(false);

const downloadCalendar = () => {
  const starred = starredEvents.value;
  const toExport = events.value.filter((event) => starred.has(event.id));
  if (toExport.length === 0) return;
  downloadStarredEventsIcs(toExport, envId.value, appUrl);
};
</script>

<template>
  <div class="sticky top-16 lg:top-0 z-2 bg-color-light dark:bg-color-dark">
    <div class="lg:hidden">
      <Divider class="!my-0" />
    </div>
    <div class="pl-5 pr-3 lg:pr-5 h-16 flex gap-2 items-center justify-between lg:justify-start">
      <span class="text-xl lg:text-2xl">My Schedule</span>
      <div class="flex">
        <IconButton
          v-if="calendarExportEnabled && hasStarredEvents"
          icon="calendar-plus"
          size="md"
          label="Add to Calendar"
          @click="downloadCalendar"
          :button-props="{ 'data-testid': 'calendar-download-button' }"
        />
        <IconButton
          v-if="scheduleSharingEnabled && hasStarredEvents"
          icon="share-fill"
          size="md"
          label="Share"
          @click="shareDialogVisible = true"
          :button-props="{ 'data-testid': 'schedule-share-button' }"
        />
      </div>
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
