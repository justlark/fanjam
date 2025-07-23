import { type RouteLocationNormalizedGeneric } from "vue-router";
import { isNotNullish } from "./types";

export const rawQueryParamToArray = (
  route: RouteLocationNormalizedGeneric,
  param: (typeof QueryParam)[keyof typeof QueryParam],
): Array<string> | undefined => {
  const raw = route.query[param];

  if (raw === null || raw === undefined) {
    return undefined;
  }

  return Array.isArray(raw) ? raw.filter(isNotNullish) : raw.split(",");
};

export const QueryParam = {
  categories: "category",
  tags: "tag",
} as const;
