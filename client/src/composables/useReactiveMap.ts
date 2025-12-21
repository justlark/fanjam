import { ref, watch, toValue, type Ref, type MaybeRefOrGetter } from "vue";

export const useReactiveMap = <T, R>(
  input: Ref<Array<T>>,
  mapper: (item: T, index: number) => R,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<T>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) => {
  const { chunkSize, sources } = options;

  const output = ref<R[]>([]);
  let runId = 0;

  function run() {
    const currentRun = ++runId;
    const source = toValue(input);

    output.value = new Array(source.length);

    let i = 0;

    const step = () => {
      if (currentRun !== runId) return;

      const end = Math.min(i + chunkSize, source.length);

      for (; i < end; i++) {
        (output.value[i] as R) = mapper(source[i], i);
      }

      if (i < source.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  }

  watch([input, ...sources], run, { immediate: true });

  return output;
};

export default useReactiveMap;
