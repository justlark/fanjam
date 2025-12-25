import { test as base, expect } from "@playwright/test";
import { mockApi, isMobile, hoursFromNow, mockTime, shiftTimeByHours } from "./common";
import { EventDetailsPage, FilterMenu, EventSummaryDrawer, SchedulePage } from "./fixtures";

type Fixtures = {
  filterMenu: FilterMenu;
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
  summaryDrawer: EventSummaryDrawer;
};

export const test = base.extend<Fixtures>({
  filterMenu: async ({ page }, use) => {
    const filterMenu = new FilterMenu(page);
    await use(filterMenu);
    await filterMenu.clear();
  },

  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },

  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },

  summaryDrawer: async ({ page }, use) => {
    await use(new EventSummaryDrawer(page));
  },
});

test.describe("filtering events", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);

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

  test.beforeEach(async ({ page, schedulePage }) => {
    await mockTime(page);
    await schedulePage.goto();
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

  test("hide past events mid-event", async ({ page, filterMenu, schedulePage }) => {
    await shiftTimeByHours(page, -2);

    const currentEvent = schedulePage.events.filter({ hasText: "Test Event 1" });

    await filterMenu.toggleOpen();
    await filterMenu.toggleHidePastEvents();

    await expect(currentEvent).toHaveCount(1);
    await expect(schedulePage.hiddenNotice).not.toBeVisible();
  });

  test("hide past events before first events of the day", async ({
    page,
    filterMenu,
    schedulePage,
  }) => {
    await shiftTimeByHours(page, -3);

    await filterMenu.toggleOpen();
    await filterMenu.toggleHidePastEvents();

    await expect(schedulePage.hiddenNotice).not.toBeVisible();
  });

  test("only show starred events", async ({ filterMenu, schedulePage, eventPage }) => {
    await schedulePage.openEventDetailsPage("Test Event 2");
    await eventPage.toggleStar();
    await eventPage.navigateBack();

    await filterMenu.toggleOpen();
    await filterMenu.toggleHideNotStarredEvents();
    await filterMenu.toggleOpen();

    await expect(schedulePage.events).toHaveAccessibleName("Starred: Test Event 2");
  });

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

  test("clearing categories and tags from the filter description", async ({
    filterMenu,
    schedulePage,
  }) => {
    await filterMenu.toggleOpen();

    await filterMenu.toggleCategory("Category 1");
    await filterMenu.toggleTag("Tag 1");

    await filterMenu.toggleOpen();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Test Event 1");

    await filterMenu.clearCategoryOrTag("Category 1");
    await filterMenu.clearCategoryOrTag("Tag 1");

    await expect(schedulePage.events).toHaveCount(2);
    await expect(filterMenu.description).not.toBeVisible();
  });

  test("follow link to search by location", async ({ filterMenu, schedulePage, eventPage }) => {
    await schedulePage.openEventDetailsPage("Test Event 2");

    await eventPage.locationLinks.filter({ hasText: "Orange Room" }).click();

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Orange Room");
  });

  test("follow link to search by person", async ({ filterMenu, schedulePage, eventPage }) => {
    await schedulePage.openEventDetailsPage("Test Event 2");

    await eventPage.personLinks.filter({ hasText: "Ash" }).click();

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Ash");
  });

  test("filter by category or tag from event page", async ({ schedulePage, eventPage }) => {
    await schedulePage.goto();

    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.filterByCategory("Category 1");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Test Event 1");

    await schedulePage.openEventDetailsPage("Test Event 1");
    await eventPage.filterByTag("Tag 2");

    await expect(schedulePage.events).toHaveCount(2);
    await expect(schedulePage.events.nth(0)).toHaveText("Test Event 1");
    await expect(schedulePage.events.nth(1)).toHaveText("Test Event 2");
  });

  test("filter by category or tag from event summary drawer", async ({
    schedulePage,
    summaryDrawer,
  }) => {
    if (!isMobile()) {
      test.skip();
    }

    await schedulePage.goto();

    await schedulePage.openEventSummaryDrawer("Test Event 1");
    await summaryDrawer.filterByCategory("Category 1");

    await summaryDrawer.close();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Test Event 1");

    await schedulePage.openEventSummaryDrawer("Test Event 1");
    await summaryDrawer.filterByTag("Tag 2");

    await summaryDrawer.close();

    await expect(schedulePage.events).toHaveCount(2);
    await expect(schedulePage.events.nth(0)).toHaveText("Test Event 1");
    await expect(schedulePage.events.nth(1)).toHaveText("Test Event 2");
  });
});
