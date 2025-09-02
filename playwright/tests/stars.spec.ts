import { test, expect } from '@playwright/test';
import { mockApiEvents, stub } from './mock';

test.describe("starring events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApiEvents(page, [stub]);
  });

  test.describe("on desktop", () => {
    test.beforeEach(async () => {
      test.skip(test.info().project.name !== "desktop");
    });

    test("star button on event page toggles link in schedule view", async ({ page }) => {
      await page.goto("schedule");

      const eventLink = page.getByTestId("schedule-event-link").first();
      await eventLink.click();

      const starButton = page.getByTestId("event-details-star-toggle-button").filter({ visible: true });
      await starButton.click();

      await expect(starButton).toHaveAttribute("aria-pressed", "true");
      await expect(eventLink).toHaveAccessibleName(/^Starred:/);
    });
  });
});
