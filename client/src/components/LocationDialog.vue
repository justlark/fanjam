<script lang="ts" setup>
import { computed } from "vue";
import IconButton from "./IconButton.vue";
import BioDialog from "./BioDialog.vue";
import useRemoteData from "@/composables/useRemoteData";

const {
  data: { config },
} = useRemoteData();

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

defineEmits(["find"]);

const props = defineProps<{
  name: string;
  icon: string;
  bio?: string;
  mapLink?: string;
}>();

const useMapLink = computed(() => config.value?.useMapLink ?? true);
</script>

<template>
  <BioDialog
    v-model:visible="visible"
    :name="props.name"
    :icon="props.icon"
    :bio="props.bio"
    @find="$emit('find', $event)"
  >
    <div class="flex flex-row flex-wrap mx-auto">
      <IconButton
        class="mx-auto"
        icon="search"
        label="Find in Schedule"
        size="sm"
        :show-label="true"
        :button-props="{ 'data-testid': 'bio-find-button' }"
        @click="(event) => $emit('find', event)"
      />
      <IconButton
        v-if="useMapLink && props.mapLink"
        class="mx-auto"
        icon="geo-fill"
        label="Find on Map"
        size="sm"
        :show-label="true"
        :button-props="{
          'data-testid': 'bio-open-map',
          as: 'a',
          href: props.mapLink,
        }"
      />
    </div>
  </BioDialog>
</template>
