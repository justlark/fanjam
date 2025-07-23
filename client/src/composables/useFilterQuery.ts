import { watch, reactive, type Reactive } from "vue";
import { useRoute, useRouter } from "vue-router";
import { isNotNullish } from "@/utils/types";

export interface FilterCriteria {
  categories: Array<string>;
  tags: Array<string>;
}

export const useFilterQuery = (): Reactive<FilterCriteria> => {
  const route = useRoute();
  const router = useRouter();

  const criteria = reactive<FilterCriteria>({
    categories: [],
    tags: [],
  });

  watch(
    route,
    (newRoute) => {
      if (newRoute.query.c) {
        criteria.categories = Array.isArray(newRoute.query.c)
          ? newRoute.query.c.filter(isNotNullish)
          : [newRoute.query.c];
      } else {
        criteria.categories = [];
      }

      if (newRoute.query.t) {
        criteria.tags = Array.isArray(newRoute.query.t)
          ? newRoute.query.t.filter(isNotNullish)
          : [newRoute.query.t];
      } else {
        criteria.tags = [];
      }
    },
    { immediate: true },
  );

  watch(criteria, async (newCriteria) => {
    await router.replace({
      query: {
        c: newCriteria.categories,
        t: newCriteria.tags,
      },
    });
  });

  return criteria;
};

export const toFilterQueryParams = (criteria: Reactive<FilterCriteria>) => ({
  c: criteria.categories,
  t: criteria.tags,
});

export default useFilterQuery;
