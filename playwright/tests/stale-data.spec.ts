import { test as base, expect } from "@playwright/test";
import { mockApi, mockWrappedApiResponseSequence, countRequestsTo, mockTime } from "./common";
import { InfoPage } from "./fixtures";

type Fixtures = {
  infoPage: InfoPage;
};

export const test = base.extend<Fixtures>({
  infoPage: async ({ page }, use) => {
    await use(new InfoPage(page));
  },
});

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
        stale: true,
        body: {
          name: "Old Convention Name",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        stale: false,
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

    // Fast-forward through the first retry delay (1500ms), waiting for the
    // response to ensure the DOM update completes before asserting.
    const nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Assert fresh data is now displayed
    await expect(infoPage.name).toHaveText("New Convention Name");
  });

  test("does not retry when data is fresh", async ({ page, infoPage }) => {
    await mockTime(page);

    const requestCounter = countRequestsTo(page, "/info");

    // Mock all endpoints with fresh data (stale: false)
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
        stale: true,
        body: {
          name: "Stale Data 1",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        stale: true,
        body: {
          name: "Stale Data 2",
          description: null,
          website_url: null,
          links: [],
          files: [],
        },
      },
      {
        stale: false,
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

    // Fast-forward through first retry (1500ms), waiting for the response to
    // ensure scheduleRetry() has created the next timer before we advance again.
    let nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Fast-forward through second retry (3000ms)
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
        stale: true,
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

    // Fast-forward through all 5 retry delays, waiting for each response
    // to ensure scheduleRetry() has created the next timer before advancing.
    let nextResponse: Promise<unknown>;

    // Retry 1: 1500ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(1500);
    await nextResponse;

    // Retry 2: 3000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(3000);
    await nextResponse;

    // Retry 3: 6000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(6000);
    await nextResponse;

    // Retry 4: 12000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(12000);
    await nextResponse;

    // Retry 5: 24000ms
    nextResponse = page.waitForResponse(/info/);
    await page.clock.fastForward(24000);
    await nextResponse;

    // Fast-forward well past another retry delay
    await page.clock.fastForward(60000);

    // Should have made: initial load (varies based on mount behavior) + 5
    // retries We can't assert exact count without knowing mount behavior, but
    // we can verify it stopped.
    expect(requestCounter.count).toBeGreaterThanOrEqual(5);

    // Reasonable upper bound.
    expect(requestCounter.count).toBeLessThanOrEqual(8);
  });
});
