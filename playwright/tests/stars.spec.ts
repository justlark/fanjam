import { test, expect } from '@playwright/test';
import { mockApi, stub } from './mock';

test.describe("starring events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, { events: [stub] });
  });

  test.describe("star button on event page toggles link in schedule view", () => {
    test("on desktop", async ({ page }) => {
      test.skip(test.info().project.name !== "desktop");

      await page.goto("schedule");

      const eventLink = page.getByTestId("schedule-event-link").filter({ visible: true }).first();
      await eventLink.click();

      const starButton = page.getByTestId("event-details-star-toggle-button").filter({ visible: true });
      await starButton.click();

      await expect(starButton).toHaveAttribute("aria-pressed", "true");
      await expect(eventLink).toHaveAccessibleName(/^Starred:/);
    });

    test("on mobile", async ({ page }) => {
      test.skip(test.info().project.name !== "mobile");

      await page.goto("schedule");

      await page.getByTestId("schedule-event-link").filter({ visible: true }).first().click();
      await page.getByTestId("event-summary-drawer-expand-button").click();

      const starButton = page.getByTestId("event-details-star-toggle-button").filter({ visible: true });
      await starButton.click();

      await expect(starButton).toHaveAttribute("aria-pressed", "true");

      await page.getByTestId("event-details-back-button").click();

      await expect(page.getByTestId("schedule-event-link").filter({ visible: true }).first()).toHaveAccessibleName(/^Starred:/);
    });
  });
});
