import { test as base, expect } from "@playwright/test";
import { isMobile, mockApi } from "./common";
import { SchedulePage, MainMenu, EventDetailsPage, FilterMenu } from "./fixtures";

type Fixtures = {
  mainMenu: MainMenu;
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
  filterMenu: FilterMenu;
};

export const test = base.extend<Fixtures>({
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  filterMenu: async ({ page }, use) => {
    await use(new FilterMenu(page));
  },
});

test.describe("main menu", () => {
  test.beforeEach(async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Unstarred Event",
        },
        {
          id: "2",
          name: "Starred Event",
        },
      ],
    });

    await eventPage.goto("2");
    await eventPage.toggleStar();
  });

  test("navigating to the schedule does not only show starred events", async ({
    mainMenu,
    schedulePage,
  }) => {
    if (isMobile()) {
      await mainMenu.open();
    }

    await mainMenu.navigateToSchedule();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.events).toHaveCount(2);
    await expect(schedulePage.events).toHaveText(["Unstarred Event", "Starred Event"]);
  });

  test("navigating to my schedule only shows starred events", async ({
    mainMenu,
    schedulePage,
  }) => {
    if (isMobile()) {
      await mainMenu.open();
    }

    await mainMenu.navigateToMySchedule();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText(["Starred Event"]);
  });

  test("the main nav menu shows you're filtering by starred events", async ({
    mainMenu,
    schedulePage,
    filterMenu,
  }) => {
    await schedulePage.goto();

    await filterMenu.toggleOpen();
    await filterMenu.toggleHideNotStarredEvents();
    await filterMenu.toggleOpen();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.myScheduleLink).toHaveAttribute("aria-current", "page");
    await expect(mainMenu.scheduleLink).not.toHaveAttribute("aria-current");
  });

  test("the main nav menu shows you're not filtering by starred events", async ({
    schedulePage,
    mainMenu,
  }) => {
    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.scheduleLink).toHaveAttribute("aria-current", "page");
    await expect(mainMenu.myScheduleLink).not.toHaveAttribute("aria-current");
  });
});
