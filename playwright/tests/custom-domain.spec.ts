import { test as base, expect } from "@playwright/test";
import { customDomainMode, hoursFromNow, mockApi, mockTime } from "./common";
import { SchedulePage, ShareModal, StarredEvents } from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  shareModal: ShareModal;
  starredEvents: StarredEvents;
};

const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  shareModal: async ({ page }, use) => {
    await use(new ShareModal(page));
  },
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },
});

// When the SPA boots on a custom domain it lives at the origin root (no `/app/<envId>/` prefix
// in any URL). All test navigation here is relative to the dev server's root so route
// resolution exercises the same code paths an attendee would hit on `app.example.org`.
test.use({
  baseURL: `http://${process.env.CI ? "localhost" : "hostmachine"}:5173/`,
});

test.describe("custom domain mode", () => {
  test.beforeEach(async ({ page }) => {
    await customDomainMode(page);
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
      ],
      config: { use_schedule_sharing: true },
    });
  });

  test("root path enters the schedule view", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveURL(/\/schedule(\/|$)/);
    await expect(page).not.toHaveURL(/\/app\//);
  });

  test("unprefixed deep link loads the schedule", async ({ page, schedulePage }) => {
    await page.goto("/schedule/1");
    await expect(page).toHaveURL(/\/schedule\/1$/);
    await expect(schedulePage.events.first()).toBeVisible();
  });

  test("share URL is built without the /app/<envId>/ prefix", async ({
    page,
    schedulePage,
    shareModal,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("/schedule?star=true");

    await page.getByTestId("schedule-share-button").click();

    await expect(shareModal.urlInput).toBeVisible();
    const urlValue = await shareModal.urlInput.inputValue();
    expect(urlValue).not.toContain("/app/");
    expect(urlValue).toMatch(/\/share\/\?s=MQ$/);
  });

  test("/share?s=... redirects to schedule/all", async ({ page }) => {
    await page.goto("/share/?s=MQ");
    await expect(page).toHaveURL(/\/schedule\/all\?star=true&share=MQ$/);
    await expect(page).not.toHaveURL(/\/app\//);
  });
});
