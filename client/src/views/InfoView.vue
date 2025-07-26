<script setup lang="ts">
import { computed } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import SiteNav from "@/components/SiteNav";
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const {
  data: { info },
} = useRemoteData();

const websiteUrlDomain = computed(() =>
  info.value?.websiteUrl ? new URL(info.value.websiteUrl).hostname : undefined,
);
</script>

<template>
  <SiteNav>
    <div class="mt-8 text-center flex flex-col gap-8">
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
      <ul v-if="info && info.links.length > 0" class="flex flex-col items-center text-lg gap-1">
        <li v-for="link in info.links" :key="link.url" class="flex gap-2">
          <SimpleIcon icon="box-arrow-up-right" label="External Link" />
          <a
            :href="link.url"
            target="_blank"
            class="text-primary underline underline-offset-2 hover:decoration-2"
          >
            {{ link.name }}
          </a>
        </li>
      </ul>
    </div>
  </SiteNav>
</template>
