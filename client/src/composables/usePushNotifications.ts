import { ref, reactive, computed, watch, type ComputedRef } from "vue";
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

// Set when the user declines the push prompt toast (or unsubscribes), so we
// don't nag them again for that env.
export const promptDismissedKey = (envId: string): string => `push-prompt-dismissed:${envId}`;

const SUBSCRIPTION_KEY_PREFIX = "subscription:";
const subscriptionKey = (envId: string): string => `${SUBSCRIPTION_KEY_PREFIX}${envId}`;

// `Notification.permission` and the browser's push subscription are shared
// across all composable consumers — both are global (per-origin), so there's
// no reason to recompute them per component instance.
//
// Note that on the default hostname, every env lives on the same origin, so
// they all share one permission and one push subscription. Whether the user
// has opted into a *particular* env is tracked separately, in `localStorage`
// under `subscription:<envId>`, whose value is a hash of the endpoint we last
// sent to the server for that env.
const permission = ref<NotificationPermission | "unsupported">(detectInitialPermission());

// `undefined` until we've probed the service worker registration.
const browserSubscription = ref<PushSubscription | null | undefined>(undefined);

// Reactive overlay of the `subscription:<envId>` keys in `localStorage`. Only
// `setStoredId` writes to it; reads fall back to `localStorage` for envs it
// hasn't seen. Reading is side-effect free so it's safe inside a computed, and
// the `in` check tracks the key so dependents update once it's set.
const storedIds = reactive<Record<string, string | null>>({});

const getStoredId = (envId: string): string | null =>
  envId in storedIds ? (storedIds[envId] ?? null) : localStorage.getItem(subscriptionKey(envId));

const setStoredId = (envId: string, id: string | null): void => {
  if (id === null) {
    localStorage.removeItem(subscriptionKey(envId));
  } else {
    localStorage.setItem(subscriptionKey(envId), id);
  }
  storedIds[envId] = id;
};

const hasAnyStoredId = (): boolean => {
  for (let i = 0; i < localStorage.length; i++) {
    if (localStorage.key(i)?.startsWith(SUBSCRIPTION_KEY_PREFIX)) return true;
  }
  return false;
};

let probe: Promise<void> | undefined;

// Resolve the browser's current push subscription, probing the SW
// registration the first time. Later calls read the ref, which
// `requestAndSubscribe` and `unsubscribe` keep up to date.
const getBrowserSubscription = async (): Promise<PushSubscription | null> => {
  probe ??= (async () => {
    try {
      const registration = await navigator.serviceWorker.ready;
      const existing = await registration.pushManager.getSubscription();
      browserSubscription.value ??= existing;
    } catch {
      // Leave it `undefined`; we just don't know. Clear the cached probe so
      // the next call retries.
      probe = undefined;
    }
  })();
  await probe;
  return browserSubscription.value ?? null;
};

// Envs with a reconciliation POST in flight, so multiple consumers of this
// composable don't send duplicate requests.
const reconciling = new Set<string>();

// If the user previously opted into this env but the browser's subscription
// has changed since (SW reinstall, key rotation), re-POST it so the server
// has a fresh record. Envs the user never opted into are left alone; sharing
// a browser subscription with another env is not consent.
const reconcile = async (envId: string): Promise<void> => {
  if (!envId || reconciling.has(envId)) return;

  // Claim the env before the first `await`, so a concurrent call from another
  // consumer in the same tick sees it.
  reconciling.add(envId);
  try {
    const existing = await getBrowserSubscription();
    if (!existing) return;

    const storedId = getStoredId(envId);
    const id = endpointId(existing.endpoint);
    if (storedId === null || storedId === id) return;

    const result = await api.postSubscription(envId, existing.toJSON());
    if (result.ok) {
      setStoredId(envId, id);
    }
  } finally {
    reconciling.delete(envId);
  }
};

const usePushNotifications = (): {
  state: ComputedRef<PushState>;
  requestAndSubscribe: () => Promise<PushState>;
  unsubscribe: () => Promise<boolean>;
} => {
  const envId = useEnvId();

  if (permission.value !== "unsupported") {
    watch(envId, (id) => void reconcile(id), { immediate: true });
  }

  const state = computed<PushState>(() => {
    if (permission.value === "unsupported") return "unsupported";
    if (permission.value === "denied") return "denied";
    if (permission.value === "default") return "default";
    return browserSubscription.value && getStoredId(envId.value) !== null
      ? "granted-subscribed"
      : "granted-unsubscribed";
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
      return state.value;
    }

    try {
      const registration = await navigator.serviceWorker.ready;
      // If another env already created a subscription on this origin, this
      // returns that same subscription.
      const subscription = await registration.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey: decodeBase64urlBytes(publicKey),
      });
      browserSubscription.value = subscription;

      const result = await api.postSubscription(envId.value, subscription.toJSON());
      if (result.ok) {
        setStoredId(envId.value, endpointId(subscription.endpoint));
      }
    } catch {
      // Fall through; `state` reflects whatever we managed to do.
    }

    return state.value;
  };

  // Unsubscribe from the current env only. Returns whether it succeeded.
  const unsubscribe = async (): Promise<boolean> => {
    if (permission.value === "unsupported") return false;

    const existing = await getBrowserSubscription();
    if (existing) {
      const result = await api.deleteSubscription(envId.value, existing.endpoint);
      if (!result.ok) return false;
    }

    setStoredId(envId.value, null);
    localStorage.setItem(promptDismissedKey(envId.value), "true");

    // If no other env on this origin is using the browser subscription, drop
    // it entirely rather than leaving a dangling endpoint around.
    if (existing && !hasAnyStoredId()) {
      try {
        await existing.unsubscribe();
        browserSubscription.value = null;
      } catch {
        // The server no longer has it for this env, which is what matters.
      }
    }

    return true;
  };

  return { state, requestAndSubscribe, unsubscribe };
};

export default usePushNotifications;
