import { test as base, expect } from "@playwright/test";
import { ProgramPage } from "./fixtures";
import { hoursFromNow, mockApi, mockTime } from "./common";

type Fixtures = {
  programPage: ProgramPage;
};

export const test = base.extend<Fixtures>({
  programPage: async ({ page }, use) => {
    await use(new ProgramPage(page));
  },
});

test.describe("the program view", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);

    await mockApi(page, {
      events: [
        {
          name: "Yesterday Event 1",
          start_time: hoursFromNow(-26).toISOString(),
          end_time: hoursFromNow(-25).toISOString(),
        },
        {
          name: "Yesterday Event 2",
          start_time: hoursFromNow(-25).toISOString(),
          end_time: hoursFromNow(-24).toISOString(),
        },
        {
          name: "Today Event 1",
          start_time: hoursFromNow(0).toISOString(),
          end_time: hoursFromNow(1).toISOString(),
        },
        {
          name: "Today Event 2",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
    });
  });

  test("expand and collapse all events on one day", async ({ programPage }) => {
    await programPage.goto();

    await expect(programPage.expandedEvents).toHaveCount(0);

    await programPage.toggleDayExpanded(0);

    await expect(programPage.expandedEvents).toHaveCount(2);

    await programPage.toggleDayExpanded(1);

    await expect(programPage.expandedEvents).toHaveCount(4);

    await programPage.toggleDayExpanded(0);
    await programPage.toggleDayExpanded(1);

    await expect(programPage.expandedEvents).toHaveCount(0);
  });
});
