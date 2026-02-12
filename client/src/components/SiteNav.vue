<script setup lang="ts">
import { ref, computed, watch, useId } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import Divider from "primevue/divider";
import { RouterLink, useRoute } from "vue-router";
import SimpleIcon from "./SimpleIcon.vue";
import Drawer from "primevue/drawer";
import MainMenu from "./MainMenu.vue";
import IconButton from "./IconButton.vue";
import AppUpdater from "./AppUpdater.vue";
import SiteAttribution from "./SiteAttribution.vue";
import FeedbackCallout from "./FeedbackCallout.vue";
import Toast from "primevue/toast";
import ScrollTop from "primevue/scrolltop";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";
import { useToast } from "primevue/usetoast";

const visible = ref(false);

const toast = useToast();
const route = useRoute();
const unreadAnnouncements = useUnreadAnnouncements();

const hasUnreadAnnouncements = computed(() => unreadAnnouncements.value.size > 0);
const showNotificationBadge = computed(
  () =>
    route.name !== "announcement" && route.name !== "announcements" && hasUnreadAnnouncements.value,
);

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};

// Nav menu links don't necessarily change the active route path; they might
// just change query params. In this case, the menu drawer doesn't close on its
// own, so we need to watch for changes in all parts of the route to make sure
// the menu drawer closes.
watch(route, () => {
  visible.value = false;
});

const {
  data: { info },
  status: { info: infoStatus },
  reload,
} = useRemoteData();

const conName = computed(() => info.value?.name ?? "FanJam");

const copyLink = async () => {
  // Do not include the query params or fragment; users likely aren't intending
  // to share their current search/filter params.
  await navigator.clipboard.writeText(window.location.origin + window.location.pathname);

  toast.add({
    severity: "info",
    summary: "Link Copied",
    detail: "A link to this page has been copied to your clipboard.",
    life: 1500,
  });
};

const refresh = async () => {
  toast.add({
    severity: "info",
    summary: "Refreshing",
    detail: "Grabbing the latest schedule.",
    life: 1500,
  });

  await reload();
};

const headerHeadingId = useId();
</script>

<template>
  <div class="flex flex-col h-dvh">
    <div
      v-if="infoStatus === 'error'"
      class="flex flex-col justify-center items-center grow"
      data-testid="site-nav-error-state"
    >
      <SimpleIcon
        icon="exclamation-circle"
        class="mb-4 text-8xl dark:text-red-200 flex justify-center items-center"
      />
      <span class="mb-1 text-2xl text-muted-color">Not found</span>
      <span class="text-lg text-muted-color">There is nothing here. Is this the right URL?</span>
    </div>
    <div v-else class="flex flex-col grow">
      <header :aria-labelledby="headerHeadingId" class="sticky top-0 z-2 bg-color flex flex-col">
        <div class="h-16 flex items-center justify-between px-2 lg:px-4 gap-2">
          <div class="flex items-center gap-2">
            <span class="lg:hidden">
              <IconButton
                icon="list"
                label="Menu"
                @click="toggleMenuDrawer"
                :badge="showNotificationBadge"
                :button-props="{
                  'data-testid': 'main-menu-button',
                }"
              />
            </span>
            <RouterLink :to="{ name: 'schedule' }">
              <h1 :id="headerHeadingId" class="text-xl lg:text-2xl" data-testid="site-nav-heading">
                {{ conName }}
              </h1>
            </RouterLink>
          </div>
          <div class="flex lg:gap-2">
            <IconButton
              icon="link-45deg"
              label="Copy Link"
              @click="copyLink"
              :button-props="{ 'data-testid': 'site-nav-copy-link' }"
            />
            <IconButton
              icon="arrow-clockwise"
              label="Refresh"
              @click="refresh"
              :button-props="{ 'data-testid': 'site-nav-refresh' }"
            />
          </div>
        </div>
        <Drawer
          v-model:visible="visible"
          header="Menu"
          :block-scroll="true"
          class="!w-65"
          pt:content:data-testid="main-menu-drawer"
        >
          <div class="h-full flex flex-col justify-between">
            <div class="flex flex-col gap-4">
              <MainMenu />
              <FeedbackCallout />
            </div>
            <SiteAttribution />
          </div>
        </Drawer>
        <Divider pt:root="!my-0" />
      </header>
      <div class="flex grow">
        <div class="hidden lg:flex sticky top-0 grow-0 shrink-0 items-stretch">
          <aside
            class="px-4 grow min-w-50 flex flex-col justify-between"
            data-testid="main-menu-sidebar"
          >
            <div class="sticky top-16 pt-4">
              <div class="flex flex-col gap-4">
                <MainMenu />
                <FeedbackCallout />
              </div>
            </div>
            <div class="sticky bottom-0 pb-4">
              <SiteAttribution />
            </div>
          </aside>
        </div>
        <div class="hidden lg:block">
          <Divider class="!ms-0" layout="vertical" />
        </div>
        <main class="grow">
          <slot />
        </main>
      </div>
      <ScrollTop class="lg:hidden" />
    </div>
    <Toast position="bottom-center" />
    <AppUpdater />
  </div>
</template>
