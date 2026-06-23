<script lang="ts" setup>
import { computed } from "vue";
import { renderMarkdown } from "@/utils/markdown";
import IconButton from "./IconButton.vue";
import Dialog from "primevue/dialog";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

defineEmits(["find"]);

const props = defineProps<{
  name: string;
  bio?: string;
}>();

const bioHtml = computed(() => (props.bio ? renderMarkdown(props.bio) : undefined));
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
    <article v-if="props.bio" data-testid="person-bio-body">
      <div v-if="bioHtml" v-html="bioHtml" />
    </article>
    <template #footer>
      <IconButton
        class="mx-auto"
        icon="search"
        label="Find in Schedule"
        size="sm"
        :show-label="true"
        :button-props="{ 'data-testid': 'person-find-button' }"
        @click="(event) => $emit('find', event)"
      />
    </template>
  </Dialog>
</template>
