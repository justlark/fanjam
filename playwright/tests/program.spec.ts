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
  test.beforeEach(async ({ page, programPage }) => {
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

    await programPage.goto();
  });

  test("events are sorted by start time, then end time", async ({ programPage }) => {
    await expect(programPage.eventNames).toHaveText([
      "Yesterday Event 1",
      "Yesterday Event 2",
      "Today Event 1",
      "Today Event 2",
    ]);
  });
});
