<script setup lang="ts">
import { ref, computed } from "vue";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import SelectButton from "primevue/selectbutton";
import Button from "primevue/button";
import QrcodeVue from "qrcode.vue";
import Dialog from "primevue/dialog";
import { useToast } from "primevue/usetoast";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const toast = useToast();

export interface LinkProps {
  link: string;
  title?: string;
  message: string;
  toastMessage: string;
}

const props = defineProps<{
  title: string;
  links: Array<LinkProps>;
}>();

const copyShareUrl = async () => {
  const linkProps = props.links[selectedShareIndex.value];
  await navigator.clipboard.writeText(linkProps.link);

  toast.add({
    severity: "info",
    summary: "Link copied",
    detail: linkProps.toastMessage,
    life: 1500,
  });
};

const selectedShareIndex = ref(0);

const shareSelectButtonOptions = computed(() =>
  props.links.map((link, index) => ({
    label: link.title,
    value: index,
  })),
);
</script>

<template>
  <Dialog
    class="w-100 m-4"
    v-model:visible="visible"
    modal
    dismissable-mask
    block-scroll
    :draggable="false"
    :header="props.title"
  >
    <SelectButton
      v-if="props.links.length > 1"
      v-model="selectedShareIndex"
      :options="shareSelectButtonOptions"
      option-label="label"
      option-value="value"
      size="small"
      fluid
      :allow-empty="false"
      data-testid="link-share-selector"
      class="max-w-lg mx-auto mb-6"
    />
    <div
      v-for="(link, index) in props.links"
      v-show="index === selectedShareIndex"
      :key="link.link"
    >
      <div class="my-auto py-6">
        <div class="rounded-3xl overflow-hidden mx-auto w-fit shadow-md">
          <QrcodeVue
            :value="link.link"
            :margin="3"
            :size="220"
            foreground="#18181b"
            background="#f1f5f9"
            render-as="svg"
          />
        </div>
      </div>
      <div class="h-20 flex flex-col justify-end">
        <p data-testid="link-share-dialog-description">
          {{ link.message }}
        </p>
      </div>
      <InputGroup class="mt-3">
        <InputGroupAddon>
          <Button
            data-testid="link-share-dialog-copy-button"
            @click="copyShareUrl"
            label="Copy"
            icon="bi bi-clipboard"
          />
        </InputGroupAddon>
        <InputText data-testid="link-share-dialog-url" :value="link.link" disabled />
      </InputGroup>
    </div>
  </Dialog>
</template>
