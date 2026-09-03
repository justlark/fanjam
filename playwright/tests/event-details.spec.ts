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

  test("displays location", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { name: "Main Hall A" },
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.location).toBeVisible();
    await expect(eventPage.location).toContainText("Main Hall A");
  });

  test("displays people with search links", async ({ page, eventPage, filterMenu }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Event",
          start_time: hoursFromNow(1).toISOString(),
          people: [
            { id: "10", name: "Alice Smith" },
            { id: "20", name: "Bob Jones" },
            { id: "30", name: "Carol White" },
          ],
        },
      ],
    });

    await eventPage.goto("1");

    await expect(eventPage.hosts).toBeVisible();
    await expect(eventPage.personLinks).toHaveCount(3);
    await expect(eventPage.personLinks.nth(0)).toHaveText("Alice Smith");
    await expect(eventPage.personLinks.nth(1)).toHaveText("Bob Jones");
    await expect(eventPage.personLinks.nth(2)).toHaveText("Carol White");

    // Click a person link to open their bio, then "Find in Schedule" to filter by them.
    await eventPage.personLinks.nth(0).click();
    await eventPage.bioFindButton.click();

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Alice Smith");
  });

  test("shows a person's bio in a dialog", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Event",
          start_time: hoursFromNow(1).toISOString(),
          people: [
            { id: "10", name: "Alice Smith" },
            { id: "20", name: "Bob Jones" },
          ],
        },
      ],
      people: [
        { id: "10", name: "Alice Smith", bio: "Alice is a longtime panelist." },
        { id: "20", name: "Bob Jones", bio: null },
      ],
    });

    await eventPage.goto("1");

    // Opens the bio for the clicked person, cross-referenced by ID.
    await eventPage.personLinks.filter({ hasText: "Alice Smith" }).click();
    await expect(eventPage.bio).toHaveText("Alice is a longtime panelist.");
  });

  test("shows a message when a person has no bio", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Event",
          start_time: hoursFromNow(1).toISOString(),
          people: [
            { id: "10", name: "Alice Smith" },
            { id: "20", name: "Bob Jones" },
          ],
        },
      ],
      people: [
        { id: "10", name: "Alice Smith", bio: "Alice is a longtime panelist." },
        { id: "20", name: "Bob Jones", bio: null },
      ],
    });

    await eventPage.goto("1");

    await eventPage.personLinks.filter({ hasText: "Bob Jones" }).click();
    await expect(eventPage.bio).not.toBeVisible();
    await expect(eventPage.bioMissingMessage).toHaveText("No information available");
  });

  test("shows a location's description in a dialog", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "1", name: "Main Hall A" },
        },
        {
          id: "2",
          name: "Other Event",
          start_time: hoursFromNow(2).toISOString(),
          location: { id: "2", name: "Side Room B" },
        },
      ],
      locations: [
        { id: "1", name: "Main Hall A", description: "The big room, past the registration desk." },
        { id: "2", name: "Side Room B", description: "Upstairs, at the end of the corridor." },
      ],
    });

    await eventPage.goto("1");

    // Opens the description for this event's location, cross-referenced by ID.
    await eventPage.locationLink.click();
    await expect(eventPage.bio).toHaveText("The big room, past the registration desk.");
  });

  test("shows a message when a location has no description", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "2", name: "Side Room B" },
        },
      ],
      locations: [
        { id: "1", name: "Main Hall A", description: "The big room, past the registration desk." },
        { id: "2", name: "Side Room B", description: null },
      ],
    });

    await eventPage.goto("1");

    await eventPage.locationLink.click();
    await expect(eventPage.bio).not.toBeVisible();
    await expect(eventPage.bioMissingMessage).toHaveText("No information available");
  });

  // An event carries its location inline, so the link is there to click whether
  // or not the locations endpoint knows the place. Nothing upstream guarantees
  // the two agree — a location added between the two responses lands here.
  test("shows a message when the location is missing from the locations list", async ({
    page,
    eventPage,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { name: "Main Hall A" },
        },
      ],
    });

    await eventPage.goto("1");

    await eventPage.locationLink.click();
    await expect(eventPage.bioMissingMessage).toHaveText("No information available");
  });

  test("renders markdown in a location's description", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "1", name: "Main Hall A" },
        },
      ],
      locations: [
        {
          id: "1",
          name: "Main Hall A",
          description: "# Getting There\n\nThe **big** room.\n\n- Step-free access\n- Hearing loop",
        },
      ],
    });

    await eventPage.goto("1");

    await eventPage.locationLink.click();

    await expect(eventPage.bio.locator("h1")).toHaveText("Getting There");
    await expect(eventPage.bio.locator("strong")).toHaveText("big");
    await expect(eventPage.bio.locator("li")).toHaveCount(2);
  });

  test("labels the location dialog with the location's name", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Located Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "1", name: "Main Hall A" },
        },
      ],
      locations: [{ id: "1", name: "Main Hall A", description: "The big room." }],
    });

    await eventPage.goto("1");

    await eventPage.locationLink.click();

    await expect(eventPage.bioDialog).toContainText("Main Hall A");
    // The dialog is shared with people's bios, and which icon it wears is the
    // only thing in its header that says which of the two you are looking at.
    await expect(eventPage.bioDialog.locator("i.bi-geo-alt-fill")).toBeVisible();
  });

  test("filters the schedule by location from the location dialog", async ({
    page,
    eventPage,
    schedulePage,
    filterMenu,
  }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Main Hall Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "1", name: "Main Hall A" },
        },
        {
          id: "2",
          name: "Side Room Event",
          start_time: hoursFromNow(2).toISOString(),
          location: { id: "2", name: "Side Room B" },
        },
      ],
      locations: [
        { id: "1", name: "Main Hall A", description: "The big room." },
        { id: "2", name: "Side Room B", description: "Upstairs." },
      ],
    });

    await eventPage.goto("1");

    await eventPage.locationLink.click();
    await eventPage.bioFindButton.click();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Main Hall Event");

    await filterMenu.toggleOpen();
    await expect(filterMenu.searchInput).toHaveValue("Main Hall A");
  });

  // Both dialogs are the same component driven by two sets of refs, so opening
  // one after the other is where a mixed-up ref would show itself.
  test("keeps a person's bio and a location's description apart", async ({ page, eventPage }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Panel Event",
          start_time: hoursFromNow(1).toISOString(),
          location: { id: "1", name: "Main Hall A" },
          people: [{ id: "10", name: "Alice Smith" }],
        },
      ],
      people: [{ id: "10", name: "Alice Smith", bio: "Alice is a longtime panelist." }],
      locations: [{ id: "1", name: "Main Hall A", description: "The big room." }],
    });

    await eventPage.goto("1");

    await eventPage.personLinks.filter({ hasText: "Alice Smith" }).click();
    await expect(eventPage.bio).toHaveText("Alice is a longtime panelist.");

    await page.keyboard.press("Escape");
    await expect(eventPage.bio).not.toBeVisible();

    await eventPage.locationLink.click();
    await expect(eventPage.bio).toHaveText("The big room.");
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
          category: { name: "Workshop" },
          tags: [{ name: "Beginner" }, { name: "Interactive" }],
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
          category: { name: "Workshop" },
        },
        {
          id: "2",
          name: "Panel Event",
          start_time: hoursFromNow(2).toISOString(),
          category: { name: "Panel" },
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
          category: { name: "Workshop" },
          tags: [{ name: "Beginner" }],
        },
        {
          id: "2",
          name: "Event without Tag",
          start_time: hoursFromNow(2).toISOString(),
          category: { name: "Workshop" },
          tags: [{ name: "Advanced" }],
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
