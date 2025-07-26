<script setup lang="ts">
import { computed, useId } from "vue";
import { useRouter } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useFilterQuery, { toFilterQueryParams } from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import * as commonmark from "commonmark";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import IconButton from "@/components/system/IconButton.vue";
import Divider from "primevue/divider";

const router = useRouter();
const filterCriteria = useFilterQuery();

const props = defineProps<{
  event: Event;
  day: number;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);

const mdReader = new commonmark.Parser({ smart: true });
const mdWriter = new commonmark.HtmlRenderer({ safe: true });

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  const parsed = mdReader.parse(event.value.description);
  return mdWriter.render(parsed);
});

const back = async () => {
  await router.push({
    name: "schedule",
    params: { dayIndex: props.day },
    query: toFilterQueryParams(filterCriteria),
  });
};

const useSectionHeadingId = useId();
</script>

<template>
  <section :aria-labelledby="useSectionHeadingId">
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <IconButton class="lg:!hidden" icon="chevron-left" label="Back" @click="back()" />
      <IconButton class="!hidden lg:!block" icon="x-lg" label="Close" @click="back()" />
      <h2 :id="useSectionHeadingId" class="text-xl font-bold">{{ event.name }}</h2>
    </div>
    <div class="px-6">
      <dl class="flex flex-col items-start gap-2">
        <EventDetail v-if="event.startTime" icon="clock" icon-label="Time">
          {{ localizeTimeSpan(event.startTime, event.endTime) }}
        </EventDetail>
        <EventDetail v-if="event.people.length > 0" icon="person-circle" icon-label="Hosts">
          Hosted by {{ event.people.join(", ") }}
        </EventDetail>
        <EventDetail v-if="event.location" icon="geo-alt-fill" icon-label="Location">
          {{ event.location }}
        </EventDetail>
      </dl>
      <div v-if="event.category || event.tags.length > 0" class="mt-4 flex flex-wrap gap-3">
        <CategoryLabel
          v-if="event.category"
          :title="event.category"
          :all-categories="props.allCategories"
          display="active"
        />
        <CategoryLabel v-for="tag in event.tags" :key="tag" :title="tag" display="active" />
      </div>
      <Divider />
      <article
        id="event-description"
        v-if="descriptionHtml"
        v-html="descriptionHtml"
        class="my-4"
      ></article>
      <div v-else class="text-center text-lg italic text-slate-500 dark:text-zinc-400 mt-8">
        <span>No description</span>
      </div>
    </div>
  </section>
</template>
