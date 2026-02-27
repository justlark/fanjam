<script setup lang="ts">
import { computed } from "vue";
import Button from "primevue/button";
import LinkShareDialog from "./LinkShareDialog.vue";
import { useRoute } from "vue-router";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

defineEmits(["share-schedule"]);

const route = useRoute();

const pageLinkDescription = computed(() => {
  switch (route.name) {
    case "event":
      return "this event";
    case "announcement":
      return "this announcement";
    case "page":
      return "this page";
    default:
      return "this app";
  }
});

// Do not include the query params or fragment; users likely aren't intending
// to share their current search/filter params.
const appUrl = computed(() => window.location.origin + window.location.pathname);
</script>

<template>
  <LinkShareDialog
    v-model:visible="visible"
    :link="appUrl"
    title="Share"
    :toast-message="`A link to ${pageLinkDescription} has been copied to your clipboard.`"
    data-testid="share-dialog"
  >
    <template #header>
      <p data-testid="share-dialog-description">
        Send someone a link to {{ pageLinkDescription }}.
      </p>
    </template>
    <template #footer>
      <div class="flex justify-center items-center my-6">
        <div class="border-b w-8 mr-4"></div>
        <span>or</span>
        <div class="border-b w-8 ml-4"></div>
      </div>
      <div class="flex flex-col gap-2">
        <p>Share your schedule with a friend or move it to another device.</p>
        <Button
          data-testid="share-dialog-share-schedule-button"
          @click="$emit('share-schedule')"
          label="Share My Schedule"
          icon="bi bi-star"
          variant="outlined"
        />
      </div>
    </template>
  </LinkShareDialog>
</template>
