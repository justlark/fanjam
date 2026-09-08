import { shallowRef, triggerRef, watch, type Ref } from "vue";
import { isTest } from "@/utils/env";
import deepEqual from "fast-deep-equal";

// How much of a frame to spend appending before yielding. We don't want a
// large schedule to get spread over dozens of frames. If we just set a fixed
// number of items per frame, that would limit how fast the list could possibly
// render.
const FRAME_BUDGET_MS = 8;

// How many items to append between checking the clock. Reading the clock per
// item would cost too much.
const ITEMS_PER_CLOCK_CHECK = 8;

const useIncremental = <T>(input: Readonly<Ref<ReadonlyArray<T>>>) => {
  const output = shallowRef<Array<T>>([]);
  let runCounter = 0;

  const run = () => {
    const currentRun = ++runCounter;

    const next: Array<T> = [];
    let i = 0;

    const step = () => {
      if (currentRun !== runCounter) return;

      const deadline = performance.now() + FRAME_BUDGET_MS;

      while (i < input.value.length) {
        const end = Math.min(i + ITEMS_PER_CLOCK_CHECK, input.value.length);

        for (; i < end; i++) {
          next.push(input.value[i]);
        }

        if (performance.now() >= deadline) break;
      }

      // Swapping the new array in as it fills, rather than emptying the old one
      // up front, keeps the previous list on screen until there is something to
      // replace it with.
      output.value = next;
      triggerRef(output);

      if (i < input.value.length) {
        requestAnimationFrame(step);
      }
    };

    // The first chunk runs now rather than a frame from now, so whatever fits
    // above the fold is visible immediately.
    step();
  };

  watch(
    input,
    (newInput, oldInput) => {
      if (deepEqual(newInput, oldInput)) return;

      run();
    },
    { immediate: true },
  );

  return output;
};

// The `requestAnimationFrame` trick breaks Playwright, so we need to avoid
// using it in tests.
const useIncrementalExceptInTests = <T>(input: Readonly<Ref<ReadonlyArray<T>>>) => {
  if (isTest()) {
    return input;
  } else {
    return useIncremental(input);
  }
};

export default useIncrementalExceptInTests;
