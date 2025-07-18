<script setup lang="ts">
import MainMenuItem from "@/components/MainMenuItem.vue";
</script>

<template>
  <nav class="flex flex-col gap-3">
    <MainMenuItem icon="bi bi-calendar-event" label="Schedule" :to="{ name: 'schedule' }" />
    <MainMenuItem icon="bi bi-info-circle" label="Info" :to="{ name: 'info' }" />
  </nav>
</template>
