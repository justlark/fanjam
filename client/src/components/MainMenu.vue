<script setup lang="ts">
import { ref, onMounted } from "vue";
import MainMenuItem from "./MainMenuItem.vue";

const fromRoute = ref<string>("schedule");

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
      label="Program"
      to="program"
      :is-active="
        (route) => route.name === 'program' || (route.name === 'event' && fromRoute === 'program')
      "
    />
    <MainMenuItem
      icon="bi bi-info-circle"
      label="Info"
      to="info"
      :is-active="(route) => route.name === 'info' || route.name === 'page'"
    />
  </nav>
</template>
