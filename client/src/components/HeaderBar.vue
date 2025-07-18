<script setup lang="ts">
import { ref } from "vue";
import Button from "primevue/button";
import MenuDrawer from "@/components/MenuDrawer.vue";
import Divider from "primevue/divider";

const props = defineProps<{
  title: string;
}>();

const visible = ref(false);

const toggleMenuDrawer = () => {
  visible.value = !visible.value;
};
</script>

<template>
  <header class="flex flex-col">
    <div class="flex items-center gap-3 p-2">
      <Button
        class="text-5xl"
        icon="bi bi-list"
        pt:icon="!text-2xl"
        variant="text"
        size="large"
        aria-label="Menu"
        rounded
        @click="toggleMenuDrawer"
      />
      <h1 class="text-2xl">{{ props.title }}</h1>
    </div>
    <MenuDrawer :title="props.title" v-model:visible="visible" />
    <Divider pt:root="!mt-0" />
  </header>
</template>
