<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import EventDetail from "./EventDetail.vue";
import CategoryLabel from "@/components/system/CategoryLabel.vue";
import IconButton from "@/components/system/IconButton.vue";
import Divider from "primevue/divider";
import Tag from "primevue/tag";

export interface Event {
  title: string;
  description?: string;
  location?: string;
  startTime?: Date;
  endTime?: Date;
  people: Array<string>;
  category?: string;
  tags: Array<string>;
}

const router = useRouter();

const props = defineProps<{
  event: Event;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);

const timeFormat = new Intl.DateTimeFormat(undefined, { timeStyle: "short" });
const weekdayFormat = new Intl.DateTimeFormat(undefined, { weekday: "short" });

// TODO: What if the start and end days are more than a week apart? Unlikely,
// but in that case, we ought to show the full date.
const formatTime = (start: Date, end: Date | undefined) => {
  const startDay = start ? weekdayFormat.format(start) : undefined;
  const endDay = end ? weekdayFormat.format(end) : undefined;

  const startTime = start ? timeFormat.format(start) : undefined;
  const endTime = end ? timeFormat.format(end) : undefined;

  if (!end) {
    return `${startDay} ${startTime}`;
  } else if (startDay === endDay) {
    return `${startDay} ${startTime} - ${endTime}`;
  } else {
    return `${startDay} ${startTime} - ${endDay} ${endTime}`;
  }
};
</script>

<template>
  <div>
    <div class="flex justify-start items-center gap-2 px-2 py-4">
      <IconButton icon="chevron-left" label="Back" @click="router.back()" />
      <h2 class="text-xl font-bold">{{ event.title }}</h2>
    </div>
    <div class="px-6">
      <div class="flex flex-col items-start gap-2">
        <EventDetail v-if="event.startTime" icon="clock" icon-label="Time">
          {{ formatTime(event.startTime, event.endTime) }}
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
