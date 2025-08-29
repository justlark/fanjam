<script setup lang="ts">
import { useId, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import ProgressSpinner from "primevue/progressspinner";
import * as commonmark from "commonmark";
import IconButton from "@/components/IconButton.vue";

const route = useRoute();
const router = useRouter();

const {
  data: { pages },
} = useRemoteData();

const pageId = computed(() => route.params.pageId as string);

const page = computed(() => {
  return pages.find((p) => p.id === pageId.value);
});

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const bodyHtml = computed(() => {
  if (!page.value?.body) return undefined;
  const parsed = mdReader.parse(page.value.body);
  return mdWriter.render(parsed);
});

const back = async () => {
  await router.push({
    name: "info",
  });
};

const pageHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <article class="max-w-200 mx-auto" v-if="page" :aria-labelledby="pageHeadingId">
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton icon="chevron-left" label="Back" @click="back()" />
        <h2 :id="pageHeadingId" class="text-xl font-bold">{{ page.title }}</h2>
      </div>
      <div class="px-6" id="document" v-if="bodyHtml" v-html="bodyHtml"></div>
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
