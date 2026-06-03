import { test as base, expect } from "@playwright/test";
import { SchedulePage } from "./fixtures";
import { mockApi, mockTime } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
};

const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

// Pin the locale and timezone so that `Intl.DateTimeFormat` output is
// deterministic across environments. With en-US, `dateStyle: "medium"`
// renders as e.g. "Sep 1, 2025", and `weekday: "long"` renders as e.g.
// "Monday".
test.use({ locale: "en-US", timezoneId: "UTC" });

const TZ_CONFIG = { config: { timezone: "UTC" } };

test.describe("the schedule view's date display", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

  test("daily view shows the full date alongside the weekday", async ({ page, schedulePage }) => {
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Monday Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Tuesday Event",
          start_time: "2025-09-02T10:00:00Z",
          end_time: "2025-09-02T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.dateName).toHaveText("Sep 1, 2025");

    await schedulePage.toNextDay();

    await expect(schedulePage.dayName).toHaveText("Tuesday");
    await expect(schedulePage.dateName).toHaveText("Sep 2, 2025");
  });

  test("daily view shows the date even when events span less than a week", async ({
    page,
    schedulePage,
  }) => {
    // Prior to this change, the weekday alone was shown when the schedule fit
    // inside a week. This test guards against regressing to that behavior.
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Monday Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Wednesday Event",
          start_time: "2025-09-03T10:00:00Z",
          end_time: "2025-09-03T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.dateName).toHaveText("Sep 1, 2025");
  });

  test("daily view shows the date for schedules spanning more than a week", async ({
    page,
    schedulePage,
  }) => {
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "First Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Late Event",
          start_time: "2025-09-15T10:00:00Z",
          end_time: "2025-09-15T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.dateName).toHaveText("Sep 1, 2025");

    await schedulePage.toNextDay();

    await expect(schedulePage.dayName).toHaveText("Monday");
    await expect(schedulePage.dateName).toHaveText("Sep 15, 2025");
  });

  test("all events view shows the weekday, date, and time in each time slot", async ({
    page,
    schedulePage,
  }) => {
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Monday Morning Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Tuesday Afternoon Event",
          start_time: "2025-09-02T14:00:00Z",
          end_time: "2025-09-02T15:00:00Z",
        },
      ],
    });

    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(2);

    const firstHeading = schedulePage.timeSlots.nth(0).getByRole("heading");
    await expect(firstHeading).toContainText("Monday");
    await expect(firstHeading).toContainText("Sep 1, 2025");
    await expect(firstHeading).toContainText("10:00 AM");

    const secondHeading = schedulePage.timeSlots.nth(1).getByRole("heading");
    await expect(secondHeading).toContainText("Tuesday");
    await expect(secondHeading).toContainText("Sep 2, 2025");
    await expect(secondHeading).toContainText("2:00 PM");
  });

  test("does not group events at the same time on different days in the all events view", async ({
    page,
    schedulePage,
  }) => {
    // Both events start at 10:00 UTC but on different days, so they must be
    // shown in separate time slots in the all-events view.
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Monday Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Tuesday Event",
          start_time: "2025-09-02T10:00:00Z",
          end_time: "2025-09-02T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(2);
    await expect(schedulePage.events).toHaveCount(2);

    const firstHeading = schedulePage.timeSlots.nth(0).getByRole("heading");
    await expect(firstHeading).toContainText("Monday");
    await expect(firstHeading).toContainText("Sep 1, 2025");
    await expect(firstHeading).toContainText("10:00 AM");

    const secondHeading = schedulePage.timeSlots.nth(1).getByRole("heading");
    await expect(secondHeading).toContainText("Tuesday");
    await expect(secondHeading).toContainText("Sep 2, 2025");
    await expect(secondHeading).toContainText("10:00 AM");

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Monday Event");
    await expect(schedulePage.timeSlots.nth(0)).not.toContainText("Tuesday Event");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Tuesday Event");
    await expect(schedulePage.timeSlots.nth(1)).not.toContainText("Monday Event");
  });

  test("groups events at the same time on the same day in the all events view", async ({
    page,
    schedulePage,
  }) => {
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Monday Event A",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Monday Event B",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Tuesday Event",
          start_time: "2025-09-02T10:00:00Z",
          end_time: "2025-09-02T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();
    await schedulePage.toAllEventsView();

    await expect(schedulePage.timeSlots).toHaveCount(2);
    await expect(schedulePage.events).toHaveCount(3);

    const firstHeading = schedulePage.timeSlots.nth(0).getByRole("heading");
    await expect(firstHeading).toContainText("Monday");
    await expect(firstHeading).toContainText("Sep 1, 2025");
    await expect(firstHeading).toContainText("10:00 AM");

    const secondHeading = schedulePage.timeSlots.nth(1).getByRole("heading");
    await expect(secondHeading).toContainText("Tuesday");
    await expect(secondHeading).toContainText("Sep 2, 2025");
    await expect(secondHeading).toContainText("10:00 AM");

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Monday Event A");
    await expect(schedulePage.timeSlots.nth(0)).toContainText("Monday Event B");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Tuesday Event");
  });

  test("does not show the date as part of the time slot in daily view", async ({
    page,
    schedulePage,
  }) => {
    // The date is shown once at the top of the day in daily view, so each
    // time slot heading should only contain the time of day, not the date.
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Morning Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    const heading = schedulePage.timeSlots.nth(0).getByRole("heading");
    await expect(heading).toHaveText("10:00 AM");
    await expect(heading).not.toContainText("Monday");
    await expect(heading).not.toContainText("Sep");
    await expect(heading).not.toContainText("2025");
  });

  test("labels each time slot in daily view with its own time of day", async ({
    page,
    schedulePage,
  }) => {
    await mockApi(page, {
      ...TZ_CONFIG,
      events: [
        {
          name: "Morning Event",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Lunchtime Event A",
          start_time: "2025-09-01T12:30:00Z",
          end_time: "2025-09-01T13:30:00Z",
        },
        {
          name: "Lunchtime Event B",
          start_time: "2025-09-01T12:30:00Z",
          end_time: "2025-09-01T13:30:00Z",
        },
        {
          name: "Afternoon Event",
          start_time: "2025-09-01T14:00:00Z",
          end_time: "2025-09-01T15:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(4);

    await expect(schedulePage.timeSlots.nth(0).getByRole("heading")).toHaveText("10:00 AM");
    await expect(schedulePage.timeSlots.nth(1).getByRole("heading")).toHaveText("12:30 PM");
    await expect(schedulePage.timeSlots.nth(2).getByRole("heading")).toHaveText("2:00 PM");

    // The 12:30 slot groups both lunchtime events under a single heading.
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Lunchtime Event A");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Lunchtime Event B");
  });
});
