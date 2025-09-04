import { test as base, expect } from "@playwright/test";
import { mockApi, hoursFromNow } from "./common";
import { FilterMenu, SchedulePage } from "./fixtures";

type Fixtures = {
  filterMenu: FilterMenu;
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  filterMenu: async ({ page }, use) => {
    await use(new FilterMenu(page));
  },

  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

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
      ],
    });
  });

  test.describe("in the schedule view", () => {
    test.beforeEach(async ({ page }) => {
      await page.goto("schedule");
    });

    test("hide past events", async ({ filterMenu, schedulePage }) => {
      const pastEvent = schedulePage.events.filter({ hasText: "Test Event 1" });
      const futureEvent = schedulePage.events.filter({ hasText: "Test Event 2" });

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(schedulePage.hiddenNotice).toBeHidden();

      await filterMenu.toggleOpen();
      await filterMenu.toggleHidePastEvents();

      await expect(pastEvent).toHaveCount(0);
      await expect(futureEvent).toHaveCount(1);
      await expect(schedulePage.hiddenNotice).toBeVisible();

      await filterMenu.toggleHidePastEvents();

      await expect(pastEvent).toHaveCount(1);
      await expect(futureEvent).toHaveCount(1);
      await expect(schedulePage.hiddenNotice).toBeHidden();
    });

    test("only show starred events", async () => {});

    test("filter by category", async ({ filterMenu, schedulePage }) => {
      await filterMenu.toggleOpen();
      await filterMenu.toggleCategory("Category 1");

      await expect(schedulePage.events).toHaveCount(1);
      await expect(schedulePage.events).toHaveText("Test Event 1");

      await filterMenu.toggleCategory("Category 2");

      await expect(schedulePage.events).toHaveCount(2);
    });

    test("filter by tag", async ({ filterMenu, schedulePage }) => {
      await filterMenu.toggleOpen();

      await filterMenu.toggleCategory("Category 1");
      await filterMenu.toggleTag("Tag 1");

      await expect(schedulePage.events).toHaveCount(1);
      await expect(schedulePage.events).toHaveText("Test Event 1");

      await filterMenu.toggleTag("Tag 1");
      await filterMenu.toggleTag("Tag 3");

      await expect(schedulePage.events).toHaveCount(0);
    });
    test("search by event name", async ({ filterMenu, schedulePage }) => {
      await filterMenu.search("Event 1");

      await expect(schedulePage.events).toHaveText("Test Event 1");

      await filterMenu.search("Event 9999");

      await expect(schedulePage.events).toHaveCount(0);
    });

    test("search by event location", async ({ filterMenu, schedulePage }) => {
      await filterMenu.search("Apple");

      await expect(schedulePage.events).toHaveText("Test Event 1");

      await filterMenu.search("Banana");

      await expect(schedulePage.events).toHaveCount(0);
    });

    test("search by person", async ({ filterMenu, schedulePage }) => {
      await filterMenu.search("Ash");

      await expect(schedulePage.events).toHaveText("Test Event 2");

      await filterMenu.search("Kit");

      await expect(schedulePage.events).toHaveCount(0);
    });

    test("filter description", async ({ filterMenu }) => {
      await filterMenu.toggleOpen();

      await filterMenu.toggleHideNotStarredEvents();
      await filterMenu.toggleCategory("Category 1");
      await filterMenu.toggleCategory("Category 2");
      await filterMenu.toggleTag("Tag 1");
      await filterMenu.toggleTag("Tag 2");
      await filterMenu.search("foo");

      await filterMenu.toggleOpen();

      await expect(filterMenu.description).toHaveText(
        [
          "Only showing:",
          "Starred",
          "and",
          "(",
          "Category 1",
          "or",
          "Category 2",
          ")",
          "and",
          "(",
          "Tag 1",
          "or",
          "Tag 2",
          ")",
          "and",
          '"foo"',
        ].join(""),
      );
    });
  });
});
