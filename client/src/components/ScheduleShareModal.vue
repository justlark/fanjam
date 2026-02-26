<script setup lang="ts">
import { computed } from "vue";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import Button from "primevue/button";
import QrcodeVue from "qrcode.vue";
import useStarredEvents from "@/composables/useStarredEvents";
import { useToast } from "primevue/usetoast";
import { encodeBase64url } from "@/utils/encoding";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const starredEvents = useStarredEvents();
const toast = useToast();

const shareUrl = computed(() => {
  const starredEventIds = [...starredEvents.value];
  starredEventIds.sort();
  return `${window.location.origin}/share/?s=${encodeBase64url(starredEventIds.join(","))}`;
});

const copyShareUrl = async () => {
  await navigator.clipboard.writeText(shareUrl.value);

  toast.add({
    severity: "info",
    summary: "Link Copied",
    detail: "Share this URL to share your schedule.",
    life: 1500,
  });
};
</script>

<template>
  <Dialog
    class="max-w-100 m-4"
    v-model:visible="visible"
    modal
    dismissable-mask
    header="Share Schedule"
  >
    <p>Share your schedule with a friend or move it to another device.</p>
    <div class="rounded-3xl overflow-hidden mx-auto w-fit my-6 shadow-md">
      <QrcodeVue
        :value="shareUrl"
        :margin="3"
        :size="220"
        foreground="#18181b"
        background="#f1f5f9"
        render-as="svg"
      />
    </div>
    <template #footer>
      <InputGroup>
        <InputGroupAddon>
          <Button @click="copyShareUrl" label="Copy" icon="bi bi-clipboard" />
        </InputGroupAddon>
        <InputText :value="shareUrl" disabled />
      </InputGroup>
    </template>
  </Dialog>
</template>
