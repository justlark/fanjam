import { readonly, ref, type Ref } from "vue";

// Whether the browser currently believes it has a network connection, as a reactive value.
const online = ref(typeof navigator === "undefined" ? true : navigator.onLine);

if (typeof window !== "undefined") {
  window.addEventListener("online", () => (online.value = true));
  window.addEventListener("offline", () => (online.value = false));
}

const useOnline = (): Readonly<Ref<boolean>> => readonly(online);

export default useOnline;
