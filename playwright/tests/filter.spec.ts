import { test, expect, type Locator, type Page } from '@playwright/test';
import { mockApi, envId, hoursFromNow } from './common';

const setStarredEvents = async (page: Page, eventIds: Array<string>) =>
  page.evaluate((envId) => localStorage.setItem(`starred:${envId}`, JSON.stringify(eventIds)), envId);

test.describe("filtering events", () => {
  let scheduleEvents: Locator;
  let filterMenuButton: Locator;
  let categoryFilterList: Locator;
  let tagFilterList: Locator;
  let searchInput: Locator;

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

    scheduleEvents = page.getByTestId("schedule-event-link").filter({ visible: true });
    filterMenuButton = page.getByTestId("filter-menu-button");
    categoryFilterList = page.getByTestId("category-filter-list");
    tagFilterList = page.getByTestId("tag-filter-list");
    searchInput = page.getByTestId("filter-search-input");
  });

  const toggleFilterMenu = () => filterMenuButton.click();
  const toggleCategory = (category: string) => categoryFilterList.getByRole("button", { name: category }).click();
  const toggleTag = (tag: string) => tagFilterList.getByRole("button", { name: tag }).click();
  const search = (text: string) => searchInput.fill(text);

  test.describe("in the schedule view", () => {
    test.beforeEach(async ({ page }) => {
      await page.goto("schedule");
    });

    test("hide past events", async ({ page }) => {
      const pastEvent = scheduleEvents.filter({ hasText: "Test Event 1" });
      const futureEvent = scheduleEvents.filter({ hasText: "Test Event 2" });
      const hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
      const hidePastEventsButton = page.getByTestId("hide-past-events-button");

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeHidden();

      await toggleFilterMenu();
      await hidePastEventsButton.click();

      await expect(pastEvent).toHaveCount(0);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeVisible();

      await hidePastEventsButton.click();

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(hiddenNotice).toBeHidden();
    });

    test("only show starred events", async ({ page }) => {
    });

    test("filter by category", async () => {
      await toggleFilterMenu();
      await toggleCategory("Category 1");

      await expect(scheduleEvents).toHaveCount(1);
      await expect(scheduleEvents).toHaveText("Test Event 1");

      await toggleCategory("Category 2");

      await expect(scheduleEvents).toHaveCount(2);
    })

    test("filter by tag", async () => {
      await toggleFilterMenu();

      await toggleCategory("Category 1");
      await toggleTag("Tag 1");

      await expect(scheduleEvents).toHaveCount(1);
      await expect(scheduleEvents).toHaveText("Test Event 1");

      await toggleTag("Tag 1");
      await toggleTag("Tag 3");

      await expect(scheduleEvents).toHaveCount(0);
    })
    test("search by event name", async () => {
      await search("Event 1");

      await expect(scheduleEvents).toHaveText("Test Event 1");

      await search("Event 9999");

      await expect(scheduleEvents).toHaveCount(0);
    });

    test("search by event location", async () => {
      await search("Apple");

      await expect(scheduleEvents).toHaveText("Test Event 1");

      await search("Banana");

      await expect(scheduleEvents).toHaveCount(0);
    });

    test("search by person", async () => {
      await search("Ash");

      await expect(scheduleEvents).toHaveText("Test Event 2");

      await search("Kit");

      await expect(scheduleEvents).toHaveCount(0);
    });

    test("filter description", async ({ page }) => {
      const filterDescription = page.getByTestId("filter-description");
      const hideNotStarredEventsButton = page.getByTestId("hide-not-starred-events-button");

      await toggleFilterMenu();

      await hideNotStarredEventsButton.click();
      await toggleCategory("Category 1");
      await toggleCategory("Category 2");
      await toggleTag("Tag 1");
      await toggleTag("Tag 2");
      await search("foo");

      await toggleFilterMenu();

      await expect(filterDescription).toHaveText(["Only showing:", "Starred", "and", "(", "Category 1", "or", "Category 2", ")", "and", "(", "Tag 1", "or", "Tag 2", ")", "and", "\"foo\""].join(""));
    });
  });
});
