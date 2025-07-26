<script setup lang="ts">
import { ref, computed, useId } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import usePageMeta from "@/composables/usePageMeta";
import Divider from "primevue/divider";
import SimpleIcon from "@/components/system/SimpleIcon.vue";
import Drawer from "primevue/drawer";
import MainMenu from "./MainMenu.vue";
import IconButton from "@/components/system/IconButton.vue";
import AppUpdater from "@/components/SiteNav/AppUpdater.vue";
import Toast from "primevue/toast";
import ProgressSpinner from "primevue/progressspinner";
import { useToast } from "primevue/usetoast";

usePageMeta();

const visible = ref(false);

const toast = useToast();

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};

const {
  data: { info },
  reload,
  isPending,
  isNotFound,
} = useRemoteData();

const conName = computed(() => info.value?.name ?? "FanJam");

const refresh = async () => {
  toast.add({
    severity: "info",
    summary: "Refreshing",
    detail: "Grabbing the latest schedule.",
    life: 1500,
  });

  await reload();

  toast.add({ severity: "success", summary: "Done", detail: "You're all up to date!", life: 1500 });
};

const headerHeadingId = useId();
</script>

<template>
  <div class="flex flex-col min-h-[100vh]">
    <div v-if="isNotFound" class="flex flex-col justify-center items-center grow">
      <SimpleIcon
        icon="exclamation-circle"
        class="mb-4 text-8xl dark:text-red-200 flex justify-center items-center"
      />
      <span class="mb-1 text-2xl text-muted-color">Not found</span>
      <span class="text-lg text-muted-color">There is nothing here. Is this the right URL?</span>
    </div>
    <div v-else-if="isPending" class="flex flex-col justify-center items-center gap-4 grow">
      <ProgressSpinner />
      <div class="text-xl text-muted-color">Loading…</div>
    </div>
    <div v-else class="flex flex-col grow">
      <header :aria-labelledby="headerHeadingId" class="flex flex-col">
        <div class="flex items-center justify-between p-2 lg:px-4 gap-2">
          <div class="flex items-center gap-2">
            <span class="lg:hidden">
              <IconButton icon="list" label="Menu" @click="toggleMenuDrawer" />
            </span>
            <h1 :id="headerHeadingId" class="text-xl lg:text-2xl">{{ conName }}</h1>
          </div>
          <IconButton icon="arrow-clockwise" label="Refresh" @click="refresh" />
        </div>
        <Drawer v-model:visible="visible" :header="conName">
          <MainMenu />
        </Drawer>
        <Divider pt:root="!my-0" />
      </header>
      <div class="flex grow">
        <div class="hidden lg:flex grow-0 shrink-0 items-stretch">
          <aside class="p-4 grow min-w-50">
            <MainMenu />
          </aside>
          <Divider pt:root="!ms-0" layout="vertical" />
        </div>
        <main class="grow">
          <slot />
        </main>
      </div>
    </div>
    <Toast position="bottom-center" />
    <AppUpdater />
  </div>
</template>
