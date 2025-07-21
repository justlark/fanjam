<script setup lang="ts">
import { ref, watchEffect } from "vue";
import flexsearch from "flexsearch";
import useEvents from "@/composables/useEvents";
import { type Event } from "@/utils/api";
import InputText from "primevue/inputtext";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import IconButton from "@/components/system/IconButton.vue";

const { reload: reloadEvents } = useEvents();

const props = defineProps<{
  events: Array<Event>;
}>();

const eventIds = defineModel("ids", {
  type: Array<string>,
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

const executeSearch = () => {
  if ((searchText.value?.length ?? 0) > 0) {
    const results = searchIndex.search(searchText.value);
    eventIds.value = results.flatMap((result) => result.result).map((id) => id.toString());
  } else {
    eventIds.value = undefined;
  }
};
</script>

<template>
  <search class="flex gap-4">
    <div class="grow">
      <IconField>
        <InputIcon class="bi bi-search" />
        <InputText
          v-model="searchText"
          class="w-full"
          placeholder="Search…"
          aria-label="Search"
          @input="executeSearch()"
        />
      </IconField>
    </div>
    <IconButton icon="filter" label="Filter" />
    <IconButton
      class="!hidden lg:!flex"
      icon="arrow-clockwise"
      label="Refresh"
      @click="reloadEvents"
    />
  </search>
</template>
