import { type Ref, ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";
import api, { type Event } from "@/utils/api";

// TODO: Eventually, we'll want to persist this data to the local storage so
// the app can work offline.
export const useEvents = (): Readonly<Ref<Array<Event>>> => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);

  const events = ref<Array<Event>>([]);

  watchEffect(async () => {
    const eventsResult = await api.getEvents(envId.value);

    if (eventsResult.ok) {
      events.value = eventsResult.value;
    } else {
      events.value = [];
    }
  });

  return events;
};

export default useEvents;
