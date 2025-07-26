import { watch, reactive, type Reactive } from "vue";
import { useRoute, useRouter } from "vue-router";
import { isNotNullish } from "@/utils/types";

export interface FilterCriteria {
  categories: Array<string>;
  tags: Array<string>;
  search: string;
  hidePastEvents: boolean;
}

export const useFilterQuery = (): Reactive<FilterCriteria> => {
  const route = useRoute();
  const router = useRouter();

  const criteria = reactive<FilterCriteria>({
    categories: [],
    tags: [],
    search: "",
    hidePastEvents: false,
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

      if (newRoute.query.q) {
        criteria.search = Array.isArray(newRoute.query.q)
          ? newRoute.query.q.join(" ")
          : newRoute.query.q;
      } else {
        criteria.search = "";
      }

      if (newRoute.query.past === "false") {
        criteria.hidePastEvents = true;
      } else {
        criteria.hidePastEvents = false;
      }
    },
    { immediate: true },
  );

  watch(criteria, async (newCriteria) => {
    await router.replace({
      query: {
        c: newCriteria.categories,
        t: newCriteria.tags,
        q: newCriteria.search || undefined,
        past: newCriteria.hidePastEvents ? "false" : undefined,
      },
    });
  });

  return criteria;
};

export const toFilterQueryParams = (criteria: Reactive<FilterCriteria>) => ({
  c: criteria.categories,
  t: criteria.tags,
  q: criteria.search,
  past: criteria.hidePastEvents ? "false" : undefined,
});

export default useFilterQuery;
