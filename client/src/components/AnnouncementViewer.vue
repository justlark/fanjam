<script setup lang="ts">
import { ref, useId, watchEffect, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import IconButton from "./IconButton.vue";
import ProgressSpinner from "primevue/progressspinner";
import * as commonmark from "commonmark";

const route = useRoute();
const router = useRouter();

const announcements = ref<Array<Announcement>>([
  {
    id: "1",
    title: "Elevator Maintenance",
    body: "Elevators are down for maintenance!",
    attachments: [
      {
        fileName: "README.md",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
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
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
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
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/README.md",
      },
      {
        fileName: "LICENSE",
        signedUrl: "https://raw.githubusercontent.com/justlark/fanjam/refs/heads/main/LICENSE",
      },
    ],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
]);

const announcementId = computed(() => route.params.announcementId as string);

const announcement = computed(() => {
  return announcements.value.find((p) => p.id === announcementId.value);
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
  if (announcementsStatus.value === "success" && !page.value) {
    await back();
  }
});

const announcementHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <article class="max-w-200 mx-auto" v-if="announcement" :aria-labelledby="announcementHeadingId">
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton icon="chevron-left" label="Back" @click="back()" />
        <h2 :id="announcementHeadingId" class="text-xl font-bold">{{ announcement.title }}</h2>
      </div>
      <div class="px-6" id="document" v-if="bodyHtml" v-html="bodyHtml"></div>
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
