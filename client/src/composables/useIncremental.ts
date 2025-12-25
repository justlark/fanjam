import { ref, watch, type Ref, type MaybeRefOrGetter } from "vue";
import { isTest } from "@/utils/env";

const useIncremental = <T>(
  input: Readonly<Ref<ReadonlyArray<T>>>,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<unknown>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  const { chunkSize, sources } = options;

  const output = ref<Array<T>>([]);
  let runCounter = 0;

  const run = () => {
    const currentRun = ++runCounter;

    output.value.length = 0;

    let i = 0;

    const step = () => {
      if (currentRun !== runCounter) return;

      const end = Math.min(i + chunkSize, input.value.length);

      for (; i < end; i++) {
        (output.value as Array<T>).push(input.value[i]);
      }

      if (i < input.value.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  };

  watch([input, ...sources], run, { immediate: true });

  return output;
};

// The `requestAnimationFrame` trick breaks Playwright, so we need to avoid
// using it in tests.
const useIncrementalExceptInTests = <T>(
  input: Readonly<Ref<ReadonlyArray<T>>>,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<unknown>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  if (isTest()) {
    return input;
  } else {
    return useIncremental(input, options);
  }
};

export default useIncrementalExceptInTests;
