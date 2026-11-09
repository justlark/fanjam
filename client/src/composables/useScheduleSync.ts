import { ref, computed, watch, nextTick } from "vue";
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

// Every device that has ever starred an event holds a sync code, and its schedule is always pushed
// to the server under that code — whether or not the user has ever opened the sync dialog. This
// keeps per-event star counts available for analytics while staying anonymous: the code is random,
// generated on the device, and the server only ever sees it alongside a list of event IDs. Devices
// that sync together share one code, so they count as one attendee. "Syncing" a second device just
// means handing it this device's code.
//
// Module-singleton sync state, shared across every component that uses this composable. Keyed to
// the current environment via `currentEnvId`; switching environments resets everything.
const syncCode = ref<string | undefined>();
const currentEnvId = ref<string>();

// The schedule we last know the server holds (sorted, comma-joined), used to suppress redundant
// pushes — in particular the echo when a pull writes the server's schedule into `starredEvents`.
let lastServerSchedule: string | undefined;

// Becomes true once the first pull has resolved (or as soon as we mint a code, since there's
// nothing on the server to pull yet). Until then the push watcher holds off, so the initial load of
// the device's local stars never clobbers a newer server schedule before we've had a chance to pull
// it.
let ready = false;

// What was on screen when the first pull started. On a slow connection the user can easily star
// something before that pull comes back; the watcher won't push it, and the pull would otherwise
// replace it with the server's copy. Diffing against this snapshot recovers those changes so they
// can be layered on top of the server's schedule instead. Only set while `ready` is false.
let heldSnapshot: Set<string> | undefined;

// Events the user explicitly added while `ready` was false (see `addStars`). Unlike changes picked
// up by diffing against `heldSnapshot`, these count no matter whether the pull's snapshot was taken
// before or after they were added.
let heldAdds = new Set<string>();

// Every instance watches for the user leaving a shared schedule; this makes sure only one of them
// pulls in response.
let shareExitPullQueued = false;

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

const serialize = (events: Set<string>): string => [...events].sort().join(",");

const storageKey = (envId: string): string => `sync:${envId}`;

const pendingPushKey = (envId: string): string => `sync-pending:${envId}`;

// Codes this device has stopped using (because it joined another device's sync link, or stopped
// syncing) whose server entries we still need to delete. Left behind, they'd keep counting this
// device's old schedule as a separate attendee until their TTL ran out. Persisted, because the
// device might be offline when it switches codes.
const abandonedKey = (envId: string): string => `sync-abandoned:${envId}`;

const getAbandonedCodes = (envId: string): Array<string> => {
  try {
    const parsed: unknown = JSON.parse(localStorage.getItem(abandonedKey(envId)) ?? "[]");
    return Array.isArray(parsed) ? parsed.filter((code) => typeof code === "string") : [];
  } catch {
    return [];
  }
};

const setAbandonedCodes = (envId: string, codes: Array<string>) => {
  if (codes.length === 0) {
    localStorage.removeItem(abandonedKey(envId));
  } else {
    localStorage.setItem(abandonedKey(envId), JSON.stringify(codes));
  }
};

const abandonCode = (envId: string, code: string) => {
  const codes = getAbandonedCodes(envId);
  if (!codes.includes(code)) setAbandonedCodes(envId, [...codes, code]);
};

let deletingAbandoned = false;

// Best-effort, in the background: delete the server entries for every abandoned code. Whatever we
// can't reach stays queued for the next attempt (app load or the network coming back).
const deleteAbandonedCodes = async (envId: string) => {
  if (deletingAbandoned) return;
  deletingAbandoned = true;

  try {
    // Re-read the list each time round, so a code abandoned while we're mid-loop still gets
    // picked up by this run.
    const attempted = new Set<string>();

    for (;;) {
      const code = getAbandonedCodes(envId).find((abandoned) => !attempted.has(abandoned));
      if (code === undefined) break;

      attempted.add(code);
      const result = await api.deleteSchedule(envId, code);

      // A 404 can only mean there's nothing left to delete.
      if (result.ok || result.code === 404) {
        setAbandonedCodes(
          envId,
          getAbandonedCodes(envId).filter((abandoned) => abandoned !== code),
        );
      }
    }
  } finally {
    deletingAbandoned = false;
  }
};

if (typeof window !== "undefined") {
  window.addEventListener("online", () => {
    if (pendingPush) flushPendingPush?.();
    if (currentEnvId.value !== undefined) void deleteAbandonedCodes(currentEnvId.value);
  });
}

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
  heldSnapshot = undefined;
  heldAdds = new Set();
  pushInFlight = false;
  pendingPush = false;
};

// Switch this device over to another device's sync code, from a sync link. The router calls this
// synchronously while redirecting, so the code is in place before the schedule view mounts and
// runs its initial pull, which then adopts the joined schedule.
export const adoptSyncCode = (envId: string, code: string) => {
  const previous = localStorage.getItem(storageKey(envId));
  if (previous === code) return;

  // An unsent local change belongs to the code we're leaving. Pushing it under the new code would
  // overwrite the very schedule we're joining.
  localStorage.removeItem(pendingPushKey(envId));
  localStorage.setItem(storageKey(envId), code);

  // Rejoining a code we'd abandoned earlier must not delete it out from under us.
  setAbandonedCodes(
    envId,
    getAbandonedCodes(envId).filter((abandoned) => abandoned !== code),
  );
  if (previous !== null) abandonCode(envId, previous);

  if (currentEnvId.value === envId) {
    resetState();
    syncCode.value = code;
  }

  void deleteAbandonedCodes(envId);
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
    void deleteAbandonedCodes(envId.value);
  }

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

    // While a shared schedule is on screen, `starredEvents` holds someone else's stars. Leave the
    // push pending; it's retried on the next navigation outside share mode.
    if (isSharedSchedule.value) {
      setPendingPush(true);
      return;
    }

    const schedule = [...starredEvents.value].sort();
    const serialized = schedule.join(",");

    pushInFlight = true;
    const result = await api.putSchedule(envId.value, code, schedule);
    pushInFlight = false;

    // The device switched codes while this was in flight; the outcome describes a code we no
    // longer use, and whatever replaced it has its own push queued.
    if (syncCode.value !== code) return;

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

  // Give this device a fresh sync code. Purely local: the code only reaches the server with the
  // next push, which the caller queues.
  const mintCode = () => {
    const code = generateSyncCode();
    syncCode.value = code;
    localStorage.setItem(storageKey(envId.value), code);
    lastServerSchedule = undefined;

    // The server has nothing under a brand-new code, so there's no pull to wait for (and nothing
    // held back from it: everything on screen gets pushed).
    ready = true;
    heldSnapshot = undefined;
    heldAdds = new Set();
  };

  // Push on change. Registered without `immediate`, so it never fires for the initial load of
  // local stars — only for genuine changes.
  watch(
    starredEvents,
    () => {
      // A watcher from before an environment switch must not push the new environment's stars
      // under the old environment's code.
      if (currentEnvId.value !== envId.value || isSharedSchedule.value) return;

      // The first star on this device: there's no server copy that could be newer, so there's
      // nothing to wait for before pushing.
      if (syncCode.value === undefined) {
        mintCode();
        schedulePush();
        return;
      }

      if (!ready) return;

      // Skip if nothing changed relative to the server (this is also how we avoid echoing a pull
      // that just wrote the server's schedule into `starredEvents`).
      if (serialize(starredEvents.value) === lastServerSchedule) return;

      schedulePush();
    },
    { deep: true },
  );

  // Make sure there's a sync link to show. Every device with stars already has a code, so this
  // almost always does nothing; either way it never waits on the network, so the sync dialog can
  // open immediately.
  const revealSync = () => {
    if (syncCode.value !== undefined) return;

    mintCode();
    schedulePush();
  };

  // Stop syncing this device with the others by moving it onto a fresh code of its own. Local stars
  // are left untouched and get pushed under the new code. The old code's server entry is deleted so
  // this device's schedule isn't counted twice; any devices still syncing under the old code see a
  // 404 on their next pull and re-create it from their own stars.
  const stopSync = () => {
    if (pushTimer !== undefined) {
      clearTimeout(pushTimer);
      pushTimer = undefined;
    }

    const oldCode = syncCode.value;

    mintCode();
    schedulePush();

    if (oldCode !== undefined) {
      abandonCode(envId.value, oldCode);
      void deleteAbandonedCodes(envId.value);
    }
  };

  // What the user starred and unstarred since `heldSnapshot` was taken, plus anything added through
  // `addStars` while held, or `undefined` if there's nothing. Consumes both either way.
  const takeHeldChanges = (): { added: Set<string>; removed: Set<string> } | undefined => {
    const snapshot = heldSnapshot;
    const explicitAdds = heldAdds;
    heldSnapshot = undefined;
    heldAdds = new Set();

    const current = starredEvents.value;

    // Only what's still starred: the user may have unstarred an added event since.
    const added = new Set([...explicitAdds].filter((id) => current.has(id)));
    const removed = new Set<string>();

    if (snapshot !== undefined) {
      for (const id of current) if (!snapshot.has(id)) added.add(id);
      for (const id of snapshot) if (!current.has(id)) removed.add(id);
    }

    return added.size > 0 || removed.size > 0 ? { added, removed } : undefined;
  };

  // Star several events at once on the user's behalf (e.g. merging a shared schedule into their
  // own), returning the ones that weren't already starred. If we're still waiting on the first
  // pull, they're also recorded so that pull layers them on top of the server's schedule.
  const addStars = (eventIds: Iterable<string>): Set<string> => {
    const added = new Set([...eventIds].filter((id) => !starredEvents.value.has(id)));
    if (added.size === 0) return added;

    if (!ready) for (const id of added) heldAdds.add(id);
    // Not `Set.prototype.union`: `starredEvents.value` is a reactive proxy, which it rejects.
    starredEvents.value = new Set([...starredEvents.value, ...added]);

    return added;
  };

  // Pull the latest schedule from the server and adopt it (last-write-wins). Called on app load and
  // whenever the rest of the app data is refreshed.
  const pullSchedule = async () => {
    // Set when we skip the pull because a shared schedule is on screen. See below.
    let stayHeld = false;

    // However this turns out (short of being in share mode), the push watcher has to come out of
    // the hold it starts in. Leaving `ready` false silently drops every star the user makes for
    // the rest of the session — which is exactly what used to happen when this threw on an
    // offline device.
    try {
      // `starredEvents` holds someone else's schedule, so there's nothing to reconcile. If this
      // is the load's first pull, we also mustn't leave the hold: we still don't know what the
      // server has, and once the user returns to their own schedule, pushing their stored stars
      // would overwrite whatever their other devices changed in the meantime. The watcher on
      // `isSharedSchedule` pulls as soon as they're back.
      if (isSharedSchedule.value) {
        stayHeld = true;
        return;
      }

      const code = syncCode.value;

      if (!code) {
        // Stars from before every device pushed its schedule (or from a device that's never been
        // online since) still need to reach the server once. A device with no stars has nothing
        // to send, and stays off the server until it stars something.
        if (starredEvents.value.size > 0) {
          mintCode();
          schedulePush();
        }

        return;
      }

      // Overlapping pulls (every view mounts its own) share the first one's snapshot.
      if (!ready && heldSnapshot === undefined) heldSnapshot = new Set(starredEvents.value);

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

      // The device switched codes, or a local change landed, while we were fetching. Either way
      // this response no longer describes what the device should adopt.
      if (syncCode.value !== code || isDirty()) return;

      if (!result.ok) {
        if (result.code === 404) {
          // The server has nothing under this code: its TTL ran out, the device that shared it
          // stopped syncing, or the link was shared before its first push landed. Keep the code
          // and re-create the entry from this device's stars.
          lastServerSchedule = undefined;
          const heldChanges = takeHeldChanges();
          if (starredEvents.value.size > 0 || heldChanges !== undefined) schedulePush();
        }

        // Anything else — an unreachable network included — tells us nothing about the
        // server's schedule, so we leave this device's stars exactly as they are.
        return;
      }

      const server = new Set(result.value);
      lastServerSchedule = serialize(server);

      // Last-write-wins, except for anything the user changed while we were waiting: those are
      // newer than the server's copy, so they go on top of it and get pushed.
      const heldChanges = takeHeldChanges();
      const merged =
        heldChanges === undefined
          ? server
          : new Set([...server, ...heldChanges.added].filter((id) => !heldChanges.removed.has(id)));

      if (serialize(merged) !== serialize(starredEvents.value)) {
        starredEvents.value = merged;
      }

      if (serialize(merged) !== lastServerSchedule) schedulePush();
    } finally {
      if (!stayHeld) {
        ready = true;

        // Every other way out of here (offline, a pending push, a queued change) leaves the local
        // stars untouched, so changes made while we were waiting are already on screen; they just
        // still need pushing.
        if (takeHeldChanges() !== undefined) schedulePush();
      }
    }
  };

  // Leaving a shared schedule while still held means this load started on it and never pulled.
  // Pull now, before anything can push this device's stored stars over a newer server schedule.
  // Deferred to the next tick so `starredEvents` has been switched back to the user's own stars
  // by then (that happens in another watcher reacting to the same route change), which makes them
  // the baseline the pull's snapshot measures later changes against.
  watch(isSharedSchedule, (shared, wasShared) => {
    if (!wasShared || shared || ready || shareExitPullQueued) return;
    if (currentEnvId.value !== envId.value) return;

    shareExitPullQueued = true;
    void nextTick(() => {
      shareExitPullQueued = false;
      void pullSchedule();
    });
  });

  return { syncLink, syncEmojis, revealSync, stopSync, pullSchedule, addStars };
};

export default useScheduleSync;
