<script setup lang="ts">
import { ref, computed, watch, watchEffect, type DeepReadonly, useId } from "vue";
import flexsearch from "flexsearch";
import { debounce } from "@/utils/debounce";
import useRemoteData from "@/composables/useRemoteData";
import useStarredEvents from "@/composables/useStarredEvents";
import useFilterQuery from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { isNotNullish } from "@/utils/types";
import { getSortedCategories } from "@/utils/tags";
import InputText from "primevue/inputtext";
import FilterDescription from "./FilterDescription.vue";
import IconField from "primevue/iconfield";
import SelectButton from "primevue/selectbutton";
import InputIcon from "primevue/inputicon";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import IconButton from "./IconButton.vue";
import Button from "primevue/button";
import FilterMenu from "./FilterMenu.vue";

const {
  data: { events },
  status: { events: eventsStatus },
} = useRemoteData();

const filterCriteria = useFilterQuery();
const starredEvents = useStarredEvents();

// How long to wait before acting on what's been typed in the search box.
// Rebuilding the whole schedule on each keystroke would become a performance
// problem for a large schedule.
const SEARCH_DEBOUNCE_MS = 150;

const searchText = ref(filterCriteria.search);

const commitSearch = debounce((value: string) => {
  filterCriteria.search = value;
}, SEARCH_DEBOUNCE_MS);

watch(searchText, (value) => {
  commitSearch(value);
});

// The search input can be set from elsewhere, like directly from the query
// params. This happens on back navigation and when clicking the "Find in
// Schedule" button on the person and location bio pages.
watch(
  () => filterCriteria.search,
  (value) => {
    if (value !== searchText.value) {
      commitSearch.cancel();
      searchText.value = value;
    }
  },
);

// Pressing the clear button is not typing, so there is nothing to wait for.
const clearSearch = () => {
  commitSearch.cancel();
  searchText.value = "";
  filterCriteria.search = "";
};

const showFilterDescription = computed(
  () =>
    filterCriteria.categories.length > 0 ||
    filterCriteria.tags.length > 0 ||
    filterCriteria.search.length > 0,
);

const eventIds = defineModel<Array<string>>("ids");
const viewType = defineModel<"daily" | "all">("view", { required: true });

const viewTypeButtonOptions = ref([
  { label: "By Day", value: "daily" },
  { label: "All Events", value: "all" },
]);
const allCategories = computed(() => getSortedCategories(events.value));

const allTags = computed(() =>
  events.value.reduce<Array<string>>((set, event) => {
    for (const tag of event.tags) {
      if (!set.includes(tag.name)) {
        set.push(tag.name);
      }
    }
    return set;
  }, []),
);

const eventsById = computed<Map<string, DeepReadonly<Event>>>(() => {
  const map = new Map<string, DeepReadonly<Event>>();

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
const searchFocused = ref(false);

watchEffect(() => {
  // Wait until all events have loaded before building the search index,
  // otherwise we're going to rebuild the search index every time new events
  // arrive, which would be a performance nightmare.
  if (eventsStatus.value !== "success") {
    return;
  }

  searchIndex.clear();

  for (const event of events.value) {
    searchIndex.add({
      id: event.id,
      name: event.name,
      description: event.description ?? "",
      location: event.location?.name ?? "",
      people: event.people.map((person) => person.name).join(", "),
      category: event.category?.name ?? "",
      tags: event.tags.map((tag) => tag.name).join(", "),
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
      (event) => event.category && filterCategories.value.has(event.category.name),
    );
  }

  if (filterCriteria.tags.length > 0) {
    filteredEvents = filteredEvents.filter((event) =>
      event.tags.some((tag) => filterTags.value.has(tag.name)),
    );
  }

  if (filterCriteria.hidePastEvents) {
    const now = new Date();
    filteredEvents = filteredEvents.filter(
      (event) => event.endTime === undefined || event.endTime >= now,
    );
  }

  if (filterCriteria.hideNotStarred) {
    filteredEvents = filteredEvents.filter((event) => starredEvents.value.has(event.id));
  }

  const ids = filteredEvents.map((event) => event.id);

  // Avoid rebuilding the whole schedule if the filtered event IDs haven't
  // changed.
  if (
    eventIds.value !== undefined &&
    eventIds.value.length === ids.length &&
    eventIds.value.every((id, index) => id === ids[index])
  ) {
    return;
  }

  eventIds.value = ids;
});

const filterMenuId = useId();

// Shared by both breakpoint variants of the filter button below.
const filterButtonProps = computed(() => ({
  "aria-controls": filterMenuId,
  "aria-expanded": showFilterMenu.value,
  "data-testid": "filter-menu-button",
}));
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex gap-4 min-h-15 items-center">
      <div class="grow">
        <InputGroup>
          <IconField>
            <InputIcon class="bi bi-search" />
            <InputText
              v-model="searchText"
              class="w-full"
              placeholder="Search…"
              aria-label="Search"
              pt:root:data-testid="filter-search-input"
              @focus="searchFocused = true"
              @blur="searchFocused = false"
            />
          </IconField>
          <InputGroupAddon>
            <Button
              aria-label="Clear"
              icon="bit bi-x-lg"
              severity="secondary"
              @click="clearSearch()"
            />
          </InputGroupAddon>
        </InputGroup>
      </div>
      <IconButton
        class="max-sm:hidden"
        icon="filter"
        label="Filter"
        size="md"
        icon-size-override="lg"
        :show-label="true"
        :active="showFilterMenu"
        @click="showFilterMenu = !showFilterMenu"
        :button-props="filterButtonProps"
      />
      <IconButton
        v-if="!searchFocused"
        class="sm:hidden"
        icon="filter"
        label="Filter"
        size="md"
        icon-size-override="lg"
        :show-label="true"
        :active="showFilterMenu"
        @click="showFilterMenu = !showFilterMenu"
        :button-props="filterButtonProps"
      />
    </div>
    <FilterMenu
      :id="filterMenuId"
      class="mb-4"
      v-if="showFilterMenu"
      :categories="allCategories"
      :tags="allTags"
    />
    <SelectButton
      v-model="viewType"
      :options="viewTypeButtonOptions"
      option-label="label"
      option-value="value"
      size="small"
      fluid
      :allow-empty="false"
      data-testid="schedule-view-selector"
      class="max-w-lg mx-auto"
    />
    <FilterDescription
      v-if="showFilterDescription && !showFilterMenu"
      v-model:criteria="filterCriteria"
      :all-categories="allCategories"
    />
  </div>
</template>
