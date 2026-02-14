import { test as base, expect } from "@playwright/test";
import { SchedulePage } from "./fixtures";
import { hoursFromNow, mockApi, mockTime } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("the schedule timeline view", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

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

    await expect(schedulePage.timeSlots).toHaveCount(3);
    await expect(schedulePage.timeSlots.nth(0)).not.toContainText("now");
    await expect(schedulePage.timeSlots.nth(1)).toContainText("now");
    await expect(schedulePage.timeSlots.nth(2)).not.toContainText("now");
  });

  test("only shows the current time slot on the current day", async ({ page, schedulePage }) => {
    await mockApi(page, {
      events: [
        {
          name: "Event 1",
          start_time: hoursFromNow(-25).toISOString(),
          end_time: hoursFromNow(-24).toISOString(),
        },
        {
          name: "Event 2",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Event 3",
          start_time: hoursFromNow(24).toISOString(),
          end_time: hoursFromNow(25).toISOString(),
        },
      ],
    });

    await schedulePage.goto(1);
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(schedulePage.timeSlots.nth(0)).not.toContainText("now");

    await schedulePage.goto(2);
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(schedulePage.timeSlots.nth(0)).toContainText("now");

    await schedulePage.goto(3);
    await expect(schedulePage.timeSlots).toHaveCount(1);
    await expect(schedulePage.timeSlots.nth(0)).not.toContainText("now");
  });
});
