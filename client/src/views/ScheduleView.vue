<script setup lang="ts">
import AppRoot from "@/components/AppRoot.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import ScrollTop from "primevue/scrolltop";
import MyScheduleBanner from "@/components/MyScheduleBanner.vue";
import useFilterQuery from "@/composables/useFilterQuery";

const filterCriteria = useFilterQuery();
</script>

<template>
  <AppRoot class="lg:overflow-y-hidden">
    <div class="h-full lg:contain-strict lg:overflow-y-auto">
      <MyScheduleBanner v-if="filterCriteria.hideNotStarred" />
      <div class="p-6">
        <div class="flex justify-center">
          <ScheduleTimeline class="grow max-w-240" />
        </div>
        <ScrollTop target="parent" />
      </div>
    </div>
  </AppRoot>
</template>
