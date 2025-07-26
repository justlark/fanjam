<script setup lang="ts">
import ExternalLink from "@/components/system/ExternalLink.vue";

const props = defineProps<{
  links: Array<{ name: string; url: string }>;
}>();
</script>

<template>
  <ul class="flex flex-col items-center gap-2 px-6">
    <li v-for="(link, index) in props.links" :key="index" class="flex gap-2 w-full">
      <ExternalLink
        :url="link.url"
        :name="link.name"
        icon="box-arrow-up-right"
        icon-label="External Link"
      />
    </li>
  </ul>
</template>
