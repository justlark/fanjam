<script setup lang="ts">
import ExternalLink from "@/components/system/ExternalLink.vue";

const props = defineProps<{
  files: Array<{ name: string; mediaType: string; signedUrl: string }>;
}>();
</script>

<template>
  <ul class="flex flex-col items-center gap-2 px-6">
    <li v-for="(file, index) in props.files" :key="index" class="flex gap-2 w-full">
      <ExternalLink
        :url="file.signedUrl"
        :name="file.name"
        :media-type="file.mediaType"
        icon="download"
        icon-label="Download"
      />
    </li>
  </ul>
</template>
