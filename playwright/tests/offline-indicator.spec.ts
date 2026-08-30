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
    await expect(siteNav.heading).toHaveText("Test Convention");
    await expect(siteNav.offlineIndicator).not.toBeVisible();

    await page.context().setOffline(true);
    await expect(siteNav.offlineIndicator).toBeVisible();

    // It sits beside the refresh button rather than replacing it — refreshing is still the right
    // thing to try, and hiding the control would just leave people stuck.
    await expect(siteNav.refreshButton).toBeVisible();

    await page.context().setOffline(false);
    await expect(siteNav.offlineIndicator).not.toBeVisible();
  });

  test("shows on a page that loads while already offline", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    // Report offline from the moment the page boots, rather than genuinely cutting the
    // connection: a real offline load only paints at all once the service worker is serving the
    // shell, and that needs a secure context, which the plain-HTTP dev server isn't. What this
    // needs to pin down is narrower anyway — that the indicator comes from the initial state and
    // not only from a later `offline` event.
    await page.addInitScript(() => {
      Object.defineProperty(navigator, "onLine", { get: () => false, configurable: true });
    });

    await schedulePage.goto();

    await expect(siteNav.offlineIndicator).toBeVisible();
    await expect(siteNav.heading).toHaveText("Test Convention");
  });

  test("refresh says so instead of pretending to fetch while offline", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    await schedulePage.goto();
    await expect(siteNav.heading).toHaveText("Test Convention");

    await page.context().setOffline(true);

    await siteNav.refresh();

    await expect(page.getByText("You're offline")).toBeVisible();
    await expect(page.getByText("Grabbing the latest schedule.")).not.toBeVisible();
  });
});
