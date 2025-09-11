import { test as base, expect } from "@playwright/test";
import { SchedulePage } from "./fixtures";
import { hoursFromNow, mockApi } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("grouping events in the schedule", () => {
  test("puts non-overlapping events in different time slots", async ({ page, schedulePage }) => {
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

  test("puts events at the same time in the same time slot", async ({ page, schedulePage }) => {
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
  test("puts overlapping events in different time slots", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(3).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(2).toISOString(),
          end_time: hoursFromNow(4).toISOString(),
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
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(3).toISOString(),
        },
        {
          name: "Event 4",
          start_time: hoursFromNow(2).toISOString(),
          end_time: hoursFromNow(4).toISOString(),
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
});
