import { shallowRef, onMounted, watch, triggerRef, type Ref, type MaybeRefOrGetter } from "vue";

export function useIncremental<T>(
  input: Readonly<Ref<ReadonlyArray<T>>>,
  options: {
    chunkSize: number;
    sources: Array<MaybeRefOrGetter<unknown>>;
  } = {
      chunkSize: 5,
      sources: [],
    },
) {
  const { chunkSize, sources } = options;

  const output = shallowRef<Array<T>>([]);
  let runCounter = 0;

  const run = () => {
    const currentRun = ++runCounter;

    output.value.length = 0;
    triggerRef(output);

    let i = 0;

    const step = () => {
      if (currentRun !== runCounter) return;

      const end = Math.min(i + chunkSize, input.value.length);

      for (; i < end; i++) {
        output.value.push(input.value[i]);
      }

      triggerRef(output);

      if (i < input.value.length) {
        requestAnimationFrame(step);
      }
    };

    requestAnimationFrame(step);
  };

  onMounted(() => {
    watch([() => input.value.length, ...sources], run, { immediate: true });
  });

  return output;
}

export default useIncremental;
