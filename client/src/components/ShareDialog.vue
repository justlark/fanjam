<script setup lang="ts">
import { computed } from "vue";
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import { useRoute } from "vue-router";
import { useToast } from "primevue/usetoast";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

defineEmits(["share-schedule"]);

const route = useRoute();
const toast = useToast();

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

const copyAppUrl = async () => {
  await navigator.clipboard.writeText(appUrl.value);

  toast.add({
    severity: "info",
    summary: "Link Copied",
    detail: `A link to ${pageLinkDescription.value} has been copied to your clipboard.`,
    life: 1500,
  });
};
</script>

<template>
  <Dialog class="max-w-100 m-4" v-model:visible="visible" modal dismissable-mask header="Share">
    <div class="flex flex-col gap-4 w-full justify-stretch">
      <div class="flex flex-col gap-2">
        <p>Send someone a link to {{ pageLinkDescription }}.</p>
        <InputGroup>
          <InputGroupAddon>
            <Button @click="copyAppUrl" label="Copy" icon="bi bi-clipboard" />
          </InputGroupAddon>
          <InputText :value="appUrl" disabled />
        </InputGroup>
      </div>
      <div class="flex justify-center items-center">
        <div class="border-b w-8 mr-4"></div>
        <span>or</span>
        <div class="border-b w-8 ml-4"></div>
      </div>
      <div class="flex flex-col gap-2">
        <p>Share your schedule with a friend or move it to another device.</p>
        <Button @click="$emit('share-schedule')" label="Share My Schedule" icon="bi bi-star" />
      </div>
    </div>
  </Dialog>
</template>
