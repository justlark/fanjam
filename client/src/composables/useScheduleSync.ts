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

// True while a local change is queued or being pushed. Pulls skip applying the server's schedule
// while this holds, so a "star then immediately refresh" can't clobber the just-starred event with
// stale server data.
const isDirty = (): boolean => pushTimer !== undefined || pushInFlight;

const serialize = (events: Set<string>): string => [...events].sort().join(",");

const storageKey = (envId: string): string => `sync:${envId}`;

const resetState = () => {
  if (pushTimer !== undefined) {
    clearTimeout(pushTimer);
    pushTimer = undefined;
  }
  lastServerSchedule = undefined;
  ready = false;
  pushInFlight = false;
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
    }
  };

  const schedulePush = () => {
    if (pushTimer !== undefined) clearTimeout(pushTimer);
    pushTimer = setTimeout(() => {
      pushTimer = undefined;
      void doPush();
    }, PUSH_DEBOUNCE_MS);
  };

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
    ready = false;
  };

  // Pull the latest schedule from the server and adopt it (last-write-wins). Called on app load and
  // whenever the rest of the app data is refreshed.
  const pullSchedule = async () => {
    const code = syncCode.value;
    if (!code || isSharedSchedule.value) {
      ready = true;
      return;
    }

    // A local change is pending; our queued push is the newer truth, so don't pull over it.
    if (isDirty()) return;

    const result = await api.getSchedule(envId.value, code);

    if (!result.ok) {
      if (result.code === 404) {
        // The sync code is dead (unknown or expired). Clear it so we stop trying to sync.
        stopSync();
      }
      ready = true;
      return;
    }

    // A local change landed while we were fetching — don't clobber it with the (now stale) server
    // schedule. The pending push will reconcile the server.
    if (isDirty()) {
      ready = true;
      return;
    }

    const serverSorted = [...result.value].sort();
    lastServerSchedule = serverSorted.join(",");

    if (lastServerSchedule !== serialize(starredEvents.value)) {
      starredEvents.value = new Set(result.value);
    }

    ready = true;
  };

  return { isSyncing, syncLink, syncEmojis, enableSync, stopSync, pullSchedule };
};

export default useScheduleSync;
