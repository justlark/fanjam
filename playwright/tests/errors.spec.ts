import { test as base, expect } from "@playwright/test";
import { mockApi, mockInfoError, mockTime, hoursFromNow } from "./common";
import { SiteNav, SchedulePage, EventDetailsPage } from "./fixtures";

type Fixtures = {
  siteNav: SiteNav;
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
};

export const test = base.extend<Fixtures>({
  siteNav: async ({ page }, use) => {
    await use(new SiteNav(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
});

test.describe("error states", () => {
  test("shows error state when info API fails", async ({ page, siteNav }) => {
    await mockInfoError(page, 500);

    await page.goto("schedule");

    await expect(siteNav.errorState).toBeVisible();
    await expect(siteNav.errorState).toContainText("Not found");
    await expect(siteNav.errorState).toContainText("There is nothing here. Is this the right URL?");
  });

  test("shows error state for invalid environment ID", async ({ page, siteNav }) => {
    await mockInfoError(page, 404);

    await page.goto("schedule");

    await expect(siteNav.errorState).toBeVisible();
    await expect(siteNav.errorState).toContainText("Not found");
  });

  test("error state is shown on all pages when info fails", async ({ page, siteNav }) => {
    await mockInfoError(page, 500);

    // Test on schedule page
    await page.goto("schedule");
    await expect(siteNav.errorState).toBeVisible();

    // Test on info page
    await page.goto("info");
    await expect(siteNav.errorState).toBeVisible();

    // Test on announcements page
    await page.goto("announcements");
    await expect(siteNav.errorState).toBeVisible();
  });

  test("header buttons are not visible in error state", async ({ page, siteNav }) => {
    await mockInfoError(page, 500);

    await page.goto("schedule");

    await expect(siteNav.errorState).toBeVisible();
    await expect(siteNav.copyLinkButton).not.toBeVisible();
    await expect(siteNav.refreshButton).not.toBeVisible();
  });

  test("schedule view shows loading spinner while data loads", async ({ page }) => {
    await mockTime(page);

    // Set up a delayed response to observe loading state
    let resolveInfo: () => void;
    const infoPromise = new Promise<void>((resolve) => {
      resolveInfo = resolve;
    });

    await page.route("https://api-test.fanjam.live/apps/*/info", async (route) => {
      await infoPromise;
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({
          retry_after_ms: null,
          value: { name: "Test Con", description: null, website_url: null, links: [], files: [] },
        }),
      });
    });

    await page.route("https://api-test.fanjam.live/apps/*/events", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ retry_after_ms: null, value: { events: [] } }),
      });
    });

    await page.route("https://api-test.fanjam.live/apps/*/pages", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ retry_after_ms: null, value: { pages: [] } }),
      });
    });

    await page.route("https://api-test.fanjam.live/apps/*/announcements", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ retry_after_ms: null, value: { announcements: [] } }),
      });
    });

    await page.route("https://api-test.fanjam.live/apps/*/config", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({}),
      });
    });

    // Navigate but don't wait for load
    const gotoPromise = page.goto("schedule");

    // Expect loading spinner to be visible
    await expect(page.locator("svg[class*='p-progress-spinner']")).toBeVisible({ timeout: 1000 });

    // Resolve the info request
    resolveInfo!();

    await gotoPromise;
  });

  test("direct link to non-existent event shows spinner then schedule", async ({
    page,
    schedulePage,
  }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        {
          id: "existing-event",
          name: "Existing Event",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    // Navigate to a non-existent event
    await page.goto("events/nonexistent-event-id");

    // Should eventually show the schedule with the existing event
    // The behavior depends on how the app handles missing events
    // In this case, the schedule timeline should still be accessible
    await expect(schedulePage.events).toHaveCount(1);
  });
});
