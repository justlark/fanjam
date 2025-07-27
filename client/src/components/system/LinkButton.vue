<script setup lang="ts">
import { computed } from "vue";
import SimpleIcon from "@/components/system/SimpleIcon.vue";
import { RouterLink } from "vue-router";

export type LinkTarget =
  | {
      kind: "external";
      url: string;
    }
  | {
      kind: "file";
      url: string;
      mediaType?: string;
    }
  | {
      kind: "route";
      to: InstanceType<typeof RouterLink>["$props"]["to"];
    };

const props = defineProps<{
  name: string;
  target: LinkTarget;
  icon: string;
  iconLabel?: string;
}>();

const linkProps = computed(() => {
  if (props.target.kind === "external") {
    return {
      href: props.target.url,
      target: "_blank",
    };
  } else if (props.target.kind === "file") {
    return {
      href: props.target.url,
      type: props.target.mediaType,
      target: "_blank",
    };
  } else {
    return {
      to: props.target.to,
    };
  }
});
</script>

<template>
  <component
    :is="props.target.kind === 'route' ? RouterLink : 'a'"
    v-bind="linkProps"
    class="flex items-center justify-start gap-4 text-left text-lg border border-surface-300 hover:bg-surface-200 dark:border-surface-600 hover:dark:bg-surface-800 rounded-sm px-4 py-2 w-full hover:*:decoration-2"
  >
    <SimpleIcon :icon="props.icon" :label="props.iconLabel" />
    <span class="text-primary underline underline-offset-2">
      {{ props.name }}
    </span>
  </component>
</template>
