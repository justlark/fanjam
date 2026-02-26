import { computed } from "vue";
import { useRoute } from "vue-router";

const useIsSharedSchedule = () => {
  const route = useRoute();

  return computed(() => Boolean(route.query.share));
};

export default useIsSharedSchedule;
