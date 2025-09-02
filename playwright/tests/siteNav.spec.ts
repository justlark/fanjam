import { test, expect } from '@playwright/test';
import { mockApi } from './mock';

test.describe("site header", () => {
  test.beforeEach(async ({ page }) => {
    mockApi(page, "/info", {
      name: "My Con",
      description: "",
      website_url: "",
      links: [],
      files: [],
    });
  });

  test("has con name as heading", async ({ page }) => {
    await page.goto("");
    await expect(page.getByTestId("site-nav-heading")).toHaveText("My Con");
  });
});
