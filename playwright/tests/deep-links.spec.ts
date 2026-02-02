import { test as base, expect } from "@playwright/test";
import { mockApi, mockTime, hoursFromNow } from "./common";
import {
  EventDetailsPage,
  SchedulePage,
  AnnouncementsPage,
  FilterMenu,
} from "./fixtures";

type Fixtures = {
  eventPage: EventDetailsPage;
  schedulePage: SchedulePage;
  announcementsPage: AnnouncementsPage;
  filterMenu: FilterMenu;
};

export const test = base.extend<Fixtures>({
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  announcementsPage: async ({ page }, use) => {
    await use(new AnnouncementsPage(page));
  },
  filterMenu: async ({ page }, use) => {
    await use(new FilterMenu(page));
  },
});

test.describe("deep linking", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

  test("direct link to event opens event details page", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "event-123",
          name: "Featured Panel",
          start_time: hoursFromNow(1).toISOString(),
          description: "This is the featured panel description.",
        },
      ],
    });

    await page.goto("events/event-123");

    await expect(eventPage.name).toHaveText("Featured Panel");
    await expect(page).toHaveURL(/\/events\/event-123$/);
  });

  test("direct link to announcement opens announcement details", async ({
    page,
    announcementsPage,
  }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "announce-1",
          title: "Important Update",
          body: "Please read this important update.",
          created_at: hoursFromNow(-1).toISOString(),
          updated_at: hoursFromNow(-1).toISOString(),
        },
      ],
    });

    await page.goto("announcements/announce-1");

    await expect(page).toHaveURL(/\/announcements\/announce-1$/);
    await expect(announcementsPage.createdTime).toBeVisible();
  });

  test("direct link with category filter applies the filter", async ({
    page,
    schedulePage,
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

    // Categories use 'c' query param
    await page.goto("schedule?c=Workshop");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Workshop Event");
  });

  test("direct link with tag filter applies the filter", async ({
    page,
    schedulePage,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Beginner Event",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
          tags: ["Beginner"],
        },
        {
          id: "2",
          name: "Advanced Event",
          start_time: hoursFromNow(2).toISOString(),
          category: "Workshop",
          tags: ["Advanced"],
        },
      ],
    });

    // Categories use 'c' and tags use 't' query params
    await page.goto("schedule?c=Workshop&t=Beginner");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Beginner Event");
  });

  test("direct link with search filter applies the filter", async ({
    page,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Art Workshop",
          start_time: hoursFromNow(1).toISOString(),
        },
        {
          id: "2",
          name: "Music Panel",
          start_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    // Search uses 'q' query param
    await page.goto("schedule?q=Art");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Art Workshop");
    await expect(filterMenu.searchInput).toHaveValue("Art");
  });

  test("direct link with starred filter shows only starred events", async ({
    page,
    schedulePage,
    eventPage,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Starred Event",
          start_time: hoursFromNow(1).toISOString(),
        },
        {
          id: "2",
          name: "Unstarred Event",
          start_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    // First, star an event
    await eventPage.goto("1");
    await eventPage.toggleStar();

    // Then navigate with the starred filter
    await page.goto("schedule?star=true");
    await schedulePage.toAllEventsView();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Starred Event");
  });

  test("direct link to specific day opens that day", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Day 1 Event",
          start_time: hoursFromNow(-24).toISOString(),
          end_time: hoursFromNow(-23).toISOString(),
        },
        {
          id: "2",
          name: "Day 2 Event",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          id: "3",
          name: "Day 3 Event",
          start_time: hoursFromNow(24).toISOString(),
          end_time: hoursFromNow(25).toISOString(),
        },
      ],
    });

    // Navigate directly to day 1
    await page.goto("schedule/1");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Day 1 Event");
    await expect(page).toHaveURL(/\/schedule\/1$/);
  });

  test("direct link to all events view shows all events", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Event A",
          start_time: hoursFromNow(-24).toISOString(),
        },
        {
          id: "2",
          name: "Event B",
          start_time: hoursFromNow(0).toISOString(),
        },
        {
          id: "3",
          name: "Event C",
          start_time: hoursFromNow(24).toISOString(),
        },
      ],
    });

    await page.goto("schedule/all");

    await expect(schedulePage.events).toHaveCount(3);
    await expect(page).toHaveURL(/\/schedule\/all$/);
  });

  test("direct link to info page opens info page", async ({ page }) => {
    await mockApi(page, {
      info: {
        name: "Test Convention",
        description: "Welcome to our convention!",
      },
    });

    await page.goto("info");

    await expect(page).toHaveURL(/\/info$/);
    await expect(page.getByTestId("info-page-name")).toHaveText("Test Convention");
  });

  test("direct link to custom page opens the page", async ({ page }) => {
    await mockApi(page, {
      pages: [
        {
          id: "rules",
          title: "Convention Rules",
          body: "Please follow these rules.",
          files: [],
        },
      ],
    });

    await page.goto("pages/rules");

    await expect(page).toHaveURL(/\/pages\/rules$/);
    await expect(page.getByTestId("page-viewer-title")).toHaveText("Convention Rules");
  });

  test("direct link preserves filters when navigating back from event", async ({
    page,
    schedulePage,
    eventPage,
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

    // Navigate with a filter (categories use 'c' param)
    await page.goto("schedule?c=Workshop");
    await expect(schedulePage.events).toHaveCount(1);

    // Open event details
    await schedulePage.openEventDetailsPage("Workshop Event");

    // Navigate back
    await eventPage.navigateBack();

    // Filter should be preserved
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Workshop Event");
  });

  test("multiple filters can be combined in URL", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Target Event",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
          tags: ["Beginner"],
          location: "Room A",
        },
        {
          id: "2",
          name: "Other Event",
          start_time: hoursFromNow(2).toISOString(),
          category: "Workshop",
          tags: ["Advanced"],
          location: "Room B",
        },
      ],
    });

    // Categories use 'c' and tags use 't' query params
    await page.goto("schedule?c=Workshop&t=Beginner");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Target Event");
  });
});
