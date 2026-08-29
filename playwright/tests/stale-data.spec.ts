import { test as base, expect } from "@playwright/test";
import {
  mockApi,
  mockApiOffline,
  mockWrappedApiResponseSequence,
  countRequestsTo,
  mockTime,
} from "./common";
import { InfoPage } from "./fixtures";

type Fixtures = {
  infoPage: InfoPage;
};

export const test = base.extend<Fixtures>({
  infoPage: async ({ page }, use) => {
    await use(new InfoPage(page));
  },
});

// The retry delays below are upper bounds, not exact values: the client jitters
// each delay across the lower half of its interval (retry 1 fires somewhere in
// 750-1500ms, retry 2 in 1500-3000ms, and so on). Fast-forwarding by the full
// undithered delay therefore always fires the pending timer.
test.describe("stale data retry behavior", () => {
  test("updates the page when fresh data arrives after a stale response", async ({
    infoPage,
    page,
  }) => {
    await mockTime(page);

    // Mock other endpoints first
    await mockApi(page, {
      events: [],
      pages: [],
      announcements: [],
    });

    // Mock the info endpoint to return stale data first, then fresh data
    // This must come AFTER mockApi so it doesn't get overwritten
    await mockWrappedApiResponseSequence(page, "/info", [
      {
        freshness: "stale",
        body: {
          name: "Old Convention Name",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        freshness: "fresh",
        body: {
          name: "New Convention Name",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
    ]);

    await infoPage.goto();

    // Assert stale data is displayed immediately
    await expect(infoPage.name).toHaveText("Old Convention Name");

    // Fast-forward through the first retry delay (at most 1500ms), waiting for
    // the response to ensure the DOM update completes before asserting.
    const nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Assert fresh data is now displayed
    await expect(infoPage.name).toHaveText("New Convention Name");
  });

  test("does not retry when data is fresh", async ({ page, infoPage }) => {
    await mockTime(page);

    const requestCounter = countRequestsTo(page, "/info");

    // Mock all endpoints with fresh data
    await mockApi(page, {
      info: { name: "Test Convention" },
    });

    await infoPage.goto();

    // Assert the page loaded
    await expect(infoPage.name).toHaveText("Test Convention");

    // Fast-forward well past any retry delay
    await page.clock.fastForward(5000);

    // The initial load triggers reload() twice (once from cache check, once as background refresh)
    // With fresh data, there should be no additional retries
    expect(requestCounter.count).toBeLessThanOrEqual(2);
  });

  test("does not retry when the server is in backoff", async ({ page, infoPage }) => {
    await mockTime(page);

    const requestCounter = countRequestsTo(page, "/info");

    // Mock other endpoints first
    await mockApi(page, {
      events: [],
      pages: [],
      announcements: [],
    });

    // The server is serving out of its persistent cache and has stopped
    // refreshing upstream, so there is nothing fresher to come back for.
    // This must come AFTER mockApi so it doesn't get overwritten
    await mockWrappedApiResponseSequence(page, "/info", [
      {
        freshness: "backoff",
        body: {
          name: "Cached While Upstream Is Down",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
    ]);

    await infoPage.goto();

    // The cached data still renders — degraded, not broken.
    await expect(infoPage.name).toHaveText("Cached While Upstream Is Down");

    const countAfterLoad = requestCounter.count;

    // Fast-forward past every rung of the retry ladder.
    await page.clock.fastForward(60000);

    // Assert the client never came back. This is the whole point: during an
    // upstream outage the server sees only organic traffic.
    expect(requestCounter.count).toBe(countAfterLoad);
  });

  test("stops retrying after receiving fresh data", async ({ page, infoPage }) => {
    await mockTime(page);

    const requestCounter = countRequestsTo(page, "/info");

    // Mock other endpoints first
    await mockApi(page, {
      events: [],
      pages: [],
      announcements: [],
    });

    // Mock the info endpoint: stale twice, then fresh
    // This must come AFTER mockApi so it doesn't get overwritten
    await mockWrappedApiResponseSequence(page, "/info", [
      {
        freshness: "stale",
        body: {
          name: "Stale Data 1",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        freshness: "stale",
        body: {
          name: "Stale Data 2",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        freshness: "fresh",
        body: {
          name: "Fresh Data",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
    ]);

    await infoPage.goto();

    // Fast-forward through first retry (at most 1500ms), waiting for the response
    // to ensure scheduleRetry() has created the next timer before we advance again.
    let nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Fast-forward through second retry (at most 3000ms)
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(3000);
    await nextResponse;

    // Assert fresh data is displayed
    await expect(infoPage.name).toHaveText("Fresh Data");

    const countAfterFreshData = requestCounter.count;

    // Fast-forward well past the next retry delay
    await page.clock.fastForward(10000);

    // Assert no additional requests were made after receiving fresh data
    expect(requestCounter.count).toBe(countAfterFreshData);
  });

  test("caps retries at maximum count", async ({ page, infoPage }) => {
    await mockTime(page);

    const requestCounter = countRequestsTo(page, "/info");

    // Mock other endpoints first
    await mockApi(page, {
      events: [],
      pages: [],
      announcements: [],
    });

    // Mock the info endpoint to always return stale data
    // This must come AFTER mockApi so it doesn't get overwritten
    await mockWrappedApiResponseSequence(page, "/info", [
      {
        freshness: "stale",
        body: {
          name: "Forever Stale",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
    ]);

    await infoPage.goto();

    // Fast-forward through all 3 retry delays, waiting for each response
    // to ensure scheduleRetry() has created the next timer before advancing.
    let nextResponse: Promise<unknown>;

    // Retry 1: at most 1500ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Retry 2: at most 3000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(3000);
    await nextResponse;

    // Retry 3: at most 6000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(6000);
    await nextResponse;

    // Fast-forward well past another retry delay
    await page.clock.fastForward(60000);

    // Should have made: initial load (varies based on mount behavior) + 3
    // retries. We can't assert exact count without knowing mount behavior, but
    // we can verify it stopped.
    expect(requestCounter.count).toBeGreaterThanOrEqual(3);

    // Reasonable upper bound.
    expect(requestCounter.count).toBeLessThanOrEqual(6);
  });
});

test.describe("unreachable network", () => {
  test("keeps showing cached data when the network goes away", async ({ page, infoPage }) => {
    await mockTime(page);
    await mockApi(page, { info: { name: "Cached Convention" } });

    // Populate the cache while we still have a network.
    await infoPage.goto();
    await expect(infoPage.name).toHaveText("Cached Convention");

    // Now take the network away and come back to the app.
    await mockApiOffline(page);
    await infoPage.goto();

    await expect(infoPage.name).toHaveText("Cached Convention");
  });

  test("does not clear cached data when the network goes away", async ({ page, infoPage }) => {
    await mockTime(page);
    await mockApi(page, { info: { name: "Cached Convention" } });

    await infoPage.goto();
    await expect(infoPage.name).toHaveText("Cached Convention");

    await mockApiOffline(page);
    await infoPage.goto();
    await expect(infoPage.name).toHaveText("Cached Convention");

    // The cache entry itself must survive, so the next cold start still has something to show.
    const stored = await page.evaluate(() => localStorage.getItem("store:info"));
    expect(stored).not.toBeNull();
    expect(stored).toContain("Cached Convention");
  });

  test("does not retry when the network is unreachable", async ({ page, infoPage }) => {
    await mockTime(page);
    await mockApi(page, { info: { name: "Cached Convention" } });

    await infoPage.goto();
    await expect(infoPage.name).toHaveText("Cached Convention");

    await mockApiOffline(page);
    await infoPage.goto();
    await expect(infoPage.name).toHaveText("Cached Convention");

    // Start counting only once we're offline and settled.
    const requestCounter = countRequestsTo(page, "/info");
    const countAfterLoad = requestCounter.count;

    // Fast-forward past every rung of the retry ladder. The ladder exists for the sub-second
    // window while the edge cache repopulates, not for a radio that's out of range.
    await page.clock.fastForward(60000);

    expect(requestCounter.count).toBe(countAfterLoad);
  });
});
