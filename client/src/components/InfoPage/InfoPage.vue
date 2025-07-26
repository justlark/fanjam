<script setup lang="ts">
import { computed } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import SimpleIcon from "@/components/system/SimpleIcon.vue";
import LinksList from "./LinksList.vue";
import FilesList from "./FilesList.vue";

const {
  data: { info },
} = useRemoteData();

const websiteUrlDomain = computed(() =>
  info.value?.websiteUrl ? new URL(info.value.websiteUrl).hostname : undefined,
);
</script>

<template>
  <div class="mt-8 text-center flex flex-col gap-10">
    <div
      v-if="info?.name || info?.websiteUrl || info?.description"
      class="flex flex-col items-center gap-4"
    >
      <h1 class="text-3xl">{{ info.name }}</h1>
      <span v-if="info.websiteUrl && websiteUrlDomain" class="flex gap-2 text-lg">
        <SimpleIcon icon="box-arrow-up-right" label="External Link" />
        <a
          :href="info.websiteUrl"
          target="_blank"
          class="text-primary underline underline-offset-2 hover:decoration-2"
        >
          {{ websiteUrlDomain }}
        </a>
      </span>
      <p v-if="info.description">{{ info.description }}</p>
    </div>
    <div class="flex flex-col gap-2">
      <LinksList
        class="max-w-140 w-full mx-auto"
        v-if="info && info.links.length > 0"
        :links="info.links"
      />
      <FilesList
        class="max-w-140 w-full mx-auto"
        v-if="info && info.files.length > 0"
        :files="info.files"
      />
    </div>
  </div>
</template>
