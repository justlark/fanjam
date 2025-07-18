<script setup lang="ts">
import { computed } from "vue";
import Tag from "primevue/tag";
import { bgColorForString, fgColorForString } from "@/utils/colors";

const props = defineProps<{
  title: string;
  category: string;
}>();

const bgColor = computed(() => bgColorForString(props.category, 100));
const fgColor = computed(() => fgColorForString(props.category, 500));
</script>

<template>
  <Tag :value="props.title" :class="`!${fgColor} !${bgColor}`" />
</template>
