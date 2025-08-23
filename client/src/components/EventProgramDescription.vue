<script setup lang="ts">
import Panel from "primevue/panel";
import * as commonmark from "commonmark";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import { type Event } from "@/utils/api";
import { computed, useId } from "vue";
import { localizeTimeSpan } from "@/utils/time";

const props = defineProps<{
  expand: boolean;
  event: Event;
}>();

const datetimeFormats = useDatetimeFormats();

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const descriptionHtml = computed(() => {
  if (!props.event.description) return undefined;
  const parsed = mdReader.parse(props.event.description);
  return mdWriter.render(parsed);
});

const eventNameHeadingId = useId();
</script>

<template>
  <Panel
    :toggleable="event.description !== undefined"
    :collapsed="!props.expand"
    pt:header:class="!py-0"
    pt:content:class="!pb-0"
    pt:root:class="!py-4"
  >
    <template #header>
      <div class="flex flex-col">
        <h3 class="text-lg font-bold" :id="eventNameHeadingId">{{ props.event.name }}</h3>
        <span class="text-muted-color" v-if="datetimeFormats !== undefined">{{
          localizeTimeSpan(datetimeFormats, props.event.startTime, props.event.endTime)
        }}</span>
      </div>
    </template>
    <article
      id="document"
      v-if="descriptionHtml"
      v-html="descriptionHtml"
      :aria-labelledby="eventNameHeadingId"
    ></article>
  </Panel>
</template>
