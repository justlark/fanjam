<script setup lang="ts">
import { toRef, ref, computed, watchEffect, useId } from "vue";
import flexsearch from "flexsearch";
import useRemoteData from "@/composables/useRemoteData";
import useStarredEvents from "@/composables/useStarredEvents";
import useFilterQuery from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { isNotNullish } from "@/utils/types";
import { getSortedCategories } from "@/utils/tags";
import InputText from "primevue/inputtext";
import FilterDescription from "./FilterDescription.vue";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import IconButton from "./IconButton.vue";
import Button from "primevue/button";
import FilterMenu from "./FilterMenu.vue";

const {
  data: { events },
} = useRemoteData();

const filterCriteria = useFilterQuery();
const starredEvents = useStarredEvents();
const searchText = toRef(filterCriteria, "search");

const showFilterBadge = computed(
  () =>
    filterCriteria.hidePastEvents ||
    filterCriteria.hideNotStarred ||
    filterCriteria.categories.length > 0 ||
    filterCriteria.tags.length > 0,
);

const showFilterDescription = computed(
  () =>
    filterCriteria.hideNotStarred ||
    filterCriteria.categories.length > 0 ||
    filterCriteria.tags.length > 0 ||
    filterCriteria.search.length > 0,
);

const eventIds = defineModel<Array<string>>("ids");

const allCategories = computed(() => getSortedCategories(events.value));

const allTags = computed(() =>
  events.value.reduce<Array<string>>((set, event) => {
    for (const tag of event.tags) {
      if (!set.includes(tag)) {
        set.push(tag);
      }
    }
    return set;
  }, []),
);

const eventsById = computed<Map<string, Event>>(() => {
  const map = new Map<string, Event>();

  for (const event of events.value) {
    map.set(event.id, event);
  }

  return map;
});

const searchIndex = new flexsearch.Document({
  tokenize: "forward",
  cache: true,
  document: {
    id: "id",
    store: ["id"],
    index: ["name", "description", "location", "people", "category", "tags"],
  },
});

const showFilterMenu = ref(false);

watchEffect(() => {
  for (const event of events.value) {
    searchIndex.add({
      id: event.id,
      name: event.name,
      description: event.description ?? "",
      location: event.location ?? "",
      people: event.people.join(", "),
      category: event.category ?? "",
      tags: event.tags.join(", "),
    });
  }
});

const filterTags = computed(() => new Set(filterCriteria.tags));
const filterCategories = computed(() => new Set(filterCriteria.categories));

watchEffect(() => {
  let filteredEvents = [...events.value];

  if (searchText.value.length > 0) {
    const results = searchIndex.search(searchText.value);

    filteredEvents = results
      .flatMap((result) => result.result)
      .map((id) => eventsById.value.get(id.toString()))
      .filter(isNotNullish);
  }

  if (filterCriteria.categories.length > 0) {
    filteredEvents = filteredEvents.filter(
      (event) => event.category && filterCategories.value.has(event.category),
    );
  }

  if (filterCriteria.tags.length > 0) {
    filteredEvents = filteredEvents.filter((event) =>
      event.tags.some((tag) => filterTags.value.has(tag)),
    );
  }

  if (filterCriteria.hidePastEvents) {
    const now = new Date();
    filteredEvents = filteredEvents.filter((event) => event.startTime >= now);
  }

  if (filterCriteria.hideNotStarred) {
    filteredEvents = filteredEvents.filter((event) => starredEvents.value.has(event.id));
  }

  eventIds.value = filteredEvents.map((event) => event.id);
});

const filterMenuId = useId();
</script>

<template>
  <search class="flex flex-col gap-4">
    <div class="flex gap-4">
      <div class="grow">
        <InputGroup>
          <IconField>
            <InputIcon class="bi bi-search" />
            <InputText
              v-model="searchText"
              class="w-full"
              placeholder="Search…"
              aria-label="Search"
            />
          </IconField>
          <InputGroupAddon>
            <Button
              aria-label="Clear"
              icon="bit bi-x-lg"
              severity="secondary"
              @click="searchText = ''"
            />
          </InputGroupAddon>
        </InputGroup>
      </div>
      <IconButton
        icon="filter"
        label="Filter"
        :active="showFilterMenu"
        :badge="showFilterBadge"
        @click="showFilterMenu = !showFilterMenu"
        :button-props="{
          'aria-controls': filterMenuId,
          'aria-expanded': showFilterMenu,
        }"
      />
    </div>
    <FilterMenu
      :id="filterMenuId"
      class="mb-4"
      v-if="showFilterMenu"
      :categories="allCategories"
      :tags="allTags"
    />
    <FilterDescription
      v-if="showFilterDescription && !showFilterMenu"
      v-model:criteria="filterCriteria"
      :all-categories="allCategories"
    />
  </search>
</template>
