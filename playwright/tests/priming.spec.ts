import { test as base, expect } from "@playwright/test";
import { mockApi, mockTime, shiftTimeByHours, countRequestsTo, hoursFromNow } from "./common";
import { SchedulePage } from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
};

const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

// Priming is deliberately deferred to an idle callback, which only guarantees it runs within
// its 2s timeout — then it still has to fetch. Under a fully parallel suite that comfortably
// outruns Playwright's 5s default, so these wait longer for something that is meant to happen
// in the background, and wait out the same window before concluding it didn't.
const POLL_TIMEOUT_MS = 15000;
const SETTLE_MS = 4000;

const EVENTS = [{ id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() }];
const PEOPLE = [{ id: "p1", name: "Test Person", bio: "A bio" }];
const PAGES = [{ id: "g1", title: "Test Page", body: "Some body" }];

const stored = (page: import("@playwright/test").Page, key: string) =>
  page.evaluate((k) => localStorage.getItem(k), `store:${key}`);

// The fetch policies hold each page load to the endpoints that page renders. Priming is what
// keeps that from leaving the offline cache full of holes: whatever the landing route didn't
// ask for still has to end up on the device, or it simply isn't there when the signal goes.
test.describe("priming the offline cache", () => {
  test("fetches data the landing route does not itself use", async ({ page, schedulePage }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, people: PEOPLE, pages: PAGES });

    // The schedule route fetches events, info, announcements and config — but not people or
    // pages, which are exactly what the event and info screens need offline.
    await schedulePage.goto();

    await expect
      .poll(() => stored(page, "people"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Person");
    await expect
      .poll(() => stored(page, "pages"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Page");
  });

  test("fetches the schedule when the user arrives on an announcement", async ({ page }) => {
    await mockTime(page);
    const data = await mockApi(page, {
      events: EVENTS,
      people: PEOPLE,
      announcements: [{ title: "Test Announcement" }],
    });

    // This is the push-notification path: the only page this user ever opened is a deep link to
    // one announcement, which on its own leaves them with no schedule at all.
    await page.goto(`announcements/${data.announcements[0].id}`);

    await expect
      .poll(() => stored(page, "events"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Event 1");
    await expect
      .poll(() => stored(page, "people"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Person");
  });

  test("does not refetch data that is still fresh", async ({ page, schedulePage }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, people: PEOPLE, pages: PAGES });

    await schedulePage.goto();
    await expect
      .poll(() => stored(page, "people"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Person");

    const requests = countRequestsTo(page, "/people");

    // Announcements is another route that doesn't use people, so priming reconsiders it — and
    // should decide the cached copy is current and leave it alone.
    await page.goto("announcements");
    await page.waitForTimeout(SETTLE_MS);

    expect(requests.count).toBe(0);
  });

  test("refetches once the cached copy has aged out", async ({ page, schedulePage }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, people: PEOPLE, pages: PAGES });

    await schedulePage.goto();
    await expect
      .poll(() => stored(page, "people"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Person");

    const requests = countRequestsTo(page, "/people");

    // Past the five minute default, so the copy cached on the schedule is no longer trusted.
    await shiftTimeByHours(page, 1);
    await page.goto("announcements");

    await expect.poll(() => requests.count, { timeout: POLL_TIMEOUT_MS }).toBeGreaterThan(0);
  });

  test("honours the local cache max age from the environment config", async ({
    page,
    schedulePage,
  }) => {
    await mockTime(page);
    await mockApi(page, {
      events: EVENTS,
      people: PEOPLE,
      pages: PAGES,
      // A day, so the hour that ages data out under the default is well inside it.
      config: { local_cache_max_age: 24 * 60 * 60 * 1000 },
    });

    await schedulePage.goto();
    await expect
      .poll(() => stored(page, "people"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Person");

    const requests = countRequestsTo(page, "/people");

    await shiftTimeByHours(page, 1);
    await page.goto("announcements");
    await page.waitForTimeout(SETTLE_MS);

    expect(requests.count).toBe(0);
  });

  // What an app update does: `invalidate()` marks every entry stale in place. The old behaviour
  // deleted them outright, which left the device with a near-empty offline cache right after an
  // update — the reload only refetches what the current route needs.
  test("data marked stale by an app update is refreshed, not discarded", async ({
    page,
    schedulePage,
  }) => {
    await mockTime(page);
    await mockApi(page, { events: EVENTS, people: PEOPLE, pages: PAGES });

    await schedulePage.goto();
    await expect
      .poll(() => stored(page, "pages"), { timeout: POLL_TIMEOUT_MS })
      .toContain("Test Page");

    await page.evaluate(() => {
      for (const key of Object.keys(localStorage).filter((k) => k.startsWith("store:"))) {
        const entry = JSON.parse(localStorage.getItem(key) as string) as { fetched_at: number };
        entry.fetched_at = 0;
        localStorage.setItem(key, JSON.stringify(entry));
      }
    });

    const requests = countRequestsTo(page, "/pages");

    await schedulePage.goto();

    // Still readable the whole way through: there is never a window where a user who goes
    // offline right after updating has nothing to show.
    expect(await stored(page, "pages")).toContain("Test Page");

    await expect.poll(() => requests.count, { timeout: POLL_TIMEOUT_MS }).toBeGreaterThan(0);
  });
});
