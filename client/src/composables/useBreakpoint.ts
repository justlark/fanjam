import { readonly, ref, type Ref } from "vue";

export type Breakpoint = "sm" | "md" | "lg" | "xl" | "2xl";

const minWidth = (breakpoint: Breakpoint): string =>
  getComputedStyle(document.documentElement).getPropertyValue(`--breakpoint-${breakpoint}`).trim();

const cache = new Map<Breakpoint, Readonly<Ref<boolean>>>();

// Whether the viewport is at least as wide as the given Tailwind breakpoint.
const useBreakpoint = (breakpoint: Breakpoint): Readonly<Ref<boolean>> => {
  const cached = cache.get(breakpoint);

  if (cached !== undefined) {
    return cached;
  }

  const width = minWidth(breakpoint);

  const query = window.matchMedia(width ? `(min-width: ${width})` : "not all");
  const matches = ref(query.matches);

  query.addEventListener("change", (event) => {
    matches.value = event.matches;
  });

  const result = readonly(matches);
  cache.set(breakpoint, result);

  return result;
};

export default useBreakpoint;
