<script setup lang="ts">
import ExternalLink from "@/components/system/ExternalLink.vue";

const props = defineProps<{
  links: Array<{ name: string; url: string }>;
}>();
</script>

<template>
  <ul class="flex flex-col items-center gap-2 px-6">
    <li v-for="link in props.links" :key="link.url" class="flex gap-2 w-full">
      <ExternalLink :url="link.url" :name="link.name" />
    </li>
  </ul>
</template>
