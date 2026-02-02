import { test as base, expect } from "@playwright/test";
import { mockApi, hoursFromNow, mockTime } from "./common";
import { EventDetailsPage, SchedulePage, FilterMenu } from "./fixtures";

type Fixtures = {
  eventPage: EventDetailsPage;
  schedulePage: SchedulePage;
  filterMenu: FilterMenu;
};

export const test = base.extend<Fixtures>({
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  filterMenu: async ({ page }, use) => {
    await use(new FilterMenu(page));
  },
});

test.describe("event details page", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

  test("displays event name", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Amazing Panel Discussion",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.name).toHaveText("Amazing Panel Discussion");
  });

  test("displays event time", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Timed Event",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.time).toBeVisible();
  });

  test("displays location with search link", async ({ page, eventPage, schedulePage, filterMenu }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: "Main Hall A",
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.location).toBeVisible();
    await expect(eventPage.locationLinks).toHaveText("Main Hall A");

    // Click the location link and verify it filters by location
    await eventPage.locationLinks.click();

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Main Hall A");
  });

  test("displays people with search links", async ({ page, eventPage, filterMenu }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Event",
          start_time: hoursFromNow(1).toISOString(),
          people: ["Alice Smith", "Bob Jones", "Carol White"],
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.hosts).toBeVisible();
    await expect(eventPage.personLinks).toHaveCount(3);
    await expect(eventPage.personLinks.nth(0)).toHaveText("Alice Smith");
    await expect(eventPage.personLinks.nth(1)).toHaveText("Bob Jones");
    await expect(eventPage.personLinks.nth(2)).toHaveText("Carol White");

    // Click a person link and verify it filters by person
    await eventPage.personLinks.nth(0).click();

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Alice Smith");
  });

  test("displays event summary when present", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Summarized Event",
          start_time: hoursFromNow(1).toISOString(),
          summary: "A brief overview of what this event is about.",
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.summary).toBeVisible();
    await expect(eventPage.summary).toHaveText("A brief overview of what this event is about.");
  });

  test("displays event description with markdown rendering", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Described Event",
          start_time: hoursFromNow(1).toISOString(),
          description:
            "# Event Details\n\nThis is a **detailed** description.\n\n- Point one\n- Point two",
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.description).toBeVisible();
    await expect(eventPage.description.locator("h1")).toHaveText("Event Details");
    await expect(eventPage.description.locator("strong")).toHaveText("detailed");
    await expect(eventPage.description.locator("li")).toHaveCount(2);
  });

  test("handles events without optional fields", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Minimal Event",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.name).toHaveText("Minimal Event");
    await expect(eventPage.location).not.toBeVisible();
    await expect(eventPage.hosts).not.toBeVisible();
    await expect(eventPage.noDescription).toBeVisible();
    await expect(eventPage.noDescription).toContainText("No description");
  });

  test("shows no description message when no summary or description", async ({
    page,
    eventPage,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Empty Description Event",
          start_time: hoursFromNow(1).toISOString(),
          summary: null,
          description: null,
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.noDescription).toBeVisible();
  });

  test("does not show no description when summary is provided but description is empty", async ({
    page,
    eventPage,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Summary Only Event",
          start_time: hoursFromNow(1).toISOString(),
          summary: "Just a summary.",
          description: null,
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.summary).toHaveText("Just a summary.");
    await expect(eventPage.noDescription).not.toBeVisible();
  });

  test("displays category and tags", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Tagged Event",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
          tags: ["Beginner", "Interactive"],
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.tagbarCategoryLink).toHaveText("Workshop");
    await expect(eventPage.tagbarTagLinks).toHaveCount(2);
    await expect(eventPage.tagbarTagLinks.nth(0)).toHaveText("Beginner");
    await expect(eventPage.tagbarTagLinks.nth(1)).toHaveText("Interactive");
  });

  test("filters by category when clicking category link", async ({
    page,
    eventPage,
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

    await eventPage.goto("1");
    await eventPage.filterByCategory("Workshop");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Workshop Event");
  });

  test("filters by tag when clicking tag link", async ({ page, eventPage, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Event with Tag",
          start_time: hoursFromNow(1).toISOString(),
          category: "Workshop",
          tags: ["Beginner"],
        },
        {
          id: "2",
          name: "Event without Tag",
          start_time: hoursFromNow(2).toISOString(),
          category: "Workshop",
          tags: ["Advanced"],
        },
      ],
    });

    await eventPage.goto("1");
    await eventPage.filterByTag("Beginner");

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Event with Tag");
  });

  test("star button toggles event starring", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Starrable Event",
          start_time: hoursFromNow(1).toISOString(),
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.starButton).toHaveAttribute("aria-pressed", "false");

    await eventPage.toggleStar();

    await expect(eventPage.starButton).toHaveAttribute("aria-pressed", "true");

    await eventPage.toggleStar();

    await expect(eventPage.starButton).toHaveAttribute("aria-pressed", "false");
  });
});
