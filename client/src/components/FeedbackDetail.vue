<script setup lang="ts">
import { computed } from "vue";
import useRemoteData from "@/composables/useRemoteData";

import SimpleIcon from "./SimpleIcon.vue";

const {
  data: { config },
} = useRemoteData();

const isSameDomain = computed(
  () =>
    config.value?.feedback_url &&
    new URL(config.value.feedback_url).origin === window.location.origin,
);
</script>

<template>
  <div v-if="config?.feedback_url" class="flex flex-col gap-1">
    <div>
      <a
        :href="config.feedback_url"
        class="text-primary flex items-center gap-1"
        :target="isSameDomain ? '_self' : '_blank'"
      >
        <SimpleIcon icon="hand-thumbs-up" class="text-lg" />
        <span class="font-bold decoration-2 underline underline-offset-2 hover:decoration-3">
          Feedback?
        </span>
      </a>
    </div>
    <div class="leading-4">
      <small class="text-muted-color">Bugs? Requests? Let us know what you think.</small>
    </div>
  </div>
</template>
