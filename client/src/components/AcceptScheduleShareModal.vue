<script setup lang="ts">
import Dialog from "primevue/dialog";
import Button from "primevue/button";

const visible = defineModel<boolean>("visible", {
  type: Boolean,
  required: true,
});

const props = defineProps<{
  eventsCount: number;
}>();
</script>

<template>
  <Dialog class="max-w-100 m-4" v-model:visible="visible" modal header="View Schedule">
    <p>
      Someone shared a schedule with <strong>{{ props.eventsCount }} events</strong>. Do you want to
      view it, or do you want to add the events to your schedule?
    </p>
    <template #footer>
      <div class="flex flex-col gap-2 w-full justify-stretch">
        <Button label="View Schedule" icon="bi bi-search" />
        <Button label="Add to My Schedule" icon="bi bi-heart" />
      </div>
    </template>
  </Dialog>
</template>
