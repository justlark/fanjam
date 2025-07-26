<script setup lang="ts">
import SimpleIcon from "@/components/system/SimpleIcon.vue";

const props = defineProps<{
  url: string;
  name: string;
  icon: string;
  iconLabel?: string;
  mediaType?: string;
}>();
</script>

<template>
  <a
    :href="props.url"
    :type="props.mediaType"
    target="_blank"
    class="flex items-center justify-start gap-4 text-left text-lg border border-slate-300 hover:bg-slate-100 dark:border-zinc-600 hover:dark:bg-zinc-800 rounded-sm px-4 py-2 w-full hover:*:decoration-2"
  >
    <SimpleIcon :icon="props.icon" :label="props.iconLabel" />
    <span class="text-primary underline underline-offset-2">
      {{ props.name }}
    </span>
  </a>
</template>
