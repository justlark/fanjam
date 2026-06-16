<script setup lang="ts">
import { useId, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { getSortedCategories } from "@/utils/tags";
import useRemoteData from "@/composables/useRemoteData";
import SimpleIcon from "./SimpleIcon.vue";
import IconButton from "./IconButton.vue";
import ScheduleList from "./ScheduleList.vue";
import ProgressSpinner from "primevue/progressspinner";

const route = useRoute();
const router = useRouter();
const {
  data: { events, /*locations,*/ people },
} = useRemoteData();

// TODO: Hook up to live data.
const locations = computed(() => [
  {
    id: "1",
    name: "Thoreau Room",
    description: "Upstairs on the left.",
    events: [10, 8, 6, 33, 15, 36, 4, 40, 5, 20],
  },
]);

const thisBio = computed(() => {
  if (route.name === "location") {
    return locations.value.find((location) => location.id === route.params.locationId);
  } else if (route.name === "person") {
    return people.value.find((person) => person.id === route.params.personId);
  } else {
    return undefined;
  }
});

const thisBioEvents = computed(() => events.value.filter((event) => event.id === thisBio.value.id));
const allCategories = computed(() => getSortedCategories(thisBioEvents.value));

const descriptionHtml = computed(() => thisBio.value?.description);

const back = async () => {
  await router.back();
};

const bioHeadingId = useId();
</script>

<template>
  <div class="h-full">
    <section
      class="max-w-200 mx-auto"
      v-if="thisBio"
      :aria-labelledby="bioHeadingId"
      data-testid="bio-page"
    >
      <div class="flex justify-start items-center gap-2 pl-2 pr-4 py-4">
        <IconButton
          icon="chevron-left"
          label="Back"
          @click="back()"
          :button-props="{ 'data-testid': 'bio-page-back-button' }"
        />
        <SimpleIcon
          class="text-xl"
          :icon="route.name === 'location' ? 'geo-alt-fill' : 'person-circle'"
          :label="route.name === 'location' ? 'Location' : 'Person'"
        />
        <h2 :id="bioHeadingId" class="text-xl font-bold" data-testid="bio-page-title">
          {{ thisBio.name }}
        </h2>
      </div>
      <div class="px-6">
        <div
          id="document"
          v-if="descriptionHtml && thisBio?.description?.trim() !== ''"
          v-html="descriptionHtml"
          data-testid="bio-page-body"
        ></div>
        <div
          v-else
          class="text-center text-lg italic text-muted-color mt-8"
          data-testid="bio-page-no-details"
        >
          No details provided
        </div>
      </div>
      <ScheduleList
        class="p-6"
        :events="thisBioEvents"
        :all-categories="allCategories"
        view-type="all"
      />
    </section>
    <div v-else class="flex items-center h-full">
      <ProgressSpinner />
    </div>
  </div>
</template>
