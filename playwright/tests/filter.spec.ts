import { test, expect } from '@playwright/test';
import { mockApi, hoursFromNow } from './common';

test.describe("starring events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Test Event 1",
          category: "Category 1",
          tags: ["Tag 1", "Tag 2"],
          startTime: hoursFromNow(-2).toISOString(),
          endTime: hoursFromNow(-1).toISOString(),
        },
        {
          id: "2",
          name: "Test Event 2",
          category: "Category 2",
          tags: ["Tag 2", "Tag 3"],
          startTime: hoursFromNow(1).toISOString(),
          endTime: hoursFromNow(2).toISOString(),
        },
      ]
    });
  });

  test("hide past events", async ({ page }) => {
    await page.goto("schedule");

    const pastEvent = page.getByTestId("schedule-event-link").filter({ visible: true, hasText: "Test Event 1" });
    expect(pastEvent).toHaveCount(1);

    const futureEvent = page.getByTestId("schedule-event-link").filter({ visible: true, hasText: "Test Event 2" });
    expect(futureEvent).toHaveCount(1);

    await page.getByTestId("filter-menu-button").click();
    await page.getByTestId("hide-past-events-button").click();

    expect(pastEvent).toHaveCount(0);
    expect(futureEvent).toHaveCount(1);

    await page.getByTestId("hide-past-events-button").click();

    expect(pastEvent).toHaveCount(1);
    expect(futureEvent).toHaveCount(1);
  });
});
