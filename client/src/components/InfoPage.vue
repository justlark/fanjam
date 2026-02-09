<script setup lang="ts">
import { computed } from "vue";
import useRemoteData from "@/composables/useRemoteData";
import SimpleIcon from "./SimpleIcon.vue";
import LinksList from "./LinksList.vue";

const {
  data: { info, pages },
} = useRemoteData();

const websiteUrl = computed(() => {
  try {
    if (!info.value?.websiteUrl) return undefined;

    if (
      !info.value.websiteUrl.startsWith("http://") &&
      !info.value.websiteUrl.startsWith("https://")
    ) {
      return new URL(`https://${info.value.websiteUrl}`);
    } else {
      return new URL(info.value.websiteUrl);
    }
  } catch {
    return undefined;
  }
});

const conName = computed(() => info.value?.name || "FanJam");
</script>

<template>
  <div class="mt-8 text-center flex flex-col gap-10 px-6" data-testid="info-page">
    <div class="flex flex-col items-center gap-4">
      <h1 class="text-3xl" data-testid="info-page-name">{{ conName }}</h1>
      <span v-if="websiteUrl" class="flex gap-2 text-lg" data-testid="info-page-website">
        <SimpleIcon icon="box-arrow-up-right" label="External Link" />
        <a :href="websiteUrl.href" target="_blank" class="text-link-sm">
          {{ websiteUrl.hostname }}
        </a>
      </span>
      <p
        class="max-w-200 text-justify"
        v-if="info?.description"
        data-testid="info-page-description"
      >
        {{ info.description }}
      </p>
    </div>
    <div class="flex flex-col gap-2">
      <LinksList
        class="max-w-140 w-full mx-auto"
        v-if="info || pages"
        :links="info?.links ?? []"
        :files="info?.files ?? []"
        :pages="pages"
      />
    </div>
  </div>
</template>
