import { test as base, expect } from "@playwright/test";
import { EventDetailsPage, MainMenu, SchedulePage } from "./fixtures";
import { envId, hoursFromNow, isMobile, mockApi, mockTime } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
  mainMenu: MainMenu;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
});

test.describe("a multi-day schedule", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);

    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Yesterday Event",
          start_time: hoursFromNow(-25).toISOString(),
          end_time: hoursFromNow(-24).toISOString(),
        },
        {
          id: "2",
          name: "Today Event",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          id: "3",
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

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.events).toHaveText("Today Event");

    await schedulePage.toNextDay();
    await expect(schedulePage.dayName).toHaveText("Tuesday");
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Tomorrow Event");

    await schedulePage.toToday();
    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");

    await schedulePage.toPrevDay();
    await expect(schedulePage.dayName).toHaveText("Sunday");
    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");

    await schedulePage.toToday();
    await expect(schedulePage.dayName).toHaveText("Monday");
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
    page,
    eventPage,
    schedulePage,
  }) => {
    await schedulePage.goto();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");
    await expect(page).toHaveURL(new RegExp(`/schedule/2`));

    await schedulePage.toPrevDay();
    await schedulePage.openEventDetailsPage("Yesterday Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Yesterday Event");
    await expect(page).toHaveURL(new RegExp(`/schedule/1`));

    await schedulePage.toToday();
    await schedulePage.openEventDetailsPage("Today Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Today Event");
    await expect(page).toHaveURL(new RegExp(`/schedule/2`));

    await schedulePage.toNextDay();
    await schedulePage.openEventDetailsPage("Tomorrow Event");
    await eventPage.navigateBack();

    await expect(schedulePage.events).toHaveCount(1);
    await expect(schedulePage.events).toHaveText("Tomorrow Event");
    await expect(page).toHaveURL(new RegExp(`/schedule/3`));
  });

  test("back button from event page returns to the all events view", async ({
    page,
    eventPage,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await schedulePage.openEventDetailsPage("Today Event");
    await eventPage.navigateBack();

    await expect(page).toHaveURL(new RegExp(`/schedule/all`));

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(3);

    await expect(schedulePage.events.nth(0)).toContainText("Yesterday Event");
    await expect(schedulePage.events.nth(1)).toContainText("Today Event");
    await expect(schedulePage.events.nth(2)).toContainText("Tomorrow Event");
  });

  test("back button from event page adds a fragment for the event when navigating back to the daily view", async ({
    page,
    eventPage,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await schedulePage.openEventDetailsPage("Today Event");
    await eventPage.navigateBack();

    await expect(page).toHaveURL(new RegExp("#event-2$"));
  });

  test("back button from event page adds a fragment for the event when navigating back to the all events view", async ({
    page,
    eventPage,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await schedulePage.openEventDetailsPage("Today Event");
    await eventPage.navigateBack();

    await expect(page).toHaveURL(new RegExp("#event-2$"));
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

  test("the daily events URL brings you to the daily events view", async ({
    page,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await schedulePage.toNextDay();

    await expect(page).toHaveURL(new RegExp(`/schedule/3$`));

    await page.reload();

    await expect(page).toHaveURL(new RegExp(`/schedule/3$`));
  });

  test("the all events URL brings you to the all events view", async ({ page, schedulePage }) => {
    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(page).toHaveURL(new RegExp(`/schedule/all$`));

    await page.reload();

    await expect(page).toHaveURL(new RegExp(`/schedule/all$`));
  });

  test("the all events view shows events from all days", async ({ schedulePage }) => {
    await schedulePage.goto();

    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(3);

    await expect(schedulePage.events.nth(0)).toContainText("Yesterday Event");
    await expect(schedulePage.events.nth(1)).toContainText("Today Event");
    await expect(schedulePage.events.nth(2)).toContainText("Tomorrow Event");
  });

  test("time slots include the day when showing all events", async ({ schedulePage }) => {
    schedulePage.goto();

    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.timeSlots.nth(0).getByRole("heading")).toHaveText(/^Sunday/);
    await expect(schedulePage.timeSlots.nth(1).getByRole("heading")).toHaveText(/^Monday/);
    await expect(schedulePage.timeSlots.nth(2).getByRole("heading")).toHaveText(/^Tuesday/);
  });

  test("switch from daily events view to all events view on event details page", async ({
    page,
    schedulePage,
    eventPage,
  }) => {
    // We're specifically testing the double-pane view on desktop.
    if (isMobile()) {
      test.skip();
    }

    await schedulePage.goto();

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`/schedule/2$`));

    await eventPage.goto("3");

    await expect(schedulePage.dayName).toHaveText("Tuesday");
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`/events/3$`));

    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(page).toHaveURL(new RegExp(`/events/3$`));

    await schedulePage.openEventDetailsPage("Yesterday Event");

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(page).toHaveURL(new RegExp(`/events/1$`));
  });

  test("switch from all events view to daily events view on event details page", async ({
    page,
    schedulePage,
    eventPage,
  }) => {
    // We're specifically testing the double-pane view on desktop.
    if (isMobile()) {
      test.skip();
    }

    await schedulePage.goto();
    await schedulePage.toAllEventsView();
    await schedulePage.openEventDetailsPage("Tomorrow Event");

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(page).toHaveURL(new RegExp(`/events/3$`));

    await schedulePage.toByDayView();

    await expect(schedulePage.dayName).toHaveText("Tuesday");
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`/events/3$`));

    await schedulePage.toPrevDay();
    await schedulePage.openEventDetailsPage("Today Event");

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`/events/2$`));
  });

  test("navigating back to schedule view switches back to the daily events view", async ({
    page,
    schedulePage,
    mainMenu,
  }) => {
    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(page).toHaveURL(new RegExp(`/schedule/all$`));

    if (isMobile()) {
      await mainMenu.open();
    }

    await mainMenu.navigateToSchedule();

    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`/schedule$`));
  });
});
