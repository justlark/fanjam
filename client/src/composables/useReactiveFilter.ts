import { ref, watch, toValue, type Ref, type MaybeRefOrGetter } from "vue";

export const useReactiveFilter = <T>(
  input: Ref<T[]>,
  predicate: (item: T, index: number) => boolean,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<T>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  const { chunkSize, sources } = options;

  const output = ref<T[]>([]);

  let runId = 0;

  const run = () => {
    const currentRun = ++runId;
    const source = toValue(input);

    output.value.length = 0;

    let i = 0;

    const step = () => {
      if (currentRun !== runId) return;

      const end = Math.min(i + chunkSize, source.length);

      for (; i < end; i++) {
        const item = source[i];
        if (predicate(item, i)) {
          (output.value as Array<T>).push(item);
        }
      }

      if (i < source.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  };

  watch([input, ...sources], run, { immediate: true });

  return output;
};

export default useReactiveFilter;
