import { reactive, readonly, watch, toValue, type MaybeRefOrGetter } from "vue";

export const useIncremental = <T>(
  input: MaybeRefOrGetter<T[]>,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<T>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  const { chunkSize, sources } = options;

  const output: Array<T> = reactive([]);
  let runCounter = 0;

  const run = () => {
    const currentRun = ++runCounter;

    output.length = 0;

    let i = 0;

    const step = () => {
      const source = toValue(input);

      if (currentRun !== runCounter) return;

      const end = Math.min(i + chunkSize, source.length);

      for (; i < end; i++) {
        output.push(source[i]);
      }

      if (i < source.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  };

  watch([input, ...sources], run, { immediate: true });

  return readonly(output);
};

export default useIncremental;
