<script setup lang="ts">
import { computed } from "vue";
import useStarredEvents from "@/composables/useStarredEvents";
import { encodeBase64url } from "@/utils/encoding";
import LinkShareDialog from "@/components/LinkShareDialog.vue";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const starredEvents = useStarredEvents();

const shareUrl = computed(() => {
  const starredEventIds = [...starredEvents.value];
  starredEventIds.sort();
  return `${window.location.origin}/share/?s=${encodeBase64url(starredEventIds.join(","))}`;
});
</script>

<template>
  <LinkShareDialog
    v-model:visible="visible"
    :link="shareUrl"
    toast-message="Share this URL to share your schedule."
  >
    Share your schedule with a friend or move it to another device.
  </LinkShareDialog>
</template>
