<script setup lang="ts">
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import Button from "primevue/button";
import QrcodeVue from "qrcode.vue";
import Dialog from "primevue/dialog";
import { useToast } from "primevue/usetoast";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const toast = useToast();

const props = defineProps<{
  link: string;
  title: string;
  toastMessage: string;
}>();

const copyShareUrl = async () => {
  await navigator.clipboard.writeText(props.link);

  toast.add({
    severity: "info",
    summary: "Link Copied",
    detail: props.toastMessage,
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
    block-scroll
    :draggable="false"
    :header="props.title"
  >
    <slot name="header" />
    <div class="rounded-3xl overflow-hidden mx-auto w-fit my-6 shadow-md">
      <QrcodeVue
        :value="props.link"
        :margin="3"
        :size="220"
        foreground="#18181b"
        background="#f1f5f9"
        render-as="svg"
      />
    </div>
    <InputGroup>
      <InputGroupAddon>
        <Button @click="copyShareUrl" label="Copy" icon="bi bi-clipboard" />
      </InputGroupAddon>
      <InputText :value="props.link" disabled />
    </InputGroup>
    <slot name="footer" />
  </Dialog>
</template>
