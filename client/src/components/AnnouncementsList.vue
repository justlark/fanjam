<script setup lang="ts">
import { computed } from "vue";
import SimpleIcon from "./SimpleIcon.vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useRemoteData from "@/composables/useRemoteData";
import { localizeDatetime } from "@/utils/time";

const datetimeFormats = useDatetimeFormats();

const {
  data: { announcements },
} = useRemoteData();

const filteredAnnouncements = computed(() => {
  const withTitle = announcements.filter((announcement) => announcement.title.trim() !== "");

  withTitle.sort((a, b) => {
    return b.updatedAt.valueOf() - a.updatedAt.valueOf();
  });

  return withTitle;
});
</script>

<template>
  <div class="mt-8 text-center flex flex-col gap-10 px-6">
    <div class="flex flex-col items-center gap-4">
      <h1 class="text-3xl">Announcements</h1>
    </div>
    <div v-if="filteredAnnouncements.length > 0" class="flex flex-col gap-4 pb-6">
      <RouterLink
        v-for="announcement of filteredAnnouncements"
        :key="announcement.id"
        :to="{ name: 'announcement', params: { announcementId: announcement.id } }"
        data-testid="announcement-link"
      >
        <div
          class="flex items-center gap-6 text-left border border-surface-300 hover:bg-surface-200 dark:border-surface-600 hover:dark:bg-surface-800 rounded-sm px-4 py-2 mx-auto max-w-160 hover:*:decoration-2"
        >
          <SimpleIcon icon="megaphone" class="text-2xl" />
          <div class="flex flex-col items-start gap-1">
            <h2 class="text-primary underline underline-offset-2 text-lg">
              {{ announcement.title }}
            </h2>
            <span v-if="datetimeFormats" class="text-muted-color">{{
              localizeDatetime(datetimeFormats, announcement.updatedAt)
            }}</span>
          </div>
        </div>
      </RouterLink>
    </div>
    <div v-else class="text-center text-lg italic text-muted-color">No announcements yet</div>
  </div>
</template>
