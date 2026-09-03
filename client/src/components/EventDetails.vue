<script setup lang="ts">
import { computed, ref, useId, type DeepReadonly, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { localizeTimeSpan } from "@/utils/time";
import useDatetimeFormats from "@/composables/useDatetimeFormats";
import useIsEventStarred from "@/composables/useIsEventStarred";
import useIsSharedSchedule from "@/composables/useIsSharedSchedule";
import { toFilterQueryParams } from "@/composables/useFilterQuery";
import { renderMarkdown } from "@/utils/markdown";
import { useToast } from "primevue/usetoast";
import useRemoteData from "@/composables/useRemoteData";
import { type Event } from "@/utils/api";
import EventDetail from "./EventDetail.vue";
import BioDialog from "./BioDialog.vue";
import IconButton from "./IconButton.vue";
import Divider from "primevue/divider";
import TagBar from "./TagBar.vue";
import LinkShareDialog from "./LinkShareDialog.vue";
import { TOAST_TTL_LONG } from "@/utils/toast";

const datetimeFormats = useDatetimeFormats();
const route = useRoute();
const router = useRouter();
const toast = useToast();

const props = defineProps<{
  event: DeepReadonly<Event>;
  day?: number;
  allCategories: Array<string>;
}>();

const event = computed(() => props.event);
const isStarred = useIsEventStarred(computed(() => event.value.id));
const isSharedSchedule = useIsSharedSchedule();

const {
  data: { people, locations },
} = useRemoteData();

const descriptionHtml = computed(() => {
  if (!event.value.description) return undefined;
  return renderMarkdown(event.value.description);
});

const sectionHeadingId = useId();

const fromViewType = ref<"daily" | "all">();
const shareDialogVisible = ref(false);

const personBioDialogVisible = ref(false);
const currentPersonId = ref<string>();
const currentPersonName = ref<string>();

const currentPersonBio = computed(
  () => people.value.find((person) => person.id === currentPersonId.value)?.bio,
);

const locationBioDialogVisible = ref(false);
const currentLocationId = ref<string>();
const currentLocationName = ref<string>();

const currentLocationBio = computed(
  () => locations.value.find((location) => location.id === currentLocationId.value)?.description,
);

// Do not include the query params or fragment; users likely aren't intending
// to share their current search/filter params.
const eventUrl = computed(() => window.location.origin + window.location.pathname);

// The day number in the path is 1-based. It is undefined until the event's day
// is known, which on a direct link takes until the schedule has loaded; leaving
// it out lets the schedule pick its own landing day in the meantime.
const dayIndexParam = computed(() => (props.day === undefined ? undefined : props.day + 1));

// Where the back button goes. There is no view type when the event was opened
// from outside the app — a shared link in a fresh tab — so fall back to the day
// the event is on.
const scheduleRoute = computed(() => ({
  name: "schedule",
  params: {
    dayIndex: fromViewType.value === "all" ? "all" : dayIndexParam.value,
  },
}));

const back = async () => {
  await router.push({ ...scheduleRoute.value, query: route.query });
};

const toggleStar = () => {
  if (isSharedSchedule.value) {
    toast.add({
      severity: "warn",
      summary: "You're viewing someone else's schedule",
      detail:
        "To go back to your own schedule and make changes, open the options menu at the bottom of the screen.",
      life: TOAST_TTL_LONG,
    });

    return;
  }

  isStarred.value = !isStarred.value;
};

onMounted(() => {
  fromViewType.value = history.state.fromViewType;
});
</script>

<template>
  <section :aria-labelledby="sectionHeadingId">
    <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
      <span class="flex items-center">
        <IconButton
          class="lg:!hidden"
          icon="chevron-left"
          label="Back"
          :button-props="{ 'data-testid': 'event-details-back-button' }"
          @click="back()"
        />
        <IconButton
          class="!hidden lg:!block"
          icon="x-lg"
          label="Close"
          :button-props="{ 'data-testid': 'event-details-back-button' }"
          @click="back()"
        />
        <IconButton
          class="hidden lg:block"
          label="Star"
          :icon="isStarred ? 'star-fill' : 'star'"
          :button-props="{
            'aria-pressed': isStarred,
            'data-testid': 'event-details-star-button',
          }"
          @click="toggleStar()"
        />
        <IconButton
          icon="share-fill"
          class="hidden lg:block"
          label="Copy Link"
          size="md"
          @click="shareDialogVisible = true"
          :button-props="{ 'data-testid': 'event-share-link' }"
        />
      </span>
      <h2 :id="sectionHeadingId" class="text-xl font-bold" data-testid="event-details-name">
        {{ event.name }}
      </h2>
    </div>
    <div class="px-6">
      <div class="flex justify-between items-end">
        <dl class="flex flex-col items-start gap-2">
          <EventDetail
            v-if="datetimeFormats && event.startTime"
            icon="clock"
            icon-label="Time"
            data-testid="event-details-time"
          >
            {{ localizeTimeSpan(datetimeFormats, event.startTime, event.endTime) }}
          </EventDetail>
          <EventDetail
            v-if="event.people.length > 0"
            icon="person-circle"
            icon-label="Hosts"
            data-testid="event-details-hosts"
          >
            <span>Hosted by </span>
            <span v-for="(person, index) in event.people" :key="person.id">
              <button
                class="text-link-sm cursor-pointer"
                data-testid="event-details-person-link"
                @click="
                  personBioDialogVisible = true;
                  currentPersonId = person.id;
                  currentPersonName = person.name;
                "
              >
                {{ person.name }}
              </button>
              <span v-if="index < event.people.length - 1">, </span>
            </span>
          </EventDetail>
          <EventDetail
            v-if="event.location"
            icon="geo-alt-fill"
            icon-label="Location"
            data-testid="event-details-location"
          >
            <button
              class="text-link-sm cursor-pointer"
              data-testid="event-details-location-link"
              @click="
                locationBioDialogVisible = true;
                currentLocationId = event.location.id;
                currentLocationName = event.location.name;
              "
            >
              {{ event.location.name }}
            </button>
          </EventDetail>
        </dl>
        <div class="lg:hidden flex flex-col gap-1">
          <IconButton
            icon="share-fill"
            label="Copy Link"
            size="md"
            @click="shareDialogVisible = true"
            :button-props="{ 'data-testid': 'event-share-link' }"
          />
          <IconButton
            label="Star"
            :icon="isStarred ? 'star-fill' : 'star'"
            :button-props="{
              'aria-pressed': isStarred,
              'data-testid': 'event-details-star-button',
            }"
            @click="toggleStar()"
          />
        </div>
      </div>
      <TagBar
        class="mt-4"
        :category="event.category?.name"
        :tags="event.tags.map((tag) => tag.name)"
        :all-categories="props.allCategories"
        :to="scheduleRoute"
      />
      <Divider />
      <article
        id="document"
        :aria-labelledby="sectionHeadingId"
        class="my-4"
        data-testid="event-details-content"
      >
        <p v-if="event.summary" data-testid="event-details-summary">{{ event.summary }}</p>
        <div
          v-if="descriptionHtml"
          v-html="descriptionHtml"
          data-testid="event-details-description"
        />
      </article>
      <div
        v-if="!event.summary && !descriptionHtml"
        class="text-center text-lg italic text-surface-500 dark:text-surface-400 mt-8"
        data-testid="event-details-no-description"
      >
        <span>No description</span>
      </div>
    </div>
    <BioDialog
      v-if="currentPersonName"
      v-model:visible="personBioDialogVisible"
      :name="currentPersonName"
      icon="person-circle"
      :bio="currentPersonBio"
      @find="
        router.push({
          name: 'schedule',
          params: { dayIndex: dayIndexParam },
          query: toFilterQueryParams({ search: currentPersonName }),
        })
      "
    />
    <BioDialog
      v-if="currentLocationName"
      v-model:visible="locationBioDialogVisible"
      :name="currentLocationName"
      icon="geo-alt-fill"
      :bio="currentLocationBio"
      @find="
        router.push({
          name: 'schedule',
          params: { dayIndex: dayIndexParam },
          query: toFilterQueryParams({ search: currentLocationName }),
        })
      "
    />
    <LinkShareDialog
      v-model:visible="shareDialogVisible"
      title="Share Event"
      :link="eventUrl"
      message="Send someone a link to this event."
      toast-message="A link to this event has been copied to your clipboard."
    />
  </section>
</template>
