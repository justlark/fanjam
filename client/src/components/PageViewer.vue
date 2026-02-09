<script setup lang="ts">
import { useId, watchEffect, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import ProgressSpinner from "primevue/progressspinner";
import LinksList from "@/components/LinksList.vue";
import IconButton from "@/components/IconButton.vue";
import { renderMarkdown } from "@/utils/markdown";

const route = useRoute();
const router = useRouter();

const {
  data: { pages },
  status: { pages: pagesStatus },
} = useRemoteData();

const pageId = computed(() => route.params.pageId as string);

const page = computed(() => {
  return pages.value.find((p) => p.id === pageId.value);
});

const bodyHtml = computed(() => {
  if (!page.value?.body) return undefined;
  return renderMarkdown(page.value.body);
});

const back = async () => {
  await router.push({
    name: "info",
  });
};

watchEffect(async () => {
  if (pagesStatus.value === "success" && pages.value.length > 0 && !page.value) {
    await back();
  }
});

const pageHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <article
      class="max-w-200 mx-auto"
      v-if="page"
      :aria-labelledby="pageHeadingId"
      data-testid="page-viewer"
    >
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton
          icon="chevron-left"
          label="Back"
          @click="back()"
          :button-props="{ 'data-testid': 'page-viewer-back-button' }"
        />
        <h2 :id="pageHeadingId" class="text-xl font-bold" data-testid="page-viewer-title">
          {{ page.title }}
        </h2>
      </div>
      <div class="px-6">
        <div
          id="document"
          v-if="bodyHtml && page?.body.trim() !== ''"
          v-html="bodyHtml"
          data-testid="page-viewer-body"
        ></div>
        <div
          v-else-if="page.files.length === 0"
          class="text-center text-lg italic text-muted-color mt-8"
          data-testid="page-viewer-no-details"
        >
          No details provided
        </div>
        <LinksList
          class="max-w-140 w-full mx-auto mt-6"
          v-if="page.files.length > 0"
          :links="[]"
          :files="[...page.files]"
          :pages="[]"
        />
      </div>
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
