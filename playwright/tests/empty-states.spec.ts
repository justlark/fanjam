import { test as base, expect } from "@playwright/test";
import { mockApi, mockTime, hoursFromNow } from "./common";
import {
  SchedulePage,
  AnnouncementsPage,
  FilterMenu,
  EventDetailsPage,
} from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  announcementsPage: AnnouncementsPage;
  filterMenu: FilterMenu;
  eventPage: EventDetailsPage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  announcementsPage: async ({ page }, use) => {
    await use(new AnnouncementsPage(page));
  },
  filterMenu: async ({ page }, use) => {
    await use(new FilterMenu(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
});

test.describe("empty states", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

  test("shows empty message when no events exist", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [],
    });

    await schedulePage.goto();

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.noEventsNotice).toHaveText("No events");
    await expect(schedulePage.events).toHaveCount(0);
  });

  test("shows empty message when no events for specific day", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Tomorrow Event",
          start_time: hoursFromNow(24).toISOString(),
          end_time: hoursFromNow(25).toISOString(),
        },
      ],
    });

    // Go to today (which has no events since the event is tomorrow)
    await schedulePage.goto();

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.noEventsNotice).toHaveText("No events");
  });

  test("shows empty message when all events are filtered out by category", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Workshop Event",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
        },
        {
          id: "2",
          name: "Panel Event",
          start_time: hoursFromNow(2).toISOString(),
          category: "Panel",
        },
      ],
    });

    await schedulePage.goto();

    // Filter by a non-existent category
    await filterMenu.toggleOpen();
    await filterMenu.toggleCategory("Workshop");
    await filterMenu.toggleOpen();

    // Unfilter Workshop
    await filterMenu.toggleOpen();
    await filterMenu.toggleCategory("Workshop");

    // Now search for something that doesn't exist
    await filterMenu.search("NonexistentEvent");

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.events).toHaveCount(0);
  });

  test("shows empty message when search finds nothing", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Discussion",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    await filterMenu.search("xyz123nonexistent");

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.events).toHaveCount(0);
  });

  test("shows empty message when no starred events exist", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Unstarred Event",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    // Enable "My Schedule" filter (show only starred events)
    await filterMenu.toggleOpen();
    await filterMenu.toggleHideNotStarredEvents();

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.events).toHaveCount(0);
  });

  test("shows empty message when no announcements exist", async ({ page, announcementsPage }) => {
    await mockApi(page, {
      announcements: [],
    });

    await announcementsPage.goto();

    await expect(announcementsPage.emptyNotice).toBeVisible();
    await expect(announcementsPage.emptyNotice).toHaveText("No announcements yet");
    await expect(announcementsPage.link).toHaveCount(0);
  });

  test("shows empty message when all announcements have no title", async ({
    page,
    announcementsPage,
  }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "",
          updated_at: hoursFromNow(0).toISOString(),
        },
        {
          id: "2",
          title: "   ",
          updated_at: hoursFromNow(-1).toISOString(),
        },
      ],
    });

    await announcementsPage.goto();

    await expect(announcementsPage.emptyNotice).toBeVisible();
    await expect(announcementsPage.link).toHaveCount(0);
  });

  test("clears empty state when search is cleared", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Real Event",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    // Search for something that doesn't exist
    await filterMenu.search("nonexistent");
    await expect(schedulePage.noEventsNotice).toBeVisible();

    // Clear the search
    await filterMenu.clear();
    await expect(schedulePage.noEventsNotice).not.toBeVisible();
    await expect(schedulePage.events).toHaveCount(1);
  });

  test("clears empty state when filter is removed", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Event 1",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
        },
      ],
    });

    await schedulePage.goto();

    // Filter by starred events (none are starred)
    await filterMenu.toggleOpen();
    await filterMenu.toggleHideNotStarredEvents();

    await expect(schedulePage.noEventsNotice).toBeVisible();

    // Remove the filter
    await filterMenu.toggleHideNotStarredEvents();

    await expect(schedulePage.noEventsNotice).not.toBeVisible();
    await expect(schedulePage.events).toHaveCount(1);
  });

  test("all events view shows empty when all events are filtered", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Day 1 Event",
          start_time: hoursFromNow(-24).toISOString(),
        },
        {
          id: "2",
          name: "Day 2 Event",
          start_time: hoursFromNow(0).toISOString(),
        },
      ],
    });

    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.events).toHaveCount(2);

    // Search for non-existent
    await filterMenu.search("nonexistent");

    await expect(schedulePage.noEventsNotice).toBeVisible();
    await expect(schedulePage.events).toHaveCount(0);
  });

  test("hiding past events shows hidden notice but not empty message if future events exist", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Past Event",
          start_time: hoursFromNow(-2).toISOString(),
          end_time: hoursFromNow(-1).toISOString(),
        },
        {
          id: "2",
          name: "Future Event",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    await filterMenu.toggleOpen();
    await filterMenu.toggleHidePastEvents();

    await expect(schedulePage.hiddenNotice).toBeVisible();
    await expect(schedulePage.noEventsNotice).not.toBeVisible();
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Future Event");
  });
});
