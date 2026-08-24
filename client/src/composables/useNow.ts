import { ref, type Ref } from "vue";
import useInterval from "./useInterval";

// Anything derived from the current time is at most this stale.
const REFRESH_NOW_TIME_INTERVAL_MILLIS = 1000 * 60 * 1;

// The current time, as a reactive value.
//
// Reading `new Date()` inside a computed does not work: the clock is not a
// reactive dependency, so the computed caches its first answer and never
// recomputes. Anything that changes as time passes — which time slot is
// current, which day counts as today — needs to derive from this instead.
const useNow = (): Readonly<Ref<Date>> => {
  const now = ref(new Date());

  useInterval(() => (now.value = new Date()), REFRESH_NOW_TIME_INTERVAL_MILLIS);

  return now;
};

export default useNow;
