import { test as base, expect } from "@playwright/test";
import { isMobile, mockApi, mockTime } from "./common";
import { EventDetailsPage, EventSummaryDrawer, SchedulePage, StarredEvents } from "./fixtures";

type Fixtures = {
  starredEvents: StarredEvents;
  eventPage: EventDetailsPage;
  schedulePage: SchedulePage;
  summaryDrawer: EventSummaryDrawer;
};

export const test = base.extend<Fixtures>({
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },

  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },

  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },

  summaryDrawer: async ({ page }, use) => {
    await use(new EventSummaryDrawer(page));
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

  test("star button in the event summary drawer", async ({
    starredEvents,
    schedulePage,
    summaryDrawer,
  }) => {
    if (!isMobile()) {
      test.skip();
    }

    await schedulePage.goto();

    await schedulePage.openEventSummaryDrawer("Test Event");
    await expect(summaryDrawer.starButton).toHaveAttribute("aria-pressed", "false");
    await summaryDrawer.toggleStar();
    await expect(summaryDrawer.starButton).toHaveAttribute("aria-pressed", "true");

    expect(await starredEvents.get()).toEqual(["1"]);

    await expect(schedulePage.events.filter({ hasText: "Test Event" })).toHaveAccessibleName(
      /^Starred:/,
    );
  });
});
