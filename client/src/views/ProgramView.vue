<script setup lang="ts">
import SiteNav from "@/components/SiteNav.vue";
import EventProgram from "@/components/EventProgram.vue";
</script>

<template>
  <SiteNav>
    <div class="flex justify-center h-full">
      <EventProgram class="p-6 grow max-w-240" />
    </div>
  </SiteNav>
</template>
