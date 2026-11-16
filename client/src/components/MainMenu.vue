<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import MainMenuItem from "./MainMenuItem.vue";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";
import useRemoteData from "@/composables/useRemoteData";

const route = useRoute();
const unreadAnnouncements = useUnreadAnnouncements();
const {
  data: { config, announcements, vendorPages, vendors },
} = useRemoteData();

const filteredVendorPages = computed(() => {
  if (config.value?.useVendorPages ?? true) return vendorPages.value;

  return [];
});

const vendorPageIdIfVendorView = computed(() => {
  if (route.name === "vendor" && route.params.vendorId) {
    const vendor = vendors.value.find((v) => v.id === (route.params.vendorId as string));
    return vendor?.page?.id;
  }

  return undefined;
});
</script>

<template>
  <nav class="flex flex-col gap-3" aria-label="Menu" data-testid="main-menu">
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
      v-for="vendorPage in filteredVendorPages"
      :key="vendorPage.id"
      icon="bi bi-people"
      :label="vendorPage.title"
      :to="{ name: 'vendor-page', params: { vendorPageId: vendorPage.id } }"
      :is-active="
        (route) =>
          (route.name === 'vendor-page' && route.params.vendorPageId === vendorPage.id) ||
          (route.name === 'vendor' && vendorPageIdIfVendorView === vendorPage.id)
      "
    />
    <MainMenuItem
      icon="bi bi-info-circle"
      label="Info"
      :to="{ name: 'info' }"
      :is-active="(route) => route.name === 'info' || route.name === 'page'"
    />
  </nav>
</template>
