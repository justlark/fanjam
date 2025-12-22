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

  const run = () => {
    const source = toValue(input);

    output.value.length = 0;

    let i = 0;

    const step = () => {
      const end = Math.min(i + chunkSize, source.length);

      for (; i < end; i++) {
        (output.value as Array<R>).push(mapper(source[i], i));
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

export default useReactiveMap;
