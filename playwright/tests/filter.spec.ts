import { test, expect, type Page } from '@playwright/test';
import { mockApi, hoursFromNow } from './common';

test.describe("filtering events", () => {
  test.beforeEach(async ({ page }) => {
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Test Event 1",
          category: "Category 1",
          tags: ["Tag 1", "Tag 2"],
          start_time: hoursFromNow(-2).toISOString(),
          end_time: hoursFromNow(-1).toISOString(),
        },
        {
          id: "2",
          name: "Test Event 2",
          category: "Category 2",
          tags: ["Tag 2", "Tag 3"],
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ]
    });
  });

  test("hide past events", async ({ page }) => {
    await page.goto("schedule");

    const pastEvent = page.getByTestId("schedule-event-link").filter({ visible: true, hasText: "Test Event 1" });
    const futureEvent = page.getByTestId("schedule-event-link").filter({ visible: true, hasText: "Test Event 2" });
    const hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
    const hidePastEventsButton = page.getByTestId("hide-past-events-button");

    await expect(pastEvent).toHaveCount(1);
    await expect(futureEvent).toHaveCount(1);
    await expect(hiddenNotice).toBeHidden();

    await page.getByTestId("filter-menu-button").click();
    await hidePastEventsButton.click();

    await expect(pastEvent).toHaveCount(0);
    await expect(futureEvent).toHaveCount(1);
    await expect(hiddenNotice).toBeVisible();

    await hidePastEventsButton.click();

    await expect(pastEvent).toHaveCount(1);
    await expect(futureEvent).toHaveCount(1);
    await expect(hiddenNotice).toBeHidden();
  });
});
