<script setup lang="ts">
import { ref } from "vue";
import Button from "primevue/button";
import Divider from "primevue/divider";
import Drawer from "primevue/drawer";
import MainMenu from "@/components/MainMenu.vue";

const props = defineProps<{
  title: string;
}>();

const visible = ref(false);

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};
</script>

<template>
  <div class="flex flex-col">
    <header class="flex flex-col">
      <div class="flex items-center p-2 lg:p-4 gap-2">
        <Button
          class="lg:!hidden"
          icon="bi bi-list"
          pt:icon="!text-3xl"
          variant="text"
          size="large"
          aria-label="Menu"
          rounded
          @click="toggleMenuDrawer"
        />
        <h1 class="text-2xl">{{ props.title }}</h1>
      </div>
      <Drawer v-model:visible="visible" :header="props.title">
        <MainMenu />
      </Drawer>
      <Divider pt:root="!my-0" />
    </header>
    <div class="flex">
      <div class="hidden lg:flex grow items-stretch min-h-[100vh] max-w-60">
        <aside class="p-4 grow">
          <MainMenu />
        </aside>
        <Divider pt:root="!ms-0" layout="vertical" />
      </div>
      <div class="grow">
        <slot />
      </div>
    </div>
  </div>
</template>
