import { ref, watch, toValue, type Ref } from "vue";

export function useReactiveFilter<T>(
  input: Ref<T[]>,
  predicate: (item: T, index: number) => boolean,
  chunkSize = 5,
) {
  const output = ref<T[]>([]);

  let runId = 0;

  function run() {
    const currentRun = ++runId;
    const source = toValue(input);

    output.value = [];

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
  }

  watch(input, run, { immediate: true });

  return output;
}

export default useReactiveFilter;
