<script setup lang="ts">
import { computed } from "vue";
import useRemoteData from "@/composables/useRemoteData";

import SimpleIcon from "./SimpleIcon.vue";

const {
  data: { config },
} = useRemoteData();

const isSameDomain = computed(
  () =>
    config.value?.feedbackUrl &&
    new URL(config.value.feedbackUrl).origin === window.location.origin,
);

const feedbackInfo = computed(() => {
  if (
    !(config.value?.useFeedback ?? false) ||
    config.value?.feedbackIcon === undefined ||
    config.value.feedbackTitle === undefined ||
    config.value.feedbackDetail === undefined ||
    config.value.feedbackUrl === undefined
  ) {
    return undefined;
  }

  return {
    icon: config.value.feedbackIcon,
    title: config.value.feedbackTitle,
    detail: config.value.feedbackDetail,
    url: config.value.feedbackUrl,
  };
});
</script>

<template>
  <div v-if="feedbackInfo" class="flex flex-col gap-1">
    <div>
      <a
        :href="feedbackInfo.url"
        class="text-primary flex items-center gap-1"
        :target="isSameDomain ? '_self' : '_blank'"
      >
        <SimpleIcon :icon="feedbackInfo.icon" class="text-lg" />
        <span class="text-link-lg">{{ feedbackInfo.title }}</span>
      </a>
    </div>
    <div class="leading-4">
      <small class="text-muted-color">{{ feedbackInfo.detail }}</small>
    </div>
  </div>
</template>
