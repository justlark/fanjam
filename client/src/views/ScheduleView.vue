<script setup lang="ts">
import { ref } from "vue";
import AppRoot from "@/components/AppRoot.vue";
import ScheduleTimeline from "@/components/ScheduleTimeline.vue";
import ScrollTop from "primevue/scrolltop";
import MyScheduleBanner from "@/components/MyScheduleBanner.vue";
import ShareViewFooter from "@/components/ShareViewFooter.vue";
import ScheduleShareOptionsDialog from "@/components/ScheduleShareOptionsDialog.vue";
import useIsSharedSchedule from "@/composables/useIsSharedSchedule";
import useFilterQuery from "@/composables/useFilterQuery";

const isSharedSchedule = useIsSharedSchedule();
const filterCriteria = useFilterQuery();

const scheduleShareOptionsDialogVisible = ref(false);
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
      <footer v-if="isSharedSchedule" class="hidden lg:flex justify-center lg:sticky bottom-0">
        <ShareViewFooter @click="scheduleShareOptionsDialogVisible = true" />
      </footer>
      <ScheduleShareOptionsDialog v-model:visible="scheduleShareOptionsDialogVisible" />
    </div>
  </AppRoot>
</template>
