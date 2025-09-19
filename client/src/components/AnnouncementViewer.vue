<script setup lang="ts">
import { useId, watchEffect, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import IconButton from "./IconButton.vue";
import ProgressSpinner from "primevue/progressspinner";
import LinksList from "./LinksList.vue";
import EventDetail from "./EventDetail.vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import { localizeDatetime, timeIsNearlyEqual } from "@/utils/time";
import * as commonmark from "commonmark";

const route = useRoute();
const router = useRouter();
const datetimeFormats = useDatetimeFormats();

const {
  data: { announcements },
  status: { announcements: announcementsStatus },
} = useRemoteData();

const announcementId = computed(() => route.params.announcementId as string);

const announcement = computed(() => {
  return announcements.find((p) => p.id === announcementId.value);
});

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const bodyHtml = computed(() => {
  if (!announcement.value?.body) return undefined;
  const parsed = mdReader.parse(announcement.value.body);
  return mdWriter.render(parsed);
});

const back = async () => {
  await router.push({
    name: "announcements",
  });
};

watchEffect(async () => {
  if (announcementsStatus.value === "success" && !announcement.value) {
    await back();
  }
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
              v-if="!timeIsNearlyEqual(announcement.createdAt, announcement.updatedAt)"
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
          :files="[...announcement.attachments]"
          :pages="[]"
          data-testid="announcement-attachments-list"
        />
      </div>
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
