import { ref, computed, watch } from "vue";
import api from "@/utils/api";
import { generateSyncCode } from "@/utils/encoding";
import { useAppUrl } from "./useAppUrl";
import useEnvId from "./useEnvId";
import useIsSharedSchedule from "./useIsSharedSchedule";
import useStarredEvents from "./useStarredEvents";
import emojiHasher, { type EmojiHasher } from "emoji-hash-gen";

// How long after a starring change we wait before pushing to the server, so a flurry of toggles
// collapses into a single PUT.
const PUSH_DEBOUNCE_MS = 1000;

const EMOJI_LIST = [
  "🐱",
  "🐶",
  "🦊",
  "🐻",
  "🐮",
  "🐷",
  "🦁",
  "🌳",
  "✈️",
  "🥕",
  "🔥",
  "❤️",
  "⭐",
  "🎉",
  "🪲",
  "⚡",
  "🪐",
  "🍎",
  "🧭",
  "⚓",
  "🚗",
  "🚀",
  "🪩",
  "🏆",
  "⚽",
  "🎲",
  "🕹️",
  "🎺",
  "🎸",
  "💎",
  "💡",
  "👑",
  "☂️",
  "🔨",
  "✏️",
  "📫",
  "⏰",
  "🔑",
  "🍄",
  "🥪",
  "🚩",
  "🎂",
  "🏈",
  "🎱",
  "♟️",
  "☎️",
  "🧳",
  "💊",
  "📢",
  "🌈",
  "☀️",
  "🍒",
  "☕",
  "🚢",
  "🎈",
  "👍️",
  "📷",
  "🧵",
  "🥁",
  "🎤",
  "💰",
  "🔦",
  "🧼",
  "🐝",
];

(emojiHasher as EmojiHasher).useTable(
  EMOJI_LIST.reduce((obj, emoji, index) => ({ [index]: emoji, ...obj }), {}),
);

// Module-singleton sync state, shared across every component that uses this composable. Keyed to
// the current environment via `currentEnvId`; switching environments resets everything.
const syncCode = ref<string | undefined>();
const currentEnvId = ref<string>();

// The schedule we last know the server holds (sorted, comma-joined), used to suppress redundant
// pushes — in particular the echo when a pull writes the server's schedule into `starredEvents`.
let lastServerSchedule: string | undefined;

// Becomes true once the first pull (or `enableSync`) has resolved. Until then we don't push, so the
// initial load of the device's local stars never clobbers a newer server schedule before we've had
// a chance to pull it.
let ready = false;

let pushTimer: ReturnType<typeof setTimeout> | undefined;
let pushInFlight = false;

// A local change the server hasn't acknowledged yet. This has to outlive the tab, not just the
// failed request: someone who stars an event with no signal and then closes the app would
// otherwise come back to a pull that overwrites the change with the server's older schedule.
// Set the moment a push is queued, cleared only once one actually lands.
let pendingPush = false;

// Set by `useScheduleSync()` so the `online` listener below can flush a failed push the moment
// the network comes back. Every instance closes over the same module-level state, and over refs
// (the route, the starred events) that are themselves app-global, so whichever instance we're
// holding stays correct.
let flushPendingPush: (() => void) | undefined;

// True while a local change is queued, being pushed, or waiting to be retried after a push we
// couldn't deliver. Pulls skip applying the server's schedule while this holds, so neither a
// "star then immediately refresh" nor a reconnect after starring offline can clobber the local
// change with the server's older copy.
const isDirty = (): boolean => pushTimer !== undefined || pushInFlight || pendingPush;

if (typeof window !== "undefined") {
  window.addEventListener("online", () => {
    if (pendingPush) flushPendingPush?.();
  });
}

const serialize = (events: Set<string>): string => [...events].sort().join(",");

const storageKey = (envId: string): string => `sync:${envId}`;

const pendingPushKey = (envId: string): string => `sync-pending:${envId}`;

const setPendingPush = (value: boolean) => {
  pendingPush = value;

  const envId = currentEnvId.value;
  if (envId === undefined) return;

  if (value) {
    localStorage.setItem(pendingPushKey(envId), "1");
  } else {
    localStorage.removeItem(pendingPushKey(envId));
  }
};

const resetState = () => {
  if (pushTimer !== undefined) {
    clearTimeout(pushTimer);
    pushTimer = undefined;
  }
  lastServerSchedule = undefined;
  ready = false;
  pushInFlight = false;
  pendingPush = false;
};

const useScheduleSync = () => {
  const envId = useEnvId();
  const appUrl = useAppUrl();
  const isSharedSchedule = useIsSharedSchedule();
  const starredEvents = useStarredEvents();

  if (currentEnvId.value === undefined || currentEnvId.value !== envId.value) {
    resetState();
    syncCode.value = localStorage.getItem(storageKey(envId.value)) ?? undefined;
    currentEnvId.value = envId.value;
    pendingPush = localStorage.getItem(pendingPushKey(envId.value)) !== null;
  }

  const isSyncing = computed(() => syncCode.value !== undefined);

  const syncLink = computed(() => (syncCode.value ? appUrl(`sync/?s=${syncCode.value}`) : ""));

  const syncEmojis = computed(() => {
    if (syncCode.value === undefined) return "";

    const hasher = emojiHasher as EmojiHasher;

    // Force the hash to be non-negative.
    const hash = hasher.getBitwise(`${envId.value}:${syncCode.value}`) >>> 0;

    // This misbehaves when the given has is negative.
    return hasher.transformBinary(hash, { length: 3, base: EMOJI_LIST.length });
  });

  const doPush = async () => {
    const code = syncCode.value;
    if (!code) return;

    const schedule = [...starredEvents.value].sort();
    const serialized = schedule.join(",");

    pushInFlight = true;
    const result = await api.putSchedule(envId.value, code, schedule);
    pushInFlight = false;

    if (result.ok) {
      lastServerSchedule = serialized;
      setPendingPush(false);
    } else {
      // We never replaced the server's copy, so `lastServerSchedule` must not move. Staying
      // dirty is what stops the next pull from overwriting this device's stars with the older
      // schedule we just failed to replace.
      setPendingPush(true);
    }
  };

  const schedulePush = () => {
    // Record the divergence now rather than when the push fires. The debounce window is small,
    // but it's long enough to close the app in, and a change we never attempted is just as lost
    // as one we attempted and couldn't deliver.
    setPendingPush(true);

    if (pushTimer !== undefined) clearTimeout(pushTimer);
    pushTimer = setTimeout(() => {
      pushTimer = undefined;
      void doPush();
    }, PUSH_DEBOUNCE_MS);
  };

  flushPendingPush = schedulePush;

  // Push on change. Registered without `immediate`, so it never fires for the initial load of
  // local stars — only for genuine changes once syncing is enabled and ready.
  watch(
    starredEvents,
    () => {
      if (!ready || !isSyncing.value || isSharedSchedule.value) return;

      // Skip if nothing changed relative to the server (this is also how we avoid echoing a pull
      // that just wrote the server's schedule into `starredEvents`).
      if (serialize(starredEvents.value) === lastServerSchedule) return;

      schedulePush();
    },
    { deep: true },
  );

  // Enable syncing for this device: mint a code, persist it, and seed the server with the current
  // schedule. No-op if already syncing (the dialog just reuses the existing link).
  const enableSync = async () => {
    if (syncCode.value) return;

    const code = generateSyncCode();
    const schedule = [...starredEvents.value].sort();

    const result = await api.putSchedule(envId.value, code, schedule);
    if (!result.ok) return;

    syncCode.value = code;
    localStorage.setItem(storageKey(envId.value), code);
    lastServerSchedule = schedule.join(",");
    ready = true;
  };

  // Stop syncing on this device only. Local stars are left untouched; the server entry is left to
  // expire via its TTL so other devices keep syncing under the same code.
  const stopSync = () => {
    if (pushTimer !== undefined) {
      clearTimeout(pushTimer);
      pushTimer = undefined;
    }
    syncCode.value = undefined;
    localStorage.removeItem(storageKey(envId.value));
    lastServerSchedule = undefined;
    setPendingPush(false);
    ready = false;
  };

  // Pull the latest schedule from the server and adopt it (last-write-wins). Called on app load and
  // whenever the rest of the app data is refreshed.
  const pullSchedule = async () => {
    // However this turns out, the push watcher has to come out of the hold it starts in.
    // Leaving `ready` false silently drops every star the user makes for the rest of the
    // session — which is exactly what used to happen when this threw on an offline device.
    try {
      const code = syncCode.value;
      if (!code || isSharedSchedule.value) return;

      // A push we couldn't deliver is still the newer truth. Retry it rather than pulling:
      // pulling here would hand us back the older schedule we failed to replace. Navigating is
      // the most common way a user comes back from a dead spot, and this runs on every
      // navigation, so it doubles as the retry the `online` event might never fire.
      if (pendingPush) {
        await doPush();
        return;
      }

      // A local change is queued; our push is the newer truth, so don't pull over it.
      if (isDirty()) return;

      const result = await api.getSchedule(envId.value, code);

      if (!result.ok) {
        if (result.code === 404) {
          // The sync code is dead (unknown or expired). Clear it so we stop trying to sync.
          stopSync();
        }

        // Anything else — an unreachable network included — tells us nothing about the
        // server's schedule, so we leave this device's stars exactly as they are.
        return;
      }

      // A local change landed while we were fetching — don't clobber it with the (now stale)
      // server schedule. The pending push will reconcile the server.
      if (isDirty()) return;

      const serverSorted = [...result.value].sort();
      lastServerSchedule = serverSorted.join(",");

      if (lastServerSchedule !== serialize(starredEvents.value)) {
        starredEvents.value = new Set(result.value);
      }
    } finally {
      ready = true;
    }
  };

  return { isSyncing, syncLink, syncEmojis, enableSync, stopSync, pullSchedule };
};

export default useScheduleSync;
