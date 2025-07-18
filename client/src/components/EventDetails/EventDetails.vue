<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import EventDetail from "./EventDetail.vue";
import IconButton from "@/components/system/IconButton.vue";

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
  <div class="flex flex-col gap-4">
    <div class="flex justify-start items-center gap-4 p-4">
      <IconButton icon="chevron-left" label="Back" @click="router.back()" />
      <h1 class="text-2xl">{{ event.title }}</h1>
    </div>
    <div class="flex flex-col gap-2 px-8">
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
  </div>
</template>
