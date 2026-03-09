<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import LinkShareDialog from "@/components/LinkShareDialog.vue";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const route = useRoute();
const envId = computed(() => route.params.envId as string);
const appUrl = computed(() => `${window.location.origin}/app/${envId.value}`);
</script>

<template>
  <LinkShareDialog
    v-model:visible="visible"
    :link="appUrl"
    title="Share This App"
    message="Send someone a link to this app."
    toast-message="A link to this app has been copied to your clipboard."
    data-testid="app-share-dialog"
  />
</template>
