<script setup lang="ts">
import { computed } from "vue";
import SiteNav from "@/components/SiteNav";
import useEvents from "@/composables/useEvents";
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const { about, links } = useEvents();

const websiteUrlDomain = computed(() =>
  about.value?.websiteUrl ? new URL(about.value.websiteUrl).hostname : undefined,
);
</script>

<template>
  <SiteNav>
    <div class="mt-8 text-center flex flex-col gap-8">
      <div class="flex flex-col items-center gap-4">
        <h1 v-if="about?.name" class="text-3xl">{{ about.name }}</h1>
        <span v-if="about?.websiteUrl && websiteUrlDomain" class="flex gap-2 text-lg">
          <SimpleIcon icon="box-arrow-up-right" label="External Link" />
          <a
            :href="about.websiteUrl"
            target="_blank"
            class="text-primary underline underline-offset-2 hover:decoration-2"
          >
            {{ websiteUrlDomain }}
          </a>
        </span>
        <p v-if="about?.description">{{ about.description }}</p>
      </div>
      <ul v-if="links.length > 0" class="flex flex-col items-center text-lg gap-1">
        <li v-for="link in links" :key="link.url" class="flex gap-2">
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
