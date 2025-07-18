<script setup lang="ts">
import Drawer from "primevue/drawer";
import MenuDrawerItem from "@/components/MenuDrawerItem.vue";

const props = defineProps<{
  title: string;
}>();

const visible = defineModel<boolean>();
</script>

<template>
  <Drawer v-model:visible="visible" :header="props.title">
    <nav class="flex flex-col gap-3">
      <MenuDrawerItem icon="bi bi-calendar-event" label="Schedule" :to="{ name: 'schedule' }" />
      <MenuDrawerItem icon="bi bi-info-circle" label="Info" :to="{ name: 'info' }" />
    </nav>
  </Drawer>
</template>
