<script setup lang="ts">
import MainMenuItem from "./MainMenuItem.vue";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";
import useRemoteData from "@/composables/useRemoteData";

const unreadAnnouncements = useUnreadAnnouncements();
const {
  data: { config, announcements },
} = useRemoteData();
</script>

<template>
  <nav class="flex flex-col gap-3" aria-label="Menu">
    <MainMenuItem
      icon="bi bi-calendar-event"
      label="Schedule"
      :to="{ name: 'schedule' }"
      :is-active="
        (route) =>
          (route.name === 'schedule' || route.name === 'event') && route.query.star !== 'true'
      "
    />
    <MainMenuItem
      icon="bi bi-star"
      label="My Schedule"
      :to="{
        name: 'schedule',
        query: { star: 'true' },
      }"
      :is-active="
        (route) =>
          (route.name === 'schedule' || route.name === 'event') && route.query.star === 'true'
      "
    />
    <MainMenuItem
      v-if="announcements.length > 0 || !(config?.hideAnnouncements ?? false)"
      icon="bi bi-megaphone"
      label="Announcements"
      :to="{ name: 'announcements' }"
      :is-active="(route) => route.name === 'announcement' || route.name === 'announcements'"
      :badge="unreadAnnouncements.size === 0 ? undefined : unreadAnnouncements.size.toString()"
    />
    <MainMenuItem
      icon="bi bi-info-circle"
      label="Info"
      :to="{ name: 'info' }"
      :is-active="(route) => route.name === 'info' || route.name === 'page'"
    />
  </nav>
</template>
