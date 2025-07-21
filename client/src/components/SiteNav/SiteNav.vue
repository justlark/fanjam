<script setup lang="ts">
import { ref, useId } from "vue";
import useEvents from "@/composables/useEvents";
import Divider from "primevue/divider";
import Drawer from "primevue/drawer";
import MainMenu from "./MainMenu.vue";
import IconButton from "@/components/system/IconButton.vue";

const props = defineProps<{
  title: string;
}>();

const visible = ref(false);

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};

const { reload: reloadEvents } = useEvents();

const headerHeadingId = useId();
</script>

<template>
  <div class="flex flex-col min-h-[100vh]">
    <header :aria-labelledby="headerHeadingId" class="flex flex-col">
      <div class="flex items-center justify-between p-2 lg:p-4 gap-2">
        <div class="flex items-center gap-2">
          <span class="lg:hidden">
            <IconButton icon="list" label="Menu" @click="toggleMenuDrawer" />
          </span>
          <h1 :id="headerHeadingId" class="text-2xl">{{ props.title }}</h1>
        </div>
        <IconButton
          class="lg:!hidden"
          icon="arrow-clockwise"
          label="Refresh"
          @click="reloadEvents"
        />
      </div>
      <Drawer v-model:visible="visible" :header="props.title">
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
</template>
