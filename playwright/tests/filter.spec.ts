import { test, expect, type Locator } from '@playwright/test';
import { mockApi, hoursFromNow } from './common';

test.describe("filtering events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Test Event 1",
          category: "Category 1",
          tags: ["Tag 1", "Tag 2"],
          location: "Apple Room",
          people: ["Alex", "Rajat"],
          start_time: hoursFromNow(-2).toISOString(),
          end_time: hoursFromNow(-1).toISOString(),
        },
        {
          id: "2",
          name: "Test Event 2",
          category: "Category 2",
          tags: ["Tag 2", "Tag 3"],
          location: "Orange Room",
          people: ["Shilpa", "Ash"],
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ]
    });
  });

  test.describe("in the schedule view", () => {
    let events: Locator;

    test.beforeEach(async ({ page }) => {
      await page.goto("schedule");
      events = page.getByTestId("schedule-event-link").filter({ visible: true });
    });

    test("hide past events", async ({ page }) => {
      const pastEvent = events.filter({ hasText: "Test Event 1" });
      const futureEvent = events.filter({ hasText: "Test Event 2" });
      const hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
      const hidePastEventsButton = page.getByTestId("hide-past-events-button");

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeHidden();

      await page.getByTestId("filter-menu-button").click();
      await hidePastEventsButton.click();

      await expect(pastEvent).toHaveCount(0);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeVisible();

      await hidePastEventsButton.click();

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeHidden();
    });

    test("filter by category", async ({ page }) => {
      const categoryFilterList = page.getByTestId("category-filter-list");

      await page.getByTestId("filter-menu-button").click();
      await categoryFilterList.getByRole("button", { name: "Category 1" }).click();

      await expect(events).toHaveCount(1);
      await expect(events).toHaveText("Test Event 1");

      await categoryFilterList.getByRole("button", { name: "Category 2" }).click();

      await expect(events).toHaveCount(2);
    })

    test("filter by tag", async ({ page }) => {
      const categoryFilterList = page.getByTestId("category-filter-list");
      const tagFilterList = page.getByTestId("tag-filter-list");

      await page.getByTestId("filter-menu-button").click();

      await categoryFilterList.getByRole("button", { name: "Category 1" }).click();
      await tagFilterList.getByRole("button", { name: "Tag 1" }).click();

      await expect(events).toHaveCount(1);
      await expect(events).toHaveText("Test Event 1");

      await tagFilterList.getByRole("button", { name: "Tag 1" }).click();
      await tagFilterList.getByRole("button", { name: "Tag 3" }).click();

      await expect(events).toHaveCount(0);
    })
    test("search by event name", async ({ page }) => {
      await page.getByTestId("filter-search-input").fill("Event 1");

      await expect(events).toHaveText("Test Event 1");

      await page.getByTestId("filter-search-input").fill("Event 9999");

      await expect(events).toHaveCount(0);
    });

    test("search by event location", async ({ page }) => {
      await page.getByTestId("filter-search-input").fill("Apple");

      await expect(events).toHaveText("Test Event 1");

      await page.getByTestId("filter-search-input").fill("Banana");

      await expect(events).toHaveCount(0);
    });

    test("search by person", async ({ page }) => {
      await page.getByTestId("filter-search-input").fill("Ash");

      await expect(events).toHaveText("Test Event 2");

      await page.getByTestId("filter-search-input").fill("Kit");

      await expect(events).toHaveCount(0);
    });
  });
});
