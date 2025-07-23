<script setup lang="ts">
import { ref, computed, watchEffect, useId } from "vue";
import flexsearch from "flexsearch";
import useEvents from "@/composables/useEvents";
import useFilterQuery from "@/composables/useFilterQuery";
import { type Event } from "@/utils/api";
import { isNotNullish } from "@/utils/types";
import { getSortedCategories } from "@/utils/tags";
import InputText from "primevue/inputtext";
import FilterDescription from "./FilterDescription.vue";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import IconButton from "@/components/system/IconButton.vue";
import FilterMenu from "./FilterMenu.vue";

const { reload: reloadEvents } = useEvents();

const filterCriteria = useFilterQuery();

const isFiltered = computed(
  () => filterCriteria.categories.length > 0 || filterCriteria.tags.length > 0,
);

const props = defineProps<{
  events: Array<Event>;
}>();

const eventIds = defineModel<Array<string>>("ids");

const allCategories = computed(() => getSortedCategories(props.events));

const allTags = computed(() =>
  props.events.reduce<Array<string>>((set, event) => {
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

  for (const event of props.events) {
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

const searchText = ref();
const showFilterMenu = ref(false);

watchEffect(() => {
  for (const event of props.events) {
    searchIndex.add({
      id: event.id,
      name: event.name,
      description: event.description,
      location: event.location,
      people: event.people.join(", "),
      category: event.category,
      tags: event.tags.join(", "),
    });
  }
});

watchEffect(() => {
  let filteredEvents = [...props.events];

  if ((searchText.value?.length ?? 0) > 0) {
    const results = searchIndex.search(searchText.value);

    filteredEvents = results
      .flatMap((result) => result.result)
      .map((id) => eventsById.value.get(id.toString()))
      .filter(isNotNullish);
  }

  if (filterCriteria.categories.length > 0) {
    filteredEvents = filteredEvents.filter((event) =>
      filterCriteria.categories.includes(event.category),
    );
  }

  if (filterCriteria.tags.length > 0) {
    filteredEvents = filteredEvents.filter((event) =>
      event.tags.some((tag) => filterCriteria.tags.includes(tag)),
    );
  }

  eventIds.value = filteredEvents.map((event) => event.id);
});

const filterMenuId = useId();
</script>

<template>
  <search class="flex flex-col gap-4">
    <div class="flex gap-4">
      <div class="grow">
        <IconField>
          <InputIcon class="bi bi-search" />
          <InputText
            v-model="searchText"
            class="w-full"
            placeholder="Search…"
            aria-label="Search"
          />
        </IconField>
      </div>
      <IconButton
        icon="filter"
        label="Filter"
        :active="showFilterMenu"
        :badge="isFiltered"
        @click="showFilterMenu = !showFilterMenu"
        :button-props="{
          'aria-controls': filterMenuId,
          'aria-expanded': showFilterMenu,
        }"
      />
      <IconButton
        class="!hidden lg:!block"
        icon="arrow-clockwise"
        label="Refresh"
        @click="reloadEvents"
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
      v-if="isFiltered && !showFilterMenu"
      :criteria="filterCriteria"
      :all-categories="allCategories"
    />
  </search>
</template>
