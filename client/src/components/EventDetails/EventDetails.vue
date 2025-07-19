<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import IconButton from "@/components/system/IconButton.vue";
import Divider from "primevue/divider";
import Tag from "primevue/tag";

const router = useRouter();

const props = defineProps<{
  event: Event;
  day: number;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);

const back = () => {
  router.push({
    name: "schedule",
    params: { dayIndex: props.day },
  });
};
</script>

<template>
  <div>
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <IconButton class="lg:!hidden" icon="chevron-left" label="Back" @click="back()" />
      <IconButton class="!hidden lg:!flex" icon="x-lg" label="Close" @click="back()" />
      <h2 class="text-xl font-bold">{{ event.name }}</h2>
    </div>
    <div class="px-6">
      <div class="flex flex-col items-start gap-2">
        <EventDetail v-if="event.startTime" icon="clock" icon-label="Time">
          {{ localizeTimeSpan(event.startTime, event.endTime) }}
        </EventDetail>
        <EventDetail v-if="event.people.length > 0" icon="person-circle">
          Hosted by {{ event.people.join(", ") }}
        </EventDetail>
        <EventDetail v-if="event.location" icon="geo-alt-fill" icon-label="Location">
          {{ event.location }}
        </EventDetail>
      </div>
      <CategoryLabel
        v-if="event.category"
        class="mt-4"
        :title="event.category"
        :all-categories="props.allCategories"
      />
      <Divider />
      <div v-if="event.description" class="my-4">
        {{ event.description }}
      </div>
      <div v-if="event.tags.length > 0" class="flex flex-wrap gap-3">
        <Tag v-for="tag in event.tags" :key="tag" :value="tag" severity="secondary" />
      </div>
    </div>
  </div>
</template>
