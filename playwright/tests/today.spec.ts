import { test as base, expect } from "@playwright/test";
import { SchedulePage } from "./fixtures";
import { daysFromNow, hoursFromNow, mockApi } from "./common";

type Fixtures = {
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("a multi-day event", () => {
  test.beforeEach(async ({ page }) => {
    mockApi(page, {
      events: [
        {
          name: "Yesterday Event",
          start_time: hoursFromNow(-24).toISOString(),
          end_time: hoursFromNow(-23).toISOString(),
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
});
