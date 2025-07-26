<script setup lang="ts">
import LinkButton from "@/components/system/LinkButton.vue";

const props = defineProps<{
  links: Array<{ name: string; url: string }>;
  files: Array<{ name: string; mediaType: string; signedUrl: string }>;
  pages: Array<{
    id: string;
    title: string;
    body: string;
  }>;
}>();
</script>

<template>
  <ul class="flex flex-col items-center gap-2">
    <li v-for="(link, index) in props.links" :key="index" class="flex gap-2 w-full">
      <LinkButton
        :name="link.name"
        :target="{ kind: 'external', url: link.url }"
        icon="box-arrow-up-right"
        icon-label="External Link"
      />
    </li>
    <li v-for="(file, index) in props.files" :key="index" class="flex gap-2 w-full">
      <LinkButton
        :name="file.name"
        :target="{
          kind: 'file',
          url: file.signedUrl,
          mediaType: file.mediaType,
        }"
        icon="download"
        icon-label="Download"
      />
    </li>
    <li v-for="(page, index) in props.pages" :key="index" class="flex gap-2 w-full">
      <LinkButton
        :name="page.title"
        :target="{
          kind: 'route',
          to: {
            name: 'page',
            params: { pageId: page.id },
          },
        }"
        icon="file-text"
        icon-label="Page"
      />
    </li>
  </ul>
</template>
