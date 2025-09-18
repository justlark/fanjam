<script setup lang="ts">
import AppRoot from "@/components/AppRoot.vue";
import AnnouncementsList from "@/components/AnnouncementsList.vue";
</script>

<template>
  <AppRoot>
    <div class="flex justify-center h-full">
      <AnnouncementsList class="p-6 grow max-w-240" />
    </div>
  </AppRoot>
</template>
