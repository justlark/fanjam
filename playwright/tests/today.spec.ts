import { test as base, expect } from "@playwright/test";
import { EventDetailsPage, SchedulePage } from "./fixtures";
import { hoursFromNow, mockApi } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
});

test.describe("a multi-day schedule", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, {
      events: [
        {
          name: "Yesterday Event",
          start_time: hoursFromNow(-25).toISOString(),
          end_time: hoursFromNow(-24).toISOString(),
        },
        {
          name: "Today Event",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Tomorrow Event",
          start_time: hoursFromNow(24).toISOString(),
          end_time: hoursFromNow(25).toISOString(),
        },
      ],
    });
  });

  test("navigates to the current day by default", async ({ schedulePage }) => {
    await schedulePage.goto();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");
  });

  test("navigating to a day explicitly does not take you to the current day", async ({
    schedulePage,
  }) => {
    await schedulePage.goto(1);
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");

    await schedulePage.goto(3);
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Tomorrow Event");
  });

  test("navigating to day 0 takes you to the first day", async ({ page, schedulePage }) => {
    await schedulePage.goto(0);
    await expect(page).toHaveURL(new RegExp("/schedule/1$"));
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");
  });

  test("navigating to a day out of bounds takes you to the first day", async ({
    page,
    schedulePage,
  }) => {
    await schedulePage.goto(999);
    await expect(page).toHaveURL(new RegExp("/schedule/1$"));
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");
  });

  test("navigating to an invalid URL takes you to the first day", async ({
    page,
    schedulePage,
  }) => {
    await schedulePage.goto("foo");
    await expect(page).toHaveURL(new RegExp("/schedule/1$"));
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");
  });

  test("pressing the Today button brings you back to the current day", async ({ schedulePage }) => {
    await schedulePage.goto();

    await schedulePage.toNextDay();
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Tomorrow Event");

    await schedulePage.toToday();
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");

    await schedulePage.toPrevDay();
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");

    await schedulePage.toToday();
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");
  });

  test("the Today button is disabled on the current day", async ({ schedulePage }) => {
    await schedulePage.goto();
    await expect(schedulePage.todayButton).toBeDisabled();

    await schedulePage.toNextDay();
    await expect(schedulePage.todayButton).toBeEnabled();

    await schedulePage.toPrevDay();
    await expect(schedulePage.todayButton).toBeDisabled();
  });

  test("back button from event page returns to that event's day in the schedule", async ({
    eventPage,
    schedulePage,
  }) => {
    await schedulePage.goto();

    await schedulePage.toPrevDay();
    await schedulePage.openEventDetailsPage("Yesterday Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");

    await schedulePage.toToday();
    await schedulePage.openEventDetailsPage("Today Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");

    await schedulePage.toNextDay();
    await schedulePage.openEventDetailsPage("Tomorrow Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Tomorrow Event");
  });

  test("does not allow navigating before the first day or past the last day", async ({
    schedulePage,
  }) => {
    await schedulePage.goto();

    await schedulePage.toPrevDay();
    await expect(schedulePage.prevDayButton).toBeDisabled();
    await expect(schedulePage.nextDayButton).toBeEnabled();

    await schedulePage.toToday();
    await expect(schedulePage.prevDayButton).toBeEnabled();
    await expect(schedulePage.nextDayButton).toBeEnabled();

    await schedulePage.toNextDay();
    await expect(schedulePage.prevDayButton).toBeEnabled();
    await expect(schedulePage.nextDayButton).toBeDisabled();
  });
});
