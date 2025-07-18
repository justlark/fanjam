<script setup lang="ts">
import InputText from "primevue/inputtext";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import Button from "primevue/button";
</script>

<template>
  <div class="flex gap-4">
    <div class="grow">
      <IconField>
        <InputIcon class="bi bi-search" />
        <InputText class="w-full" placeholder="Search…" />
      </IconField>
    </div>
    <Button
      icon="bi bi-filter"
      pt:icon="!text-4xl"
      variant="text"
      size="large"
      aria-label="Menu"
      rounded
    />
  </div>
</template>
