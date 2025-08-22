<script setup lang="ts">
import MainMenuItem from "./MainMenuItem.vue";
</script>

<template>
  <nav class="flex flex-col gap-3" aria-label="Menu">
    <MainMenuItem
      icon="bi bi-calendar-event"
      label="Schedule"
      to="schedule"
      :active-routes="['schedule', 'event']"
    />
    <MainMenuItem
      icon="bi bi-info-circle"
      label="Info"
      to="info"
      :active-routes="['info', 'page']"
    />
  </nav>
</template>
