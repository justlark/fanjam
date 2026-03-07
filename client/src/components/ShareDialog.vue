<script setup lang="ts">
import { computed } from "vue";
import LinkShareDialog from "./LinkShareDialog.vue";
import { useRoute } from "vue-router";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

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
    title="Share"
    :link="appUrl"
    :message="`Send someone a link to ${pageLinkDescription}.`"
    :toast-message="`A link to ${pageLinkDescription} has been copied to your clipboard.`"
    data-testid="share-dialog"
  >
  </LinkShareDialog>
</template>
