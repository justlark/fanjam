import { test as base, expect } from "@playwright/test";
import { mockApi, mockScheduleSync, mockTime, hoursFromNow } from "./common";
import {
  EventDetailsPage,
  MyScheduleBanner,
  SchedulePage,
  ScheduleSyncDialog,
  SiteNav,
  StarredEvents,
  SyncedSchedule,
} from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
  myScheduleBanner: MyScheduleBanner;
  syncDialog: ScheduleSyncDialog;
  siteNav: SiteNav;
  starredEvents: StarredEvents;
  syncedSchedule: SyncedSchedule;
};

const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  myScheduleBanner: async ({ page }, use) => {
    await use(new MyScheduleBanner(page));
  },
  syncDialog: async ({ page }, use) => {
    await use(new ScheduleSyncDialog(page));
  },
  siteNav: async ({ page }, use) => {
    await use(new SiteNav(page));
  },
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },
  syncedSchedule: async ({ page }, use) => {
    await use(new SyncedSchedule(page));
  },
});

const EVENTS = [
  { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
  { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
  { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
];

const SYNC_CODE_RE = /\/sync\/\?s=[a-z0-9]{12}$/;

test.describe("sync button gating", () => {
  test("hidden when the feature flag is disabled", async ({
    page,
    schedulePage,
    myScheduleBanner,
    starredEvents,
  }) => {
    await mockTime(page);
    await mockApi(page, {
      events: EVENTS,
      config: { use_schedule_sharing: false, use_calendar_export: false, use_schedule_sync: false },
    });

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await expect(myScheduleBanner.optionsButton).not.toBeVisible();
  });

  test("shown when the feature flag is enabled", async ({
    page,
    schedulePage,
    myScheduleBanner,
    starredEvents,
  }) => {
    await mockTime(page);
    await mockApi(page, {
      events: EVENTS,
      config: { use_schedule_sharing: false, use_calendar_export: false, use_schedule_sync: true },
    });
    await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.openOptions();
    await expect(myScheduleBanner.syncButton).toBeVisible();
  });
});

test.describe("enabling sync", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, config: { use_schedule_sync: true } });
  });

  test("uploads the current schedule and shows a sync link", async ({
    page,
    schedulePage,
    myScheduleBanner,
    syncDialog,
    syncedSchedule,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();

    await expect(syncDialog.url).toBeVisible();
    const url = await syncDialog.url.inputValue();
    expect(url).toMatch(SYNC_CODE_RE);

    // The schedule was uploaded under the code embedded in the link.
    const code = new URL(url).searchParams.get("s")!;
    await expect.poll(() => sync.get(code)).toEqual(["1"]);
    expect(await syncedSchedule.get()).toBe(code);
  });

  test("reopening while syncing reuses the same link without re-uploading", async ({
    page,
    schedulePage,
    myScheduleBanner,
    syncDialog,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();
    await expect(syncDialog.url).toBeVisible();
    const firstUrl = await syncDialog.url.inputValue();
    await expect.poll(() => sync.putCount).toBe(1);

    await syncDialog.close();
    await expect(syncDialog.url).not.toBeVisible();

    await myScheduleBanner.sync();
    await expect(syncDialog.url).toBeVisible();
    const secondUrl = await syncDialog.url.inputValue();

    expect(secondUrl).toBe(firstUrl);
    // No second upload — the code was reused.
    expect(sync.putCount).toBe(1);
  });
});

test.describe("joining a sync link", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, config: { use_schedule_sync: true } });
  });

  test("adopts the server schedule and stores the sync code", async ({
    page,
    schedulePage,
    syncedSchedule,
  }) => {
    await mockScheduleSync(page, { abcdef123456: ["2", "3"] });

    await page.goto("sync/?s=abcdef123456");

    await expect(page.getByText("Syncing your schedule")).toBeVisible();
    await expect(page).toHaveURL(/\/schedule\/all$/);
    expect(await syncedSchedule.get()).toBe("abcdef123456");

    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 3" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).not.toHaveAccessibleName(
      /^Starred:/,
    );
  });
});

// Starring while offline used to be lost twice over: the push watcher never armed, so the change
// was never even queued, and the next successful pull then overwrote the local stars with the
// server's older copy. The local change has to survive the outage and win the reconciliation.
test.describe("syncing changes while offline", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, config: { use_schedule_sync: true } });
  });

  test("a star made offline survives a reload and is pushed on reconnect", async ({
    page,
    schedulePage,
    eventPage,
  }) => {
    const sync = await mockScheduleSync(page, { abcdef123456: ["2"] });

    // Join an existing sync code. This writes the code synchronously in the router redirect, so
    // syncing is live before the schedule view mounts.
    await page.goto("sync/?s=abcdef123456");
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );

    // The device loses signal, and the user stars another event.
    sync.offline = true;

    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.starButton.click();
    await eventPage.navigateBack();

    // Flush the debounced push, which fails against the dead network.
    await page.clock.fastForward(1000);
    expect(sync.get("abcdef123456")).toEqual(["2"]);

    // The star is still on screen — the failed push must not roll the UI back.
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).toHaveAccessibleName(
      /^Starred:/,
    );

    // The user closes the app and comes back later with signal. This is a full reload, so
    // nothing in memory survives — the record that we owe the server a push has to have been
    // persisted, or the pull below hands back the older schedule and eats the star.
    sync.offline = false;
    await schedulePage.goto();

    // The pull must not hand back the server's older ["2"]; the undelivered push wins and lands.
    await expect.poll(() => sync.get("abcdef123456")).toEqual(["1", "2"]);
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
  });

  test("an unreachable server does not clear the local schedule", async ({
    page,
    schedulePage,
  }) => {
    const sync = await mockScheduleSync(page, { abcdef123456: ["2", "3"] });

    await page.goto("sync/?s=abcdef123456");
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );

    // A pull that can't reach the server tells us nothing about the schedule, so it must leave
    // the device's stars alone rather than treating "no answer" as "empty".
    sync.offline = true;
    await schedulePage.goto();
    await page.clock.fastForward(1000);

    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 3" })).toHaveAccessibleName(
      /^Starred:/,
    );
  });
});

test.describe("syncing changes", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, config: { use_schedule_sync: true } });
  });

  test("pushes starring changes to the server", async ({
    page,
    schedulePage,
    eventPage,
    myScheduleBanner,
    syncDialog,
    syncedSchedule,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();
    await expect(syncDialog.url).toBeVisible();
    await expect.poll(() => sync.putCount).toBe(1);
    const code = (await syncedSchedule.get())!;
    await syncDialog.close();

    // Change the schedule (unstar the only event) and confirm the change is pushed.
    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.starButton.click();

    // The push is debounced; advance the (frozen) clock to flush it.
    await page.clock.fastForward(1000);

    await expect.poll(() => sync.get(code)).toEqual([]);
  });

  test("stops pushing after the user stops syncing", async ({
    page,
    schedulePage,
    eventPage,
    myScheduleBanner,
    syncDialog,
    syncedSchedule,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();
    await expect.poll(() => sync.putCount).toBe(1);

    await syncDialog.stopSyncing();
    await expect(syncDialog.url).not.toBeVisible();
    expect(await syncedSchedule.get()).toBeNull();

    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.starButton.click();
    await page.clock.fastForward(1000);

    // No further uploads once syncing is off.
    expect(sync.putCount).toBe(1);
  });

  test("a refresh does not clobber a pending local change", async ({
    page,
    schedulePage,
    eventPage,
    myScheduleBanner,
    syncDialog,
    siteNav,
    syncedSchedule,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();
    await expect.poll(() => sync.putCount).toBe(1);
    const code = (await syncedSchedule.get())!;
    await syncDialog.close();

    // Unstar the event, then refresh *before* the debounced push lands. The server still has the
    // old schedule (["1"]), but the pending local change must win — the stale pull must not
    // re-add the event.
    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.starButton.click();
    await eventPage.navigateBack();

    await siteNav.refresh();

    // The event stays unstarred — My Schedule is empty, not re-populated from the stale server.
    await expect(schedulePage.events).toHaveCount(0);

    // Once the push flushes, the server reflects the local change.
    await page.clock.fastForward(1000);
    await expect.poll(() => sync.get(code)).toEqual([]);
  });
});

test.describe("sync isolation while viewing a shared schedule", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, config: { use_schedule_sync: true } });
  });

  test("does not push to or pull from the server in share mode", async ({
    page,
    schedulePage,
    myScheduleBanner,
    syncDialog,
    siteNav,
    starredEvents,
  }) => {
    const sync = await mockScheduleSync(page);

    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    await myScheduleBanner.sync();
    await expect.poll(() => sync.putCount).toBe(1);
    await syncDialog.close();

    // Enter share mode (viewing events 2 and 3 — "2,3" base64url-encoded).
    await page.goto("schedule?share=Miwz");

    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );

    await siteNav.refresh();
    await page.clock.fastForward(1000);

    // The shared view is untouched and nothing was pushed/pulled while it was active.
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).not.toHaveAccessibleName(
      /^Starred:/,
    );
    expect(sync.putCount).toBe(1);
  });
});
