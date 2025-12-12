<script setup lang="ts">
import { ref, computed, useId } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import Divider from "primevue/divider";
import { RouterLink } from "vue-router";
import SimpleIcon from "./SimpleIcon.vue";
import Drawer from "primevue/drawer";
import MainMenu from "./MainMenu.vue";
import IconButton from "./IconButton.vue";
import AppUpdater from "./AppUpdater.vue";
import ScrollTop from "primevue/scrolltop";
import Toast from "primevue/toast";
import { useToast } from "primevue/usetoast";

const visible = ref(false);

const toast = useToast();

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};

const {
  data: { info },
  status: { info: infoStatus },
  reload,
} = useRemoteData();

const conName = computed(() => info.value?.name ?? "FanJam");

const refresh = async () => {
  let isDone = false;

  // Fetching the latest data from the server can either be quick (if it's
  // cached) or slow (if it needs to request it from NocoDB). If it's quick, we
  // want to avoid spamming the user with a double toast. So we show them this
  // toast only if fetching the data is taking a while. If the data is cached
  // and returns quickly, they'll only see the seconds toast, which will appear
  // near-instantaneously.
  setTimeout(() => {
    if (isDone) return;

    toast.add({
      severity: "info",
      summary: "Refreshing",
      detail: "Grabbing the latest schedule.",
      life: 1500,
    });
  }, 350);

  await reload();

  isDone = true;
  toast.add({ severity: "success", summary: "Done", detail: "You're all up to date!", life: 1500 });
};

const headerHeadingId = useId();
</script>

<template>
  <div class="flex flex-col h-screen">
    <div v-if="infoStatus === 'error'" class="flex flex-col justify-center items-center grow">
      <SimpleIcon
        icon="exclamation-circle"
        class="mb-4 text-8xl dark:text-red-200 flex justify-center items-center"
      />
      <span class="mb-1 text-2xl text-muted-color">Not found</span>
      <span class="text-lg text-muted-color">There is nothing here. Is this the right URL?</span>
    </div>
    <div v-else class="flex flex-col overflow-hidden grow">
      <header
        :aria-labelledby="headerHeadingId"
        class="shrink-0 sticky top-0 z-2 bg-surface-100 dark:bg-surface-900 flex flex-col"
      >
        <div class="h-16 flex items-center justify-between px-2 lg:px-4 gap-2">
          <div class="flex items-center gap-2">
            <span class="lg:hidden">
              <IconButton icon="list" label="Menu" @click="toggleMenuDrawer" />
            </span>
            <RouterLink :to="{ name: 'info' }">
              <h1 :id="headerHeadingId" class="text-xl lg:text-2xl" data-testid="site-nav-heading">
                {{ conName }}
              </h1>
            </RouterLink>
          </div>
          <IconButton icon="arrow-clockwise" label="Refresh" @click="refresh" />
        </div>
        <Drawer v-model:visible="visible" header="Menu" class="!w-60">
          <MainMenu />
        </Drawer>
        <Divider pt:root="!my-0" />
      </header>
      <div class="flex grow min-h-0">
        <div class="hidden lg:flex grow-0 shrink-0 items-stretch">
          <aside class="p-4 grow min-w-50">
            <MainMenu />
          </aside>
        </div>
        <div class="hidden lg:block">
          <Divider class="!ms-0" layout="vertical" />
        </div>
        <main class="overflow-y-auto grow min-h-0">
          <slot />
          <ScrollTop target="parent" />
        </main>
      </div>
    </div>
    <Toast position="bottom-center" />
    <AppUpdater />
  </div>
</template>
