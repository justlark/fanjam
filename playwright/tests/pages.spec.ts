import { test as base, expect } from "@playwright/test";
import { mockApi } from "./common";
import { CustomPage, InfoPage } from "./fixtures";

type Fixtures = {
  customPage: CustomPage;
  infoPage: InfoPage;
};

export const test = base.extend<Fixtures>({
  customPage: async ({ page }, use) => {
    await use(new CustomPage(page));
  },
  infoPage: async ({ page }, use) => {
    await use(new InfoPage(page));
  },
});

test.describe("custom pages", () => {
  test("displays page title and body", async ({ page, customPage }) => {
    await mockApi(page, {
      pages: [
        {
          id: "rules",
          title: "Convention Rules",
          body: "Please follow these rules during the convention.",
          files: [],
        },
      ],
    });

    await customPage.goto("rules");

    await expect(customPage.title).toHaveText("Convention Rules");
    await expect(customPage.body).toContainText("Please follow these rules during the convention.");
  });

  test("renders markdown content as HTML", async ({ page, customPage }) => {
    await mockApi(page, {
      pages: [
        {
          id: "markdown-page",
          title: "Markdown Test",
          body: "# Heading\n\nThis is **bold** and *italic* text.\n\n- List item 1\n- List item 2",
          files: [],
        },
      ],
    });

    await customPage.goto("markdown-page");

    await expect(customPage.body).toBeVisible();
    await expect(customPage.body.locator("h1")).toHaveText("Heading");
    await expect(customPage.body.locator("strong")).toHaveText("bold");
    await expect(customPage.body.locator("em")).toHaveText("italic");
    await expect(customPage.body.locator("li")).toHaveCount(2);
  });

  test("displays page files", async ({ page, customPage }) => {
    await mockApi(page, {
      pages: [
        {
          id: "docs",
          title: "Documents",
          body: "Here are the documents:",
          files: [
            {
              name: "Schedule.pdf",
              media_type: "application/pdf",
              signed_url: "https://storage.example.com/schedule.pdf",
            },
            {
              name: "Map.png",
              media_type: "image/png",
              signed_url: "https://storage.example.com/map.png",
            },
          ],
        },
      ],
    });

    await customPage.goto("docs");

    await expect(customPage.files).toHaveCount(2);
    await expect(customPage.files.nth(0)).toContainText("Schedule.pdf");
    await expect(customPage.files.nth(1)).toContainText("Map.png");
  });

  test("shows no details message when page has no body and no files", async ({
    page,
    customPage,
  }) => {
    await mockApi(page, {
      pages: [
        {
          id: "empty-page",
          title: "Empty Page",
          body: "",
          files: [],
        },
      ],
    });

    await customPage.goto("empty-page");

    await expect(customPage.noDetailsNotice).toBeVisible();
    await expect(customPage.noDetailsNotice).toHaveText("No details provided");
  });

  test("does not show no details message when page has files but no body", async ({
    page,
    customPage,
  }) => {
    await mockApi(page, {
      pages: [
        {
          id: "files-only",
          title: "Files Only",
          body: "",
          files: [
            {
              name: "Document.pdf",
              media_type: "application/pdf",
              signed_url: "https://storage.example.com/doc.pdf",
            },
          ],
        },
      ],
    });

    await customPage.goto("files-only");

    await expect(customPage.noDetailsNotice).not.toBeVisible();
    await expect(customPage.files).toHaveCount(1);
  });

  test("redirects to info page for invalid page ID", async ({ page, customPage, infoPage }) => {
    await mockApi(page, {
      pages: [
        {
          id: "valid-page",
          title: "Valid Page",
          body: "Content",
          files: [],
        },
      ],
    });

    await customPage.goto("nonexistent-page");

    // Should redirect to info page when page is not found
    await expect(page).toHaveURL(/\/info$/);
  });

  test("back button navigates to info page", async ({ page, customPage, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
      },
      pages: [
        {
          id: "test-page",
          title: "Test Page",
          body: "Some content",
          files: [],
        },
      ],
    });

    await customPage.goto("test-page");
    await customPage.navigateBack();

    await expect(page).toHaveURL(/\/info$/);
    await expect(infoPage.name).toHaveText("Test Con");
  });

  test("navigates to custom page from info page", async ({ page, infoPage, customPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
      },
      pages: [
        {
          id: "rules",
          title: "Rules & Guidelines",
          body: "Follow the rules!",
          files: [],
        },
      ],
    });

    await infoPage.goto();
    await infoPage.pageLinks.filter({ hasText: "Rules & Guidelines" }).click();

    await expect(page).toHaveURL(/\/pages\/rules$/);
    await expect(customPage.title).toHaveText("Rules & Guidelines");
  });

  test("handles whitespace-only body as empty", async ({ page, customPage }) => {
    await mockApi(page, {
      pages: [
        {
          id: "whitespace-page",
          title: "Whitespace Page",
          body: "   \n\n   ",
          files: [],
        },
      ],
    });

    await customPage.goto("whitespace-page");

    await expect(customPage.noDetailsNotice).toBeVisible();
  });
});
