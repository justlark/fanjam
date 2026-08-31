import { test as base, expect, type Page } from "@playwright/test";
import { envId, mockAlias, mockApi, mockApiOffline, mockTime } from "./common";
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

  test("explains itself when there is nothing cached to fall back on", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    // A first-ever visit with no connection. There is no cached copy of anything, so the app has
    // nothing to render — and a blank page reads as broken. Say which of the two it is.
    await mockApiOffline(page);

    await schedulePage.goto();

    await expect(siteNav.offlineState).toBeVisible();
    await expect(siteNav.offlineState).toContainText("You're offline");

    // Specifically not the "is this the right URL?" state: nothing here says the con is missing.
    await expect(siteNav.errorState).not.toBeVisible();
  });

  test("stays out of the way when there is cached data to show", async ({
    page,
    schedulePage,
    siteNav,
  }) => {
    // The whole point of the cache. Someone who loaded the con once gets the con, not an
    // apology, however long they have been offline since.
    await schedulePage.goto();
    await expect(siteNav.heading).toHaveText("Test Convention");

    await mockApiOffline(page);
    await schedulePage.goto();

    await expect(siteNav.heading).toHaveText("Test Convention");
    await expect(siteNav.offlineState).not.toBeVisible();
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

// The URL of a different con on the same origin. `baseURL` already points at one con's mount
// point, so the sibling is one level up.
const conUrl = (baseURL: string | undefined, env: string, path: string) =>
  new URL(`../${env}/${path}`, baseURL).toString();

// Which con the cache entry claims to belong to. This is the field that decides whether the app
// will serve an entry, so it is the one worth asserting on.
const storedInstance = (page: Page, key: string) =>
  page.evaluate((k) => {
    const raw = localStorage.getItem(`store:${k}`);
    return raw ? (JSON.parse(raw) as { instance: string }).instance : undefined;
  }, key);

// Only one con fits in local storage, so opening a second one does eventually cost you the
// first. What it must not cost you is the first one *and* the second: an attempt that never
// loads has nothing to put in the cache's place.
test.describe("the cached con", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, { info: { name: "Test Convention" } });
  });

  test("survives opening a different con while offline", async ({
    page,
    baseURL,
    schedulePage,
    siteNav,
  }) => {
    await schedulePage.goto();
    await expect(siteNav.heading).toHaveText("Test Convention");
    await expect.poll(() => storedInstance(page, "info")).toBe(envId);

    await mockApiOffline(page);

    await page.goto(conUrl(baseURL, "other-con", "schedule"));

    // Nothing is cached for this con and nothing can be fetched, so there is genuinely nothing
    // to show for it.
    await expect(siteNav.offlineState).toBeVisible();
    expect(await storedInstance(page, "info")).toBe(envId);

    // The con the user actually has is still theirs, still offline.
    await schedulePage.goto();
    await expect(siteNav.heading).toHaveText("Test Convention");
    await expect(siteNav.offlineState).not.toBeVisible();
  });

  test("is not relabelled when a different con turns out to be an alias", async ({
    page,
    baseURL,
    schedulePage,
    siteNav,
  }) => {
    await schedulePage.goto();
    await expect(siteNav.heading).toHaveText("Test Convention");
    await expect.poll(() => storedInstance(page, "info")).toBe(envId);

    // `con-b` has been renamed to `con-b-new`, and this device can't reach the new one. That is
    // the window in which a cache entry relabelled with the wrong con would be served as that
    // con's data.
    await page.route("https://api-test.fanjam.live/apps/*/info", async (route) => {
      const env = new URL(route.request().url()).pathname.split("/")[2];

      if (env === "con-b") {
        await route.fulfill({
          status: 404,
          contentType: "application/json",
          body: JSON.stringify({ error: "Not found" }),
        });
        return;
      }

      await route.abort("internetdisconnected");
    });
    await mockAlias(page, "con-b-new");

    await page.goto(conUrl(baseURL, "con-b", "schedule"));

    await expect(page).toHaveURL(/\/app\/con-b-new\//);
    await expect(siteNav.offlineState).toBeVisible();
    await expect(siteNav.heading).not.toBeVisible();
    expect(await storedInstance(page, "info")).toBe(envId);
  });
});
