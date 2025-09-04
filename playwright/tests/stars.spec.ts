import { test as base, expect } from "@playwright/test";
import { mockApi } from "./common";
import { EventDetailsPage, ProgramPage, SchedulePage, StarredEvents } from "./fixtures";

type Fixtures = {
  starredEvents: StarredEvents;
  eventPage: EventDetailsPage;
  programPage: ProgramPage;
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },

  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },

  programPage: async ({ page }, use) => {
    await use(new ProgramPage(page));
  },

  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("starring events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, { events: [{ id: "1", name: "Test Event" }] });
  });

  test("star button in event page", async ({ starredEvents, eventPage, schedulePage }) => {
    await schedulePage.goto();

    await schedulePage.openEventDetailsPage("Test Event");

    await expect(eventPage.starButton).toHaveAttribute("aria-pressed", "false");
    await eventPage.toggleStar();
    await expect(eventPage.starButton).toHaveAttribute("aria-pressed", "true");

    expect(await starredEvents.get()).toEqual(["1"]);

    await eventPage.navigateBack();

    await expect(schedulePage.events.filter({ hasText: "Test Event" })).toHaveAccessibleName(
      /^Starred:/,
    );
  });

  test("star button in program view", async ({ programPage, starredEvents }) => {
    await programPage.goto();

    await programPage.toggleEventExpanded("Test Event");

    await expect(programPage.starButton).toHaveAttribute("aria-pressed", "false");
    await programPage.toggleStar();
    await expect(programPage.starButton).toHaveAttribute("aria-pressed", "true");

    expect(await starredEvents.get()).toEqual(["1"]);

    await expect(programPage.eventName("Test Event")).toHaveText(/^Starred:/);
  });
});
