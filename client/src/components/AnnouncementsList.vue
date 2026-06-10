<script setup lang="ts">
import { computed } from "vue";
import Button from "primevue/button";
import { useToast } from "primevue/usetoast";
import SimpleIcon from "./SimpleIcon.vue";
import OverlayBadge from "primevue/overlaybadge";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";
import useRemoteData from "@/composables/useRemoteData";
import usePushNotifications from "@/composables/usePushNotifications";
import { localizeDatetime } from "@/utils/time";
import { TOAST_TTL_LONG } from "@/utils/toast";

const datetimeFormats = useDatetimeFormats();
const toast = useToast();

const {
  data: { announcements, config },
} = useRemoteData();

const unreadAnnouncements = useUnreadAnnouncements();
const { state: pushState, requestAndSubscribe } = usePushNotifications();

const enablePush = async () => {
  const result = await requestAndSubscribe();
  if (result === "granted-subscribed") {
    toast.add({
      severity: "success",
      summary: "Notifications enabled! You'll get updates when there's a new announcement.",
      life: TOAST_TTL_LONG,
    });
  }
};

// Show the persistent enable button when push notifications are configured
// for this env and the user can still act on it — either they haven't
// answered the OS prompt yet, or they granted permission but no subscription
// exists yet (e.g. they cleared site data or switched browsers).
const canEnablePush = computed(
  () =>
    (config.value?.usePushNotifications ?? true) &&
    config.value?.hideAnnouncements !== true &&
    (pushState.value === "default" || pushState.value === "granted-unsubscribed"),
);

const filteredAnnouncements = computed(() => {
  const withTitle = announcements.value.filter((announcement) => announcement.title.trim() !== "");

  withTitle.sort((a, b) => {
    return (b.updatedAt ?? b.createdAt).valueOf() - (a.updatedAt ?? a.createdAt).valueOf();
  });

  return withTitle;
});
</script>

<template>
  <div class="mt-8 text-center flex flex-col gap-10 px-6">
    <div class="flex flex-col items-center gap-4">
      <h1 class="text-3xl">Announcements</h1>
      <Button
        v-if="canEnablePush"
        size="small"
        severity="secondary"
        icon="pi pi-bell"
        label="Enable push notifications"
        @click="enablePush()"
      />
    </div>
    <div v-if="filteredAnnouncements.length > 0" class="flex flex-col gap-4">
      <RouterLink
        v-for="announcement of filteredAnnouncements"
        :key="announcement.id"
        :to="{ name: 'announcement', params: { announcementId: announcement.id } }"
        data-testid="announcement-link"
      >
        <component
          :is="unreadAnnouncements.has(announcement.id) ? OverlayBadge : 'div'"
          size="small"
          severity="danger"
          class="group flex items-center gap-6 text-left border border-surface-300 hover:bg-surface-200 dark:border-surface-600 hover:dark:bg-surface-800 rounded-sm px-4 py-2 mx-auto max-w-160"
        >
          <SimpleIcon icon="megaphone" class="text-2xl" />
          <div class="flex flex-col items-start gap-1">
            <h2 class="text-link-sm group-hover:decoration-2 text-lg">
              {{ announcement.title }}
            </h2>
            <span v-if="datetimeFormats" class="text-muted-color">{{
              localizeDatetime(datetimeFormats, announcement.updatedAt ?? announcement.createdAt)
            }}</span>
          </div>
        </component>
      </RouterLink>
    </div>
    <div
      v-else
      class="text-center text-lg italic text-muted-color"
      data-testid="announcements-empty"
    >
      No announcements yet
    </div>
  </div>
</template>
