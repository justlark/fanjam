import { onMounted, onUnmounted, ref, type Ref } from "vue";

const useInterval = (action: () => void, interval: number): Ref<number | undefined> => {
  const intervalId = ref<number>();

  onMounted(() => {
    intervalId.value = setInterval(action, interval);
  });

  onUnmounted(() => {
    if (intervalId.value !== undefined) {
      clearInterval(intervalId.value);
    }
  });
  
  return intervalId;
};

export default useInterval;