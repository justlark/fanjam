<script lang="ts" setup>
import IconButton from "./IconButton.vue";
import BioDialog from "./BioDialog.vue";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

defineEmits(["find"]);

const props = defineProps<{
  name: string;
  icon: string;
  bio?: string;
}>();
</script>

<template>
  <BioDialog
    v-model:visible="visible"
    :name="props.name"
    :icon="props.icon"
    :bio="props.bio"
    @find="$emit('find', $event)"
  >
    <IconButton
      class="mx-auto"
      icon="search"
      label="Find in Schedule"
      size="sm"
      :show-label="true"
      :button-props="{ 'data-testid': 'bio-find-button' }"
      @click="(event) => $emit('find', event)"
    />
  </BioDialog>
</template>
