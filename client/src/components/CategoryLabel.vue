<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { bgColorForString, fgColorForString } from "@/utils/colors";

const props = defineProps<{
  category: string;
}>();

const bgColor = computed(() => bgColorForString(props.category, 100));
const fgColor = computed(() => fgColorForString(props.category, 500));
</script>

<template>
  <Tag :value="props.category" :class="`!${fgColor} !${bgColor}`" />
</template>
