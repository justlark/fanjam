<script setup lang="ts">
import SiteNav from "./SiteNav.vue";
import { provideRemoteData } from "@/composables/useRemoteData";

provideRemoteData();
</script>

<template>
  <SiteNav>
    <slot />
  </SiteNav>
</template>
