import { test, expect } from '@playwright/test';
import { mockApi } from './mock';

test.describe("site header", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, {
      info: {
        name: "My Con",
      },
    });
  });

  test("has con name as heading", async ({ page }) => {
    await page.goto("");
    await expect(page.getByTestId("site-nav-heading")).toHaveText("My Con");
  });
});
