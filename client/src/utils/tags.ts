import { type Event } from "@/utils/api";

export const getSortedCategories = (events: Array<Event>): Array<string> => {
  const sortedEvents = [...events];
  sortedEvents.sort((a, b) => a.startTime.valueOf() - b.startTime.valueOf());

  return sortedEvents.reduce<Array<string>>((set, event) => {
    if (!set.includes(event.category)) {
      set.push(event.category);
    }
    return set;
  }, []);
};
