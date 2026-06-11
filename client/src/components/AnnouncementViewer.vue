<script setup lang="ts">
import { useId, watch, watchEffect, computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { useAppPath } from "@/composables/useAppUrl";
import IconButton from "./IconButton.vue";
import ProgressSpinner from "primevue/progressspinner";
import LinksList from "./LinksList.vue";
import EventDetail from "./EventDetail.vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useReadAnnouncements from "@/composables/useReadAnnouncements";
import { localizeDatetime, timeIsNearlyEqual } from "@/utils/time";
import { renderMarkdown } from "@/utils/markdown";

const route = useRoute();
const router = useRouter();
const datetimeFormats = useDatetimeFormats();
const readAnnouncementsSet = useReadAnnouncements();

const {
  data: { announcements },
  status: { announcements: announcementsStatus },
  reload: { announcements: reloadAnnouncements },
} = useRemoteData();

const appPath = useAppPath();
const announcementId = computed(() => route.params.announcementId as string);

const liveAnnouncement = computed(() =>
  announcements.value.find((p) => p.id === announcementId.value),
);

// Once we find the current announcement in the list, we latch onto it. This is
// that latch. The purpose is to prevent a stale refresh that drops an
// announcement from bouncing us back to the list page.
const announcement = ref(liveAnnouncement.value);
watch(liveAnnouncement, (current) => {
  if (current) announcement.value = current;
});

// This indicates we're initiating a fresh fetch because we were sent here from
// a push notification, but have not yet found the announcement. We initialize
// this synchronously so the redirect watcher doesn't fire before we've had a
// chance to look.
const resolving = ref(route.query.notified === "1" && liveAnnouncement.value === undefined);

const bodyHtml = computed(() => {
  if (!announcement.value?.body) return undefined;
  return renderMarkdown(announcement.value.body);
});

const back = async () => {
  await router.push({
    name: "announcements",
  });
};

// If the announcement doesn't exist, bounce the user back to the list page. If
// the user was sent here from a push notification, we want to make sure we don't
// bounce them until we've completed a fresh fetch of the latest announcements and
// concluded that it definitely doesn't exist.
watchEffect(async () => {
  if (resolving.value || announcement.value) return;
  if (announcementsStatus.value === "success" && announcements.value.length > 0) {
    await back();
  }
});

// Navigate to the announcement.
watch(
  announcementId,
  async (id) => {
    readAnnouncementsSet.value.add(id);

    // Reset the latch.
    announcement.value = liveAnnouncement.value;

    // Was the user sent here from a push notification?
    if (route.query.notified !== "1") return;

    // Strip the query param so a refresh doesn't re-trigger this path.
    const query = { ...route.query };
    delete query.notified;
    void router.replace({ name: route.name as string, params: route.params, query });

    // The announcement the push notification is for may not be cached locally
    // yet.
    if (announcement.value) return;

    // Trigger a fresh read, bypassing the edge cache.
    resolving.value = true;

    try {
      await reloadAnnouncements({ fresh: true });
    } finally {
      resolving.value = false;
    }
  },
  { immediate: true },
);

const markUnread = async () => {
  readAnnouncementsSet.value.delete(announcementId.value);
  await back();
};

const isUnread = computed(() => {
  return !readAnnouncementsSet.value.has(announcementId.value);
});

const announcementHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <article class="max-w-200 mx-auto" v-if="announcement" :aria-labelledby="announcementHeadingId">
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton
          icon="chevron-left"
          label="Back"
          @click="back()"
          :button-props="{ 'data-testid': 'announcement-back-button' }"
        />
        <h2 :id="announcementHeadingId" class="text-xl font-bold">{{ announcement.title }}</h2>
      </div>
      <div class="px-6">
        <div v-if="datetimeFormats" class="">
          <dl>
            <EventDetail
              class="text-muted-color"
              icon="clock"
              size="sm"
              data-testid="announcement-created-time"
            >
              <span>Posted </span>
              <time>
                {{ localizeDatetime(datetimeFormats, announcement.createdAt) }}
              </time>
            </EventDetail>
            <EventDetail
              v-if="
                announcement.updatedAt &&
                !timeIsNearlyEqual(announcement.createdAt, announcement.updatedAt)
              "
              class="text-muted-color"
              icon="arrow-clockwise"
              size="sm"
              data-testid="announcement-updated-time"
            >
              <span>Updated </span>
              <time class="text-muted-color">
                {{ localizeDatetime(datetimeFormats, announcement.updatedAt) }}
              </time>
            </EventDetail>
          </dl>
        </div>
        <div
          id="document"
          v-if="bodyHtml && announcement?.body.trim() !== ''"
          v-html="bodyHtml"
        ></div>
        <div
          v-else-if="announcement.attachments.length === 0"
          class="text-center text-lg italic text-muted-color mt-8"
          data-testid="announcement-no-details-notice"
        >
          No details provided
        </div>
        <LinksList
          class="max-w-140 w-full mx-auto mt-6"
          v-if="announcement.attachments.length > 0"
          :links="[]"
          :files="
            announcement.attachments.map((file) => ({
              name: file.name,
              mediaType: file.mediaType,
              url: appPath(`files/${file.id}`),
            }))
          "
          :pages="[]"
          data-testid="announcement-attachments-list"
        />
      </div>
      <IconButton
        class="flex justify-center mt-4"
        v-if="!isUnread"
        icon="envelope-open"
        label="Mark Unread"
        size="sm"
        :show-label="true"
        @click="markUnread()"
      />
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
