import { test as base, expect } from "@playwright/test";
import { mockApi, hoursFromNow, mockTime, isMobile } from "./common";
import { AnnouncementsPage, MainMenu, SchedulePage } from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  announcementsPage: AnnouncementsPage;
  mainMenu: MainMenu;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  announcementsPage: async ({ page }, use) => {
    await use(new AnnouncementsPage(page));
  },
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
});

test.describe("announcements", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
  });

  test("announcements are sorted in descending order by last updated date", async ({
    page,
    announcementsPage,
  }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "Announcement 1",
          created_at: hoursFromNow(-1).toISOString(),
          updated_at: hoursFromNow(-2).toISOString(),
        },
        {
          id: "2",
          title: "Announcement 2",
          created_at: hoursFromNow(-2).toISOString(),
          updated_at: hoursFromNow(-1).toISOString(),
        },
        {
          id: "3",
          title: "Announcement 3",
          created_at: hoursFromNow(-3).toISOString(),
          updated_at: hoursFromNow(-3).toISOString(),
        },
      ],
    });

    await announcementsPage.goto();

    await expect(announcementsPage.link).toHaveCount(3);
    await expect(announcementsPage.link.nth(0)).toContainText("Announcement 2");
    await expect(announcementsPage.link.nth(1)).toContainText("Announcement 1");
    await expect(announcementsPage.link.nth(2)).toContainText("Announcement 3");
  });

  test("announcements without a title are not shown in the list", async ({
    page,
    announcementsPage,
  }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "Announcement 1",
          updated_at: hoursFromNow(0).toISOString(),
        },
        {
          id: "2",
          title: "",
          updated_at: hoursFromNow(-1).toISOString(),
        },
        {
          id: "3",
          title: "   ",
          updated_at: hoursFromNow(-2).toISOString(),
        },
      ],
    });

    await announcementsPage.goto();

    await expect(announcementsPage.link).toHaveCount(1);
    await expect(announcementsPage.link.nth(0)).toContainText("Announcement 1");
  });

  test("announcements page with no body says so", async ({ page, announcementsPage }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "Announcement 1",
          updated_at: hoursFromNow(0).toISOString(),
        },
        {
          id: "2",
          title: "Announcement 1",
          body: "Announcement body",
          updated_at: hoursFromNow(-1).toISOString(),
        },
        {
          id: "3",
          title: "Announcement 1",
          attachments: [
            {
              name: "Attachment 1",
              media_type: "text/plain",
              signed_url: "https://example.com/attachment1.txt",
            },
          ],
          updated_at: hoursFromNow(-2).toISOString(),
        },
      ],
    });

    await announcementsPage.goto();

    await announcementsPage.openDetails(0);
    await expect(announcementsPage.noDetailsNotice).toBeVisible();
    await announcementsPage.navigateBack();

    await announcementsPage.openDetails(1);
    await expect(announcementsPage.noDetailsNotice).not.toBeVisible();
    await announcementsPage.navigateBack();

    await announcementsPage.openDetails(2);
    await expect(announcementsPage.noDetailsNotice).not.toBeVisible();
    await announcementsPage.navigateBack();
  });

  test("announcement details don't show updated time if it's the same as the created time", async ({
    page,
    announcementsPage,
  }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "Announcement 1",
          created_at: hoursFromNow(0).toISOString(),
          updated_at: hoursFromNow(0).toISOString(),
        },
        {
          id: "2",
          title: "Announcement 2",
          created_at: hoursFromNow(0).toISOString(),
          updated_at: hoursFromNow(-1).toISOString(),
        },
      ],
    });

    await announcementsPage.goto();

    await announcementsPage.openDetails(0);
    await expect(announcementsPage.createdTime).toBeVisible();
    await expect(announcementsPage.updatedTime).not.toBeVisible();
    await announcementsPage.navigateBack();

    await announcementsPage.openDetails(1);
    await expect(announcementsPage.createdTime).toBeVisible();
    await expect(announcementsPage.updatedTime).toBeVisible();
    await announcementsPage.navigateBack();
  });

  test("attachments list is visible", async ({ page, announcementsPage }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          title: "Announcement 1",
          attachments: [
            {
              name: "Attachment 1",
              media_type: "text/plain",
              signed_url: "https://example.com/attachment1.txt",
            },
          ],
        },
      ],
    });

    await announcementsPage.goto();

    await announcementsPage.openDetails(0);

    await expect(announcementsPage.attachmentsList).toBeVisible();
  });

  test("attachments list is not visible", async ({ page, announcementsPage }) => {
    await mockApi(page, {
      announcements: [
        {
          id: "1",
          attachments: [],
        },
      ],
    });

    await announcementsPage.goto();

    await announcementsPage.openDetails(0);

    await expect(announcementsPage.attachmentsList).not.toBeVisible();
  });

  test("the announcements link is hidden from the main menu when there are no announcements", async ({
    page,
    schedulePage,
    mainMenu,
  }) => {
    await mockApi(page, {
      config: {
        hide_announcements: true,
      },
      announcements: [],
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.announcementsLink).not.toBeVisible();
  });

  test("the announcements link is not hidden from the main menu when there are no announcements", async ({
    page,
    schedulePage,
    mainMenu,
  }) => {
    await mockApi(page, {
      config: {
        hide_announcements: false,
      },
      announcements: [],
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.announcementsLink).toBeVisible();
  });
});
