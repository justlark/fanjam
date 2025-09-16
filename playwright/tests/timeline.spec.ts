import { test as base, expect } from "@playwright/test";
import { EventDetailsPage, SchedulePage } from "./fixtures";
import { hoursFromNow, mockApi } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("the schedule timeline view", () => {
  test("groups non-overlapping events in different time slots", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: hoursFromNow(-1).toISOString(),
          end_time: hoursFromNow(0).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(3);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 1");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Event 2");
    await expect(schedulePage.timeSlots.nth(2)).toContainText("Event 3");
  });

  test("groups events at the same time in the same time slot", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(2);
    await expect(schedulePage.events).toHaveCount(3);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 1");
    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 2");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Event 3");
  });
  test("groups overlapping events in different time slots", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: "2025-09-01T09:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Event 2",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T12:00:00Z",
        },
        {
          name: "Event 3",
          start_time: "2025-09-01T11:00:00Z",
          end_time: "2025-09-01T13:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(3);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 1");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Event 2");
    await expect(schedulePage.timeSlots.nth(2)).toContainText("Event 3");
  });

  test("groups events by start time", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: "2025-09-01T09:00:00Z",
          end_time: "2025-09-01T11:00:00Z",
        },
        {
          name: "Event 2",
          start_time: "2025-09-01T09:00:00Z",
          end_time: "2025-09-01T10:00:00Z",
        },
        {
          name: "Event 3",
          start_time: "2025-09-01T10:00:00Z",
          end_time: "2025-09-01T12:00:00Z",
        },
        {
          name: "Event 4",
          start_time: "2025-09-01T11:00:00Z",
          end_time: "2025-09-01T13:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.events).toHaveCount(4);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 1");
    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 2");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Event 3");
    await expect(schedulePage.timeSlots.nth(2)).toContainText("Event 4");
  });

  test("handles events that cross day boundaries", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: "2025-09-01T22:00:00Z",
          end_time: "2025-09-01T23:00:00Z",
        },
        {
          name: "Event 2",
          start_time: "2025-09-01T23:00:00Z",
          end_time: "2025-09-02T01:00:00Z",
        },
        {
          name: "Event 3",
          start_time: "2025-09-02T00:00:00Z",
          end_time: "2025-09-02T02:00:00Z",
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(2);
    await expect(schedulePage.events).toHaveCount(2);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 1");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("Event 2");

    await schedulePage.toNextDay();

    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(schedulePage.events).toHaveCount(1);

    await expect(schedulePage.timeSlots.nth(0)).toContainText("Event 3");
  });

  test("shows the current time slot", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: hoursFromNow(-1).toISOString(),
          end_time: hoursFromNow(0).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });

    await schedulePage.goto();

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.timeSlots.nth(1)).toContainText("now");
  });
});
