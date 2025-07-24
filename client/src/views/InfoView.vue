<script setup lang="ts">
import { computed } from "vue";
import { useRemoteInfo } from "@/composables/useEvents";
import SiteNav from "@/components/SiteNav";
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const { value: info } = useRemoteInfo();

const websiteUrlDomain = computed(() =>
  info.value?.about?.websiteUrl ? new URL(info.value.about.websiteUrl).hostname : undefined,
);
</script>

<template>
  <SiteNav>
    <div class="mt-8 text-center flex flex-col gap-8">
      <div v-if="info?.about" class="flex flex-col items-center gap-4">
        <h1 class="text-3xl">{{ info.about.name }}</h1>
        <span v-if="info.about.websiteUrl && websiteUrlDomain" class="flex gap-2 text-lg">
          <SimpleIcon icon="box-arrow-up-right" label="External Link" />
          <a
            :href="info.about.websiteUrl"
            target="_blank"
            class="text-primary underline underline-offset-2 hover:decoration-2"
          >
            {{ websiteUrlDomain }}
          </a>
        </span>
        <p v-if="info.about.description">{{ info.about.description }}</p>
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
