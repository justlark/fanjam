<script setup lang="ts">
import MainMenuItem from "./MainMenuItem.vue";
</script>

<template>
  <nav class="flex flex-col gap-3">
    <MainMenuItem icon="bi bi-calendar-event" label="Schedule" to="schedule" />
    <MainMenuItem icon="bi bi-info-circle" label="Info" to="info" />
  </nav>
</template>
