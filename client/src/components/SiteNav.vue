<script setup lang="ts">
import { ref, computed, watch, useId } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import useIsSharedSchedule from "@/composables/useIsSharedSchedule";
import useOnline from "@/composables/useOnline";
import Divider from "primevue/divider";
import { RouterLink, useRoute, useRouter } from "vue-router";
import SimpleIcon from "./SimpleIcon.vue";
import Drawer from "primevue/drawer";
import MainMenu from "./MainMenu.vue";
import IconButton from "./IconButton.vue";
import ShareViewFooter from "./ShareViewFooter.vue";
import AppUpdater from "./AppUpdater.vue";
import PushNotificationPrompter from "./PushNotificationPrompter.vue";
import SiteAttribution from "./SiteAttribution.vue";
import FeedbackCallout from "./FeedbackCallout.vue";
import Toast from "primevue/toast";
import ScrollTop from "primevue/scrolltop";
import ScheduleShareOptionsDialog from "./ScheduleShareOptionsDialog.vue";
import useUnreadAnnouncements from "@/composables/useUnreadAnnouncements";
import { TOAST_TTL_SHORT, TOAST_TTL_MEDIUM, TOAST_TTL_LONG } from "@/utils/toast";
import { checkForAppUpdate } from "@/utils/appUpdate";
import { useToast } from "primevue/usetoast";

const menuVisible = ref(false);
const scheduleShareOptionsDialogVisible = ref(false);

const toast = useToast();
const route = useRoute();
const router = useRouter();
const unreadAnnouncements = useUnreadAnnouncements();
const isSharedSchedule = useIsSharedSchedule();

const hasUnreadAnnouncements = computed(() => unreadAnnouncements.value.size > 0);
const showNotificationBadge = computed(
  () =>
    route.name !== "announcement" && route.name !== "announcements" && hasUnreadAnnouncements.value,
);

const toggleMenuDrawer = () => {
  menuVisible.value = !menuVisible.value;
};

// Nav menu links don't necessarily change the active route path; they might
// just change query params. In this case, the menu drawer doesn't close on its
// own, so we need to watch for changes in all parts of the route to make sure
// the menu drawer closes.
watch(route, () => {
  menuVisible.value = false;
});

// When the user follows a schedule sync link, the router will append a query
// param specifically to tell this component to show a toast. Once we show the
// toast, we strip the query param.
watch(
  () => route.query.synced,
  async (synced) => {
    if (synced !== "true") return;

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { synced: _, ...query } = route.query;
    await router.replace({ query });

    toast.add({
      severity: "success",
      summary: "Syncing your schedule",
      detail: "Your schedule will now stay in sync on this device.",
      life: TOAST_TTL_LONG,
    });
  },
  { immediate: true },
);

const {
  data: { info },
  status: { info: infoStatus },
  reloadAll,
} = useRemoteData();

const isOnline = useOnline();

const conName = computed(() => info.value?.name ?? "FanJam");

const refresh = async () => {
  if (!isOnline.value) {
    toast.add({
      severity: "warn",
      summary: "You're offline",
      detail: "You can keep using the app.",
      life: TOAST_TTL_MEDIUM,
    });

    return;
  }

  toast.add({
    severity: "info",
    summary: "Refreshing",
    detail: "Grabbing the latest schedule.",
    life: TOAST_TTL_SHORT,
  });

  // We do not await this alongside `reloadAll()` below, because we do not
  // necessarily want to show a toast if this fails.
  void checkForAppUpdate();

  try {
    await reloadAll();
  } catch {
    toast.add({
      severity: "error",
      summary: "Couldn't refresh",
      detail: "Something went wrong. You can keep using the app.",
      life: TOAST_TTL_MEDIUM,
    });
  }
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
    <div
      v-else-if="infoStatus === 'offline'"
      class="flex flex-col justify-center items-center grow text-center px-6"
      data-testid="site-nav-offline-state"
    >
      <SimpleIcon
        icon="cloud-slash"
        class="mb-4 text-8xl text-muted-color flex justify-center items-center"
      />
      <span class="mb-1 text-2xl text-muted-color">You're offline</span>
      <span class="text-lg text-muted-color">
        Connect to the internet once to load this con. After that it works offline.
      </span>
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
          <div class="flex items-center gap-3">
            <SimpleIcon
              v-if="!isOnline"
              icon="cloud-slash"
              label="Offline"
              class="text-3xl text-muted-color"
              data-testid="site-nav-offline"
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
          v-model:visible="menuVisible"
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
          <Divider class="!ms-0 !me-[1px]" layout="vertical" />
        </div>
        <main class="grow">
          <slot />
        </main>
      </div>
      <footer
        v-if="isSharedSchedule && (route.name === 'schedule' || route.name === 'event')"
        class="flex justify-center sticky bottom-0 lg:hidden"
      >
        <ShareViewFooter @click="scheduleShareOptionsDialogVisible = true" />
      </footer>
      <ScrollTop class="lg:hidden" />
    </div>
    <ScheduleShareOptionsDialog v-model:visible="scheduleShareOptionsDialogVisible" />
    <Toast position="bottom-center" />
    <AppUpdater />
    <PushNotificationPrompter />
  </div>
</template>
