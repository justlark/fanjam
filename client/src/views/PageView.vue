<script setup lang="ts">
import { useId, computed, watchEffect } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import * as commonmark from "commonmark";
import SiteNav from "@/components/SiteNav";
import IconButton from "@/components/system/IconButton.vue";

const route = useRoute();
const router = useRouter();

const {
  data: { pages },
} = useRemoteData();

const pageId = computed(() => route.params.pageId as string);

const page = computed(() => {
  return pages.value.find((p) => p.id === pageId.value);
});

watchEffect(async () => {
  // If the page does not (or no longer) exists, redirect to the info view.
  if (!page.value) {
    await router.replace({
      name: "info",
    });
  }
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
  <SiteNav>
    <article class="max-w-200 mx-auto" v-if="page" :aria-labelledby="pageHeadingId">
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton icon="chevron-left" label="Back" @click="back()" />
        <h2 :id="pageHeadingId" class="text-xl font-bold">{{ page.title }}</h2>
      </div>
      <div class="px-6" id="document" v-if="bodyHtml" v-html="bodyHtml"></div>
    </article>
  </SiteNav>
</template>
