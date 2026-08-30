import { test as base, expect } from "@playwright/test";
import { mockApi, mockTime } from "./common";
import { SchedulePage, SiteNav } from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  siteNav: SiteNav;
};

const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => await use(new SchedulePage(page)),
  siteNav: async ({ page }, use) => await use(new SiteNav(page)),
});

// Stale data that looks live is worse than stale data that says so: without this, someone
// refreshing a schedule that silently isn't updating has no way to tell why.
test.describe("offline indicator", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { info: { name: "Test Convention" } });
  });

  test("is hidden while the app has a connection", async ({ schedulePage, siteNav }) => {
    await schedulePage.goto();

    await expect(siteNav.refreshButton).toBeVisible();
    await expect(siteNav.offlineIndicator).not.toBeVisible();
  });

  test("appears when the connection drops and clears when it returns", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    await schedulePage.goto();
    await expect(siteNav.offlineIndicator).not.toBeVisible();

    await page.context().setOffline(true);
    await expect(siteNav.offlineIndicator).toBeVisible();

    // It sits beside the refresh button rather than replacing it — refreshing is still the right
    // thing to try, and hiding the control would just leave people stuck.
    await expect(siteNav.refreshButton).toBeVisible();

    await page.context().setOffline(false);
    await expect(siteNav.offlineIndicator).not.toBeVisible();
  });

  test("shows on a page that was loaded while already offline", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    await schedulePage.goto();
    await page.context().setOffline(true);

    // A reload with no connection still paints from the cache, so the indicator has to come from
    // the initial state and not only from the `offline` event.
    await page.reload();

    await expect(siteNav.offlineIndicator).toBeVisible();
    await expect(siteNav.heading).toHaveText("Test Convention");
  });
});
