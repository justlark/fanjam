<script setup lang="ts">
import { ref } from "vue";
import SimpleIcon from "./SimpleIcon.vue";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import { localizeDatetime } from "@/utils/time";
import { type Announcement } from "@/utils/api";

const datetimeFormats = useDatetimeFormats();

const announcements = ref<Array<Announcement>>([
  {
    id: "1",
    title: "Elevator Maintenance",
    body: "Elevators are down for maintenance!",
    attachments: [
      {
        fileName: "README.md",
        mediaType: "text/markdown",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
        mediaType: "text/plain",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/LICENSE",
      },
    ],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: "2",
    title: "Dealers' Den Is Now Open",
    body: "Come check it out!",
    attachments: [
      {
        fileName: "README.md",
        mediaType: "text/markdown",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
        mediaType: "text/plain",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/LICENSE",
      },
    ],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: "3",
    title: "Fursuit Dance Competition Is Starting Soon",
    body: "Come check it out!",
    attachments: [
      {
        fileName: "README.md",
        mediaType: "text/markdown",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
        mediaType: "text/plain",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/LICENSE",
      },
    ],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
]);
</script>

<template>
  <div class="mt-8 text-center flex flex-col gap-10 px-6">
    <div class="flex flex-col items-center gap-4">
      <h1 class="text-3xl">Announcements</h1>
    </div>
    <div class="flex flex-col gap-4">
      <RouterLink
        v-for="announcement of announcements"
        :key="announcement.id"
        :to="{ name: 'announcement', params: { announcementId: announcement.id } }"
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
  </div>
</template>
