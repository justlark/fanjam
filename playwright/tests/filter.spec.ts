import { test as base, expect } from "@playwright/test";
import { mockApi, hoursFromNow } from "./common";
import { EventDetailsPage, FilterMenu, ProgramPage, SchedulePage } from "./fixtures";

type Fixtures = {
  filterMenu: FilterMenu;
  schedulePage: SchedulePage;
  programPage: ProgramPage;
  eventPage: EventDetailsPage;
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

  programPage: async ({ page }, use) => {
    await use(new ProgramPage(page));
  },

  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
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

  for (const route of ["schedule", "program"] as const) {
    test.describe(`in the ${route} view`, () => {
      test.beforeEach(async ({ schedulePage, programPage }) => {
        if (route === "schedule") {
          await schedulePage.goto();
        } else if (route === "program") {
          await programPage.goto();
        }
      });

      test("hide past events", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;
        const hiddenNotice =
          route === "schedule" ? schedulePage.hiddenNotice : programPage.hiddenNotice;

        const pastEvent = events.filter({ hasText: "Test Event 1" });
        const futureEvent = events.filter({ hasText: "Test Event 2" });

        await expect(pastEvent).toHaveCount(1);
        await expect(futureEvent).toHaveCount(1);
        await expect(hiddenNotice).toBeHidden();

        await filterMenu.toggleOpen();
        await filterMenu.toggleHidePastEvents();

        await expect(pastEvent).toHaveCount(0);
        await expect(futureEvent).toHaveCount(1);
        await expect(hiddenNotice).toBeVisible();

        await filterMenu.toggleHidePastEvents();

        await expect(pastEvent).toHaveCount(1);
        await expect(futureEvent).toHaveCount(1);
        await expect(hiddenNotice).toBeHidden();
      });

      test("only show starred events", async ({
        filterMenu,
        schedulePage,
        programPage,
        eventPage,
      }) => {
        if (route === "schedule") {
          await schedulePage.openEventDetailsPage("Test Event 2");
        } else if (route === "program") {
          await programPage.toggleEventExpanded("Test Event 2");
          await programPage.openEventDetailsPage("Test Event 2");
        }

        await eventPage.toggleStar();
        await eventPage.navigateBack();

        await filterMenu.toggleOpen();
        await filterMenu.toggleHideNotStarredEvents();
        await filterMenu.toggleOpen();

        if (route === "schedule") {
          await expect(schedulePage.events).toHaveAccessibleName("Starred: Test Event 2");
        } else if (route === "program") {
          await expect(programPage.eventNames).toHaveText(["Starred:", "Test Event 2"].join(""));
        }
      });

      test("filter by category", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;

        await filterMenu.toggleOpen();
        await filterMenu.toggleCategory("Category 1");

        await expect(events).toHaveCount(1);
        await expect(events).toHaveText("Test Event 1");

        await filterMenu.toggleCategory("Category 2");

        await expect(events).toHaveCount(2);
      });

      test("filter by tag", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;

        await filterMenu.toggleOpen();

        await filterMenu.toggleCategory("Category 1");
        await filterMenu.toggleTag("Tag 1");

        await expect(events).toHaveCount(1);
        await expect(events).toHaveText("Test Event 1");

        await filterMenu.toggleTag("Tag 1");
        await filterMenu.toggleTag("Tag 3");

        await expect(events).toHaveCount(0);
      });

      test("search by event name", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;

        await filterMenu.search("Event 1");

        await expect(events).toHaveText("Test Event 1");

        await filterMenu.search("Event 9999");

        await expect(events).toHaveCount(0);
      });

      test("search by event location", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;

        await filterMenu.search("Apple");

        await expect(events).toHaveText("Test Event 1");

        await filterMenu.search("Banana");

        await expect(events).toHaveCount(0);
      });

      test("search by person", async ({ filterMenu, schedulePage, programPage }) => {
        const events = route === "schedule" ? schedulePage.events : programPage.eventNames;

        await filterMenu.search("Ash");

        await expect(events).toHaveText("Test Event 2");

        await filterMenu.search("Kit");

        await expect(events).toHaveCount(0);
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
  }
});
