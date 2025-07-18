<script setup lang="ts">
import Button from "primevue/button";
import MenuDrawer from "@/components/MenuDrawer.vue";
</script>

<template>
  <main class="flex flex-col justify-center items-center gap-8">
    <h1 class="text-4xl text-center mt-4">FanJam</h1>
    <MenuDrawer title="Menu" :visible="true" />
    <Button label="Example" />
  </main>
</template>
