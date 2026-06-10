import { ref, computed, onMounted, type ComputedRef } from "vue";
import api from "@/utils/api";
import { decodeBase64urlBytes, djb2Hash } from "@/utils/encoding";
import useEnvId from "./useEnvId";

type PushState =
  | "unsupported"
  | "default"
  | "denied"
  | "granted-subscribed"
  | "granted-unsubscribed";

const detectInitialPermission = (): NotificationPermission | "unsupported" => {
  if (typeof window === "undefined") return "unsupported";
  if (
    !("Notification" in window) ||
    !("PushManager" in window) ||
    !("serviceWorker" in navigator)
  ) {
    return "unsupported";
  }
  return Notification.permission;
};

const endpointId = (endpoint: string): string => djb2Hash(endpoint).toString(16);

// `Notification.permission` shared across all composable consumers — the
// state is global (per-origin), so there's no reason to recompute it per
// component instance.
const permission = ref<NotificationPermission | "unsupported">(detectInitialPermission());
const subscribed = ref<boolean | undefined>(undefined);

const usePushNotifications = (): {
  state: ComputedRef<PushState>;
  requestAndSubscribe: () => Promise<PushState>;
} => {
  const envId = useEnvId();

  // Probe the SW registration on mount so the initial state reflects whether
  // we already have a subscription cached in the browser. If the browser
  // hands us a subscription we haven't told the server about (SW reinstall,
  // key rotation, a fresh tab on a different device), re-POST it so the
  // server has a fresh record.
  onMounted(async () => {
    if (permission.value === "unsupported") return;
    try {
      const registration = await navigator.serviceWorker.ready;
      const existing = await registration.pushManager.getSubscription();
      subscribed.value = existing !== null;

      if (existing && envId.value) {
        const key = `subscription:${envId.value}:${endpointId(existing.endpoint)}`;
        if (localStorage.getItem(key) === null) {
          const result = await api.postSubscription(envId.value, existing.toJSON());
          if (result.ok) {
            localStorage.setItem(key, "true");
          }
        }
      }
    } catch {
      subscribed.value = undefined;
    }
  });

  const state = computed<PushState>(() => {
    if (permission.value === "unsupported") return "unsupported";
    if (permission.value === "denied") return "denied";
    if (permission.value === "default") return "default";
    return subscribed.value ? "granted-subscribed" : "granted-unsubscribed";
  });

  const requestAndSubscribe = async (): Promise<PushState> => {
    if (permission.value === "unsupported") return "unsupported";

    if (permission.value !== "granted") {
      const result = await Notification.requestPermission();
      permission.value = result;
      if (result !== "granted") return state.value;
    }

    const publicKey = (import.meta.env.VITE_VAPID_PUBLIC_KEY as string | undefined)?.trim();
    if (!publicKey) {
      subscribed.value = false;
      return state.value;
    }

    try {
      const registration = await navigator.serviceWorker.ready;
      const subscription = await registration.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey: decodeBase64urlBytes(publicKey),
      });
      const result = await api.postSubscription(envId.value, subscription.toJSON());
      if (result.ok) {
        const key = `subscription:${envId.value}:${endpointId(subscription.endpoint)}`;
        localStorage.setItem(key, "true");
        subscribed.value = true;
      } else {
        subscribed.value = false;
      }
    } catch {
      subscribed.value = false;
    }

    return state.value;
  };

  return { state, requestAndSubscribe };
};

export default usePushNotifications;
