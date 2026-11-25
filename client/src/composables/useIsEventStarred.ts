import { computed, type Ref } from "vue";
import useStarredEvents from "./useStarredEvents";
import useIsSharedSchedule from "./useIsSharedSchedule.ts";
import { useToast } from "primevue/usetoast";
import { TOAST_TTL_SHORT, TOAST_TTL_LONG } from "@/utils/toast";

const useIsEventStarred = (eventId: Ref<string | undefined>) => {
  const starredEvents = useStarredEvents();
  const isSharedSchedule = useIsSharedSchedule();
  const toast = useToast();

  const addStarToastMessage = {
    severity: "info",
    summary: "Added",
    detail: "Event added to your schedule.",
    life: TOAST_TTL_SHORT,
  } as const;

  const removeStarToastMessage = {
    severity: "info",
    summary: "Removed",
    detail: "Event removed from your schedule.",
    life: TOAST_TTL_SHORT,
  } as const;

  const sharedScheduleWarningToastMessage = {
    severity: "warn",
    summary: "You're viewing someone else's schedule",
    detail:
      "To go back to your own schedule and make changes, open the options menu at the bottom of the screen.",
    life: TOAST_TTL_LONG,
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

      if (isSharedSchedule.value) {
        toast.add(sharedScheduleWarningToastMessage);
        return;
      }

      toast.remove(addStarToastMessage);
      toast.remove(removeStarToastMessage);

      // Remember: Vue can't track state changes in sets/maps, so we need to
      // create a new set.
      if (value) {
        starredEvents.value = new Set([...starredEvents.value, eventId.value]);

        toast.add(addStarToastMessage);
        umami.track("star-event", { event_id: eventId.value });
      } else {
        const newSet = new Set(starredEvents.value);
        newSet.delete(eventId.value);
        starredEvents.value = newSet;

        toast.add(removeStarToastMessage);
        umami.track("unstar-event", { event_id: eventId.value });
      }
    },
  });
};

export default useIsEventStarred;
