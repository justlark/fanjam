<script setup lang="ts">
import Dialog from "primevue/dialog";
import { renderMarkdown } from "@/utils/markdown";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const toast = useToast();

const props = defineProps<{
  name: string;
  description?: string;
}>();

const descriptionHtml = computed(() => {
  if (props.description !== undefined && props.description.trim() !== "") return undefined;
  return renderMarkdown(props.description);
});
</script>

<template>
  <Dialog
    class="w-100 m-4"
    v-model:visible="visible"
    modal
    dismissable-mask
    block-scroll
    :draggable="false"
    :header="props.name"
  >
    <div
      id="document"
      v-if="descriptionHtml"
      v-html="descriptionHtml"
      data-testid="bio-description"
    ></div>
  </Dialog>
</template>
