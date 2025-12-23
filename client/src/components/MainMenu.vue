<script setup lang="ts">
import { ref, onMounted } from "vue";
import MainMenuItem from "./MainMenuItem.vue";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";

const fromRoute = ref<string>("schedule");
const unreadAnnouncements = useUnreadAnnouncements();

onMounted(() => {
  if (history.state.from !== undefined) {
    fromRoute.value = history.state.from;
  }
});
</script>

<template>
  <nav class="flex flex-col gap-3" aria-label="Menu">
    <MainMenuItem
      icon="bi bi-calendar-event"
      label="Schedule"
      to="schedule"
      :is-active="
        (route) => route.name === 'schedule' || (route.name === 'event' && fromRoute === 'schedule')
      "
    />
    <MainMenuItem
      icon="bi bi-book"
      label="All Events"
      to="program"
      :is-active="
        (route) => route.name === 'program' || (route.name === 'event' && fromRoute === 'program')
      "
    />
    <MainMenuItem
      icon="bi bi-megaphone"
      label="Announcements"
      to="announcements"
      :is-active="(route) => route.name === 'announcement' || route.name === 'announcements'"
      :badge="unreadAnnouncements.size === 0 ? undefined : unreadAnnouncements.size.toString()"
    />
    <MainMenuItem
      icon="bi bi-info-circle"
      label="Info"
      to="info"
      :is-active="(route) => route.name === 'info' || route.name === 'page'"
    />
  </nav>
</template>
