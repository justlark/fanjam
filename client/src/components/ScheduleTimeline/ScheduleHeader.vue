<script setup lang="ts">
import InputText from "primevue/inputtext";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import IconButton from "@/components/system/IconButton.vue";
</script>

<template>
  <div class="flex gap-4">
    <div class="grow">
      <IconField>
        <InputIcon class="bi bi-search" />
        <InputText class="w-full" placeholder="Search…" />
      </IconField>
    </div>
    <IconButton icon="filter" label="Menu" />
  </div>
</template>
