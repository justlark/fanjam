<script setup lang="ts">
import { computed, watchEffect } from "vue";
import { useRoute, useRouter } from "vue-router";
import useRemoteData from "@/composables/useRemoteData";
import ProgressSpinner from "primevue/progressspinner";

const route = useRoute();
const router = useRouter();

const {
  data: { vendors, vendorPages, config },
  status: { vendorPages: vendorPagesStatus },
} = useRemoteData();

const vendorPageId = computed(() => route.params.vendorPageId as string);
const vendorPage = computed(() => vendorPages.value.find((page) => page.id === vendorPageId.value));
const currentVendors = computed(() =>
  vendors.value.filter((vendor) => vendorPage.value && vendor.page?.id === vendorPage.value.id),
);

watchEffect(async () => {
  if (
    config.value?.useVendorPages === false ||
    (vendorPagesStatus.value === "success" && vendorPages.value.length > 0 && !vendorPage.value)
  ) {
    await router.push({ name: "schedule" });
  }
});
</script>

<template>
  <div class="h-full">
    <div v-if="vendorPage" class="mt-8 flex flex-col gap-10 px-6" data-testid="vendor-page">
      <div class="flex flex-col items-center gap-4">
        <h1 class="text-3xl" data-testid="vendor-page-name">
          {{ vendorPage.title }}
        </h1>
        <p
          class="max-w-200 text-justify"
          v-if="vendorPage.description"
          data-testid="vendor-page-description"
        >
          {{ vendorPage.description }}
        </p>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mx-auto w-full max-w-300">
        <RouterLink
          :to="{ name: 'vendor', params: { vendorId: vendor.id } }"
          v-for="vendor in currentVendors"
          :key="vendor.id"
          class="group w-full block cursor-pointer px-4 py-2 border border-surface-300 hover:bg-surface-200 dark:border-surface-600 hover:dark:bg-surface-800 rounded-sm"
          data-testid="vendor-page-vendor"
        >
          <h2 class="text-xl font-bold text-link-lg group-hover:!decoration-3">
            {{ vendor.name }}
          </h2>
          <p class="line-clamp-5">{{ vendor.summary }}</p>
        </RouterLink>
      </div>
    </div>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
