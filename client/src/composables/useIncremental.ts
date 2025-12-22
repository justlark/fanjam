import { shallowRef, readonly, watch, type Ref, type MaybeRefOrGetter } from "vue";

export const useIncremental = <T>(
  input: Ref<T[]>,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<unknown>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  const { chunkSize, sources } = options;

  const output = shallowRef<Array<T>>([]);
  let runCounter = 0;

  const run = () => {
    const currentRun = ++runCounter;

    output.value = [];

    let i = 0;

    const step = () => {
      if (currentRun !== runCounter) return;

      const end = Math.min(i + chunkSize, input.value.length);

      output.value = output.value.concat(input.value.slice(i, end));

      i = end;

      if (i < input.value.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  };

  watch([input, ...sources], run, { immediate: true });

  return readonly(output);
};

export default useIncremental;
