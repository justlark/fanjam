import { test as base, expect } from "@playwright/test";
import { isMobile, mockApi, mockTime, hoursFromNow, envId } from "./common";
import {
  AppShareDialog,
  EventDetailsPage,
  EventShareDialog,
  EventSummaryDrawer,
  MainMenu,
  SchedulePage,
  ShareDialog,
  ScheduleShareDialog,
  ScheduleShareFooter,
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
  appShareDialog: AppShareDialog;
  eventShareDialog: EventShareDialog;
  scheduleShareDialog: ScheduleShareDialog;
  scheduleShareFooter: ScheduleShareFooter;
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
  appShareDialog: async ({ page }, use) => {
    await use(new AppShareDialog(page));
  },
  eventShareDialog: async ({ page }, use) => {
    await use(new EventShareDialog(page));
  },
  scheduleShareDialog: async ({ page }, use) => {
    await use(new ScheduleShareDialog(page));
  },
  scheduleShareFooter: async ({ page }, use) => {
    await use(new ScheduleShareFooter(page));
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

  test("opens from share button", async ({
    page,
    schedulePage,
    siteNav,
    shareDialog,
    appShareDialog,
  }) => {
    await schedulePage.goto();
    await siteNav.share();
    await shareDialog.openAppShare();

    const expectedShareUrl = `${new URL(page.url()).origin}/app/${envId}/schedule`;

    await expect(appShareDialog.description).toBeVisible();
    await expect(appShareDialog.urlInput).toHaveValue(expectedShareUrl);
    await expect(appShareDialog.description).toHaveText(/this app/);
  });

  test("event page has its own share button", async ({
    schedulePage,
    eventPage,
    eventShareDialog,
  }) => {
    await schedulePage.goto();
    await schedulePage.openEventDetailsPage("Test Event 1");
    await expect(eventPage.name).toHaveText("Test Event 1");
    await eventPage.shareButton.click();
    await expect(eventShareDialog.description).toHaveText(/this event/);
  });

  test("URL does not include query params", async ({
    page,
    siteNav,
    shareDialog,
    appShareDialog,
  }) => {
    await page.goto("schedule?c=Workshop&q=test");
    await siteNav.share();
    await shareDialog.openAppShare();

    await expect(appShareDialog.urlInput).toBeVisible();
    const urlValue = await appShareDialog.urlInput.inputValue();
    expect(urlValue).not.toContain("?");
  });

  test("URL points to app root", async ({ siteNav, shareDialog, appShareDialog, schedulePage }) => {
    await schedulePage.goto();
    await siteNav.share();
    await shareDialog.openAppShare();

    await expect(appShareDialog.urlInput).toBeVisible();
    const appUrl = new URL(await appShareDialog.urlInput.inputValue());
    expect(appUrl.pathname).toEqual(`/app/${envId}`);
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
    await siteNav.share();
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

  test("shows share view footer", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");

    await expect(scheduleShareFooter.footer).toBeVisible();
    await expect(scheduleShareFooter.footer).toHaveText(/someone else's schedule/);
  });

  test("does not show footer without share param", async ({
    schedulePage,
    scheduleShareFooter,
  }) => {
    await schedulePage.goto();

    await expect(scheduleShareFooter.footer).not.toBeVisible();
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

  test("preserved navigating to announcements", async ({ page, mainMenu, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    if (isMobile()) await mainMenu.open();
    await mainMenu.navigateToAnnouncements();

    await expect(page).toHaveURL(/share=Miwz/);
    await expect(scheduleShareFooter.footer).toBeVisible();
  });

  test("preserved navigating to info", async ({ page, mainMenu, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    if (isMobile()) await mainMenu.open();
    await mainMenu.navigateToInfo();

    await expect(page).toHaveURL(/share=Miwz/);
    await expect(scheduleShareFooter.footer).toBeVisible();
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

  test("exit removes share param", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await expect(scheduleShareFooter.footer).toBeVisible();

    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

    await expect(page).not.toHaveURL(/share=/);
    await expect(scheduleShareFooter.footer).not.toBeVisible();
  });

  test("exit shows toast", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

    await expect(page.getByText("Returning to your schedule")).toBeVisible();
  });

  test("exit restores user's own starred events", async ({
    page,
    schedulePage,
    scheduleShareFooter,
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

    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

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

  test("share param not preserved after exit", async ({ page, mainMenu, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

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
    scheduleShareFooter,
    schedulePage,
  }) => {
    await schedulePage.goto();
    await starredEvents.set(["1"]);

    await page.clock.fastForward(200);

    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

    expect(await starredEvents.get()).toEqual(["1"]);
  });
});

test.describe("schedule share options dialog", () => {
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

  test("opens from footer button", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();

    await expect(scheduleShareFooter.dialog).toBeVisible();
  });

  test("return exits share mode", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

    await expect(page).not.toHaveURL(/share=/);
    await expect(scheduleShareFooter.footer).not.toBeVisible();
  });

  test("return shows toast", async ({ page, scheduleShareFooter }) => {
    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.returnToMySchedule();

    await expect(page.getByText("Returning to your schedule")).toBeVisible();
  });

  test("add merges events", async ({ page, schedulePage, scheduleShareFooter, starredEvents }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);

    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.addToMySchedule();

    await expect(page).not.toHaveURL(/share=/);
    await expect(scheduleShareFooter.footer).not.toBeVisible();

    await expect(schedulePage.events.filter({ hasText: "Test Event 1" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 2" })).toHaveAccessibleName(
      /^Starred:/,
    );
    await expect(schedulePage.events.filter({ hasText: "Test Event 3" })).toHaveAccessibleName(
      /^Starred:/,
    );

    expect(await starredEvents.get()).toEqual(expect.arrayContaining(["1", "2", "3"]));
    expect((await starredEvents.get()).length).toBe(3);
  });

  test("add shows toast with count", async ({
    page,
    schedulePage,
    scheduleShareFooter,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);

    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.addToMySchedule();

    await expect(page.getByText("Added 2 events")).toBeVisible();
  });

  test("add shows toast when no new events", async ({
    page,
    schedulePage,
    scheduleShareFooter,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["2", "3"]);

    await page.goto("schedule?share=Miwz");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.addToMySchedule();

    await expect(page.getByText("already have all these events")).toBeVisible();
  });

  test("add preserves existing starred events", async ({
    page,
    schedulePage,
    scheduleShareFooter,
    starredEvents,
  }) => {
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);

    await page.goto("schedule?share=MQ");
    await scheduleShareFooter.openOptions();
    await scheduleShareFooter.addToMySchedule();

    expect(await starredEvents.get()).toEqual(["1"]);
  });
});
