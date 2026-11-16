<script setup lang="ts">
import { computed, watchEffect, useId } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import { renderMarkdown } from "@/utils/markdown";
import IconButton from "@/components/IconButton.vue";
import SimpleIcon from "./SimpleIcon.vue";
import ProgressSpinner from "primevue/progressspinner";

const route = useRoute();
const router = useRouter();

const {
  data: { vendors, config },
  status: { vendors: vendorsStatus },
} = useRemoteData();

const vendorId = computed(() => route.params.vendorId as string);
const vendor = computed(() => vendors.value.find((vendor) => vendor.id === vendorId.value));

const descriptionHtml = computed(() => {
  if (!vendor.value?.description) return undefined;
  return renderMarkdown(vendor.value.description);
});
// The server adds a scheme when one is missing, but passes anything that already
// has one through unchecked, so this may still fail to parse.
const websiteUrl = computed(() => {
  if (!vendor.value?.websiteUrl) return undefined;

  try {
    return new URL(vendor.value.websiteUrl);
  } catch {
    return undefined;
  }
});

const back = async () => {
  if (vendor.value?.page?.id) {
    await router.push({
      name: "vendor-page",
      params: { vendorPageId: vendor.value.page.id },
    });

    return;
  }

  await router.push({
    name: "schedule",
  });
};

watchEffect(async () => {
  if (
    config.value?.useVendorPages === false ||
    (vendorsStatus.value === "success" && vendors.value.length > 0 && !vendor.value)
  ) {
    await router.push({
      name: "schedule",
    });
  }
});

const vendorHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <article
      class="max-w-200 mx-auto"
      v-if="vendor"
      :aria-labelledby="vendorHeadingId"
      data-testid="vendor-viewer"
    >
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton
          icon="chevron-left"
          label="Back"
          @click="back()"
          :button-props="{ 'data-testid': 'vendor-viewer-back-button' }"
        />
        <h2 :id="vendorHeadingId" class="text-xl font-bold" data-testid="vendor-viewer-title">
          {{ vendor.name }}
        </h2>
      </div>
      <span
        v-if="websiteUrl"
        class="mx-auto flex gap-2 justify-center text-lg"
        data-testid="vendor-viewer-website"
      >
        <SimpleIcon icon="box-arrow-up-right" label="External Link" />
        <a :href="websiteUrl.href" target="_blank" class="text-link-sm">
          {{ websiteUrl.hostname }}
        </a>
      </span>
      <div class="px-6">
        <div
          id="document"
          v-if="descriptionHtml && vendor?.description?.trim() !== ''"
          v-html="descriptionHtml"
          data-testid="vendor-viewer-body"
        ></div>
        <p
          v-else-if="vendor?.summary && vendor.summary.trim() !== ''"
          data-testid="vendor-viewer-body"
        >
          {{ vendor.summary }}
        </p>
        <div
          v-else
          class="text-center text-lg italic text-muted-color mt-8"
          data-testid="vendor-viewer-no-details"
        >
          No details provided
        </div>
      </div>
    </article>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
