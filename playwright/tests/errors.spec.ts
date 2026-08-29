import { test as base, expect } from "@playwright/test";
import { mockApiOffline, mockInfoError } from "./common";
import { SiteNav } from "./fixtures";

type Fixtures = {
  siteNav: SiteNav;
};

export const test = base.extend<Fixtures>({
  siteNav: async ({ page }, use) => {
    await use(new SiteNav(page));
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
    await expect(siteNav.refreshButton).not.toBeVisible();
  });

  test("does not show the not-found state when the network is unreachable", async ({
    page,
    siteNav,
  }) => {
    await mockApiOffline(page);

    await page.goto("schedule");

    await expect(siteNav.errorState).not.toBeVisible();
  });
});
