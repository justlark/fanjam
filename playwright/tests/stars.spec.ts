import { test, expect } from '@playwright/test';
import { mockApi, envId, isMobile } from './common';

test.describe("starring events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, { events: [{ id: "123" }] });
  });

  test("star button in event page", async ({ page }) => {
    await page.goto("schedule");

    await page.getByTestId("schedule-event-link").filter({ visible: true }).first().click();

    if (isMobile()) {
      await page.getByTestId("event-summary-drawer-expand-button").click();
    }

    const starButton = page.getByTestId("event-details-star-button").filter({ visible: true });
    await expect(starButton).toHaveAttribute("aria-pressed", "false");
    await starButton.click();
    await expect(starButton).toHaveAttribute("aria-pressed", "true");

    const actualStarredEvents = await page.evaluate((envId) => localStorage.getItem(`starred:${envId}`), envId);
    expect(actualStarredEvents).toEqual(JSON.stringify(["123"]));

    if (isMobile()) {
      await page.getByTestId("event-details-back-button").click();
    }

    await expect(page.getByTestId("schedule-event-link").filter({ visible: true }).first()).toHaveAccessibleName(/^Starred:/);
  })

  test("star button in program view", async ({ page }) => {
    await page.goto("program");

    await page.getByTestId("program-event-expand-button").first().click();

    const starButton = page.getByTestId("program-event-star-button").first();
    await expect(starButton).toHaveAttribute("aria-pressed", "false");
    await starButton.click();
    await expect(starButton).toHaveAttribute("aria-pressed", "true");

    const actualStarredEvents = await page.evaluate((envId) => localStorage.getItem(`starred:${envId}`), envId);
    expect(actualStarredEvents).toEqual(JSON.stringify(["123"]));

    await expect(page.getByTestId("program-event-name").first()).toHaveText(/^Starred:/);
  })
});
