import { test as base, expect } from "@playwright/test";
import { mockApi, hoursFromNow, mockTime } from "./common";
import { AnnouncementsPage } from "./fixtures";

type Fixtures = {
  announcementsPage: AnnouncementsPage;
};

export const test = base.extend<Fixtures>({
  announcementsPage: async ({ page }, use) => {
    await use(new AnnouncementsPage(page));
  },
});

test.describe("announcements", () => {
  test.beforeEach(async ({ page, announcementsPage }) => {
    await mockTime(page);
    await announcementsPage.goto();
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

    await announcementsPage.openDetails(0);

    await expect(announcementsPage.attachmentsList).not.toBeVisible();
  });
});
