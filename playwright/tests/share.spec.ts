import { test as base, expect } from "@playwright/test";
import { isMobile, mockApi, mockTime, hoursFromNow, envId } from "./common";
import {
  EventDetailsPage,
  EventSummaryDrawer,
  MainMenu,
  SchedulePage,
  ShareDialog,
  ScheduleShareDialog,
  ShareViewFooter,
  SiteNav,
  StarredEvents,
} from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  eventPage: EventDetailsPage;
  summaryDrawer: EventSummaryDrawer;
  mainMenu: MainMenu;
  siteNav: SiteNav;
  shareDialog: ShareDialog;
  scheduleShareDialog: ScheduleShareDialog;
  shareViewFooter: ShareViewFooter;
  starredEvents: StarredEvents;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  eventPage: async ({ page }, use) => {
    await use(new EventDetailsPage(page));
  },
  summaryDrawer: async ({ page }, use) => {
    await use(new EventSummaryDrawer(page));
  },
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
  siteNav: async ({ page }, use) => {
    await use(new SiteNav(page));
  },
  shareDialog: async ({ page }, use) => {
    await use(new ShareDialog(page));
  },
  scheduleShareDialog: async ({ page }, use) => {
    await use(new ScheduleShareDialog(page));
  },
  shareViewFooter: async ({ page }, use) => {
    await use(new ShareViewFooter(page));
  },
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },
});

test.describe("share dialog", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
      announcements: [{ id: "a1", title: "Test Announcement" }],
      pages: [{ id: "p1", title: "Test Page" }],
    });
  });

  test("opens from copy link button", async ({ page, schedulePage, siteNav, shareDialog }) => {
    await schedulePage.goto();
    await siteNav.copyLink();

    const expectedShareUrl = `${new URL(page.url()).origin}/app/${envId}/schedule`;

    await expect(shareDialog.description).toBeVisible();
    await expect(shareDialog.urlInput).toHaveValue(expectedShareUrl);
    await expect(shareDialog.description).toHaveText(/this app/);
  });

  test("shows context-specific description", async ({
    page,
    schedulePage,
    siteNav,
    shareDialog,
    eventPage,
  }) => {
    // Event page
    await schedulePage.goto();
    await schedulePage.openEventDetailsPage("Test Event 1");
    await expect(eventPage.name).toHaveText("Test Event 1");
    await siteNav.copyLink();
    await expect(shareDialog.description).toHaveText(/this event/);

    // Dismiss dialog and navigate to announcement
    await page.keyboard.press("Escape");
    await page.goto("announcements/a1");
    await siteNav.copyLink();
    await expect(shareDialog.description).toHaveText(/this announcement/);

    // Custom page
    await page.keyboard.press("Escape");
    await page.goto("pages/p1");
    await siteNav.copyLink();
    await expect(shareDialog.description).toHaveText(/this page/);
  });

  test("URL does not include query params", async ({ page, siteNav, shareDialog }) => {
    await page.goto("schedule?c=Workshop&q=test");
    await siteNav.copyLink();

    await expect(shareDialog.urlInput).toBeVisible();
    const urlValue = await shareDialog.urlInput.inputValue();
    expect(urlValue).not.toContain("?");
  });
});

test.describe("schedule share modal", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
    });
  });

  test("opens from share dialog", async ({
    page,
    schedulePage,
    siteNav,
    shareDialog,
    scheduleShareDialog,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1", "3"]);
    await schedulePage.goto();
    await siteNav.copyLink();
    await shareDialog.openScheduleShare();

    await expect(scheduleShareDialog.urlInput).toBeVisible();
    const urlValue = await scheduleShareDialog.urlInput.inputValue();
    expect(urlValue).toContain("?share=MSwz");
  });
});

test.describe("viewing a shared schedule", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
    });
  });

  test("shows shared events as starred", async ({ page, schedulePage }) => {
    await page.goto("schedule?share=Miwz");

    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 3" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).not.toHaveAccessibleName(
      /^Starred:/,
    );
  });

  test("shows share view footer", async ({ page, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");

    await expect(shareViewFooter.footer).toBeVisible();
    await expect(shareViewFooter.footer).toHaveText(/someone else's schedule/);
  });

  test("does not show footer without share param", async ({ schedulePage, shareViewFooter }) => {
    await schedulePage.goto();

    await expect(shareViewFooter.footer).not.toBeVisible();
  });
});

test.describe("share param preserved across navigation", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
      announcements: [{ id: "a1", title: "Test Announcement" }],
    });
  });

  test("preserved navigating to announcements", async ({ page, mainMenu, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");
    if (isMobile()) await mainMenu.open();
    await mainMenu.navigateToAnnouncements();

    await expect(page).toHaveURL(/share=Miwz/);
    await expect(shareViewFooter.footer).toBeVisible();
  });

  test("preserved navigating to info", async ({ page, mainMenu, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");
    if (isMobile()) await mainMenu.open();
    await mainMenu.navigateToInfo();

    await expect(page).toHaveURL(/share=Miwz/);
    await expect(shareViewFooter.footer).toBeVisible();
  });

  test("preserved navigating to event details and back", async ({
    page,
    schedulePage,
    eventPage,
  }) => {
    await page.goto("schedule?share=Miwz");
    await schedulePage.openEventDetailsPage("Test Event 2");

    await expect(page).toHaveURL(/share=Miwz/);

    await eventPage.navigateBack();

    await expect(page).toHaveURL(/share=Miwz/);
  });
});

test.describe("exiting shared schedule", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
    });
  });

  test("exit removes share param", async ({ page, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");
    await expect(shareViewFooter.footer).toBeVisible();

    await shareViewFooter.exit();

    await expect(page).not.toHaveURL(/share=/);
    await expect(shareViewFooter.footer).not.toBeVisible();
  });

  test("exit shows toast", async ({ page, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");
    await shareViewFooter.exit();

    await expect(page.getByText("Returning to your schedule")).toBeVisible();
  });

  test("exit restores user's own starred events", async ({
    page,
    schedulePage,
    shareViewFooter,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?share=Miwz");

    // While viewing shared schedule, events 2,3 are starred
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );

    await shareViewFooter.exit();

    // After exit, user's own starred events are restored
    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).not.toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 3" })).not.toHaveAccessibleName(
      /^Starred:/,
    );

    expect(await starredEvents.get()).toEqual(["1"]);
  });

  test("share param not preserved after exit", async ({ page, mainMenu, shareViewFooter }) => {
    await page.goto("schedule?share=Miwz");
    await shareViewFooter.exit();

    if (isMobile()) await mainMenu.open();
    await mainMenu.navigateToAnnouncements();

    await expect(page).not.toHaveURL(/share=/);
  });
});

test.describe("star buttons hidden in share mode", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
      ],
    });
  });

  test("star button hidden on event details page", async ({ page, schedulePage, eventPage }) => {
    await page.goto("schedule?share=MQ");
    await schedulePage.openEventDetailsPage("Test Event 1");

    await expect(eventPage.starButton).toHaveCount(0);
  });

  test("star button visible outside share mode", async ({ schedulePage, eventPage }) => {
    await schedulePage.goto();
    await schedulePage.openEventDetailsPage("Test Event 1");

    await expect(eventPage.starButton).toBeVisible();
  });

  test("star button hidden in summary drawer", async ({ page, schedulePage, summaryDrawer }) => {
    if (!isMobile()) {
      test.skip();
    }

    await page.goto("schedule?share=MQ");
    await schedulePage.openEventSummaryDrawer("Test Event 1");

    await expect(summaryDrawer.starButton).toHaveCount(0);
  });

  test("schedule list still shows star icons", async ({ page, schedulePage }) => {
    await page.goto("schedule?share=MQ");

    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).toHaveAccessibleName(
      /^Starred:/,
    );
  });
});

test.describe("localStorage not clobbered", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        { id: "1", name: "Test Event 1", start_time: hoursFromNow(1).toISOString() },
        { id: "2", name: "Test Event 2", start_time: hoursFromNow(2).toISOString() },
        { id: "3", name: "Test Event 3", start_time: hoursFromNow(3).toISOString() },
      ],
    });
  });

  test("localStorage unchanged while viewing shared schedule", async ({
    page,
    starredEvents,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await starredEvents.set(["1"]);

    await page.clock.fastForward(200);

    await page.goto("schedule?share=Miwz");

    expect(await starredEvents.get()).toEqual(["1"]);
  });

  test("localStorage preserved after exiting", async ({
    page,
    starredEvents,
    shareViewFooter,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await starredEvents.set(["1"]);
    await page.goto("schedule?share=Miwz");
    await shareViewFooter.exit();

    await page.clock.fastForward(2000);

    expect(await starredEvents.get()).toEqual(["1"]);
  });
});
