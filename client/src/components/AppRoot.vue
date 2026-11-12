<script setup lang="ts">
import AppNav from "./AppNav.vue";
import { provideRemoteData } from "@/composables/useRemoteData";

provideRemoteData();
</script>

<template>
  <AppNav>
    <slot />
  </AppNav>
</template>
