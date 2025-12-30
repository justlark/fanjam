import { computed, type Ref } from "vue";
import useStarredEvents from "./useStarredEvents";
import { useToast } from "primevue/usetoast";

const useIsEventStarred = (eventId: Ref<string | undefined>) => {
  const starredEvents = useStarredEvents();
  const toast = useToast();

  const addStarToastMessage = {
    severity: "info",
    summary: "Added",
    detail: "Event added to your schedule.",
    life: 1000,
  } as const;

  const removeStarToastMessage = {
    severity: "info",
    summary: "Removed",
    detail: "Event removed from your schedule.",
    life: 1000,
  } as const;

  return computed({
    get() {
      if (eventId.value === undefined) {
        return false;
      }

      return starredEvents.value.has(eventId.value);
    },
    set(value: boolean) {
      if (eventId.value === undefined) {
        return;
      }

      toast.remove(addStarToastMessage);
      toast.remove(removeStarToastMessage);

      // Remember: Vue can't track state changes in sets/maps, so we need to
      // create a new set.
      if (value) {
        starredEvents.value = new Set([...starredEvents.value, eventId.value]);
        toast.add(addStarToastMessage);
      } else {
        const newSet = new Set(starredEvents.value);
        newSet.delete(eventId.value);
        starredEvents.value = newSet;
        toast.add(removeStarToastMessage);
      }
    },
  });
};

export default useIsEventStarred;
