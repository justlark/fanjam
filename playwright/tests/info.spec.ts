import { test as base, expect } from "@playwright/test";
import { mockApi, isMobile, mockTime } from "./common";
import { InfoPage, MainMenu, SchedulePage } from "./fixtures";

type Fixtures = {
  infoPage: InfoPage;
  mainMenu: MainMenu;
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  infoPage: async ({ page }, use) => {
    await use(new InfoPage(page));
  },
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("info page", () => {
  test("displays con name and description", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Convention 2025",
        description: "This is a wonderful convention for testing purposes.",
      },
    });

    await infoPage.goto();

    await expect(infoPage.name).toHaveText("Test Convention 2025");
    await expect(infoPage.description).toHaveText(
      "This is a wonderful convention for testing purposes.",
    );
  });

  test("displays website URL when provided", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
        website_url: "https://example.com",
      },
    });

    await infoPage.goto();

    await expect(infoPage.websiteLink).toBeVisible();
    await expect(infoPage.websiteLink.getByRole("link")).toHaveAttribute(
      "href",
      "https://example.com",
    );
  });

  test("displays external links with correct URLs", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
        links: [
          { name: "Discord Server", url: "https://discord.gg/test" },
          { name: "Twitter", url: "https://twitter.com/testcon" },
        ],
      },
    });

    await infoPage.goto();

    await expect(infoPage.externalLinks).toHaveCount(2);
    await expect(infoPage.externalLinks.nth(0)).toContainText("Discord Server");
    await expect(infoPage.externalLinks.nth(1)).toContainText("Twitter");
  });

  test("displays files with download links", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
        files: [
          {
            name: "Schedule PDF",
            media_type: "application/pdf",
            signed_url: "https://storage.example.com/schedule.pdf",
          },
          {
            name: "Map Image",
            media_type: "image/png",
            signed_url: "https://storage.example.com/map.png",
          },
        ],
      },
    });

    await infoPage.goto();

    await expect(infoPage.files).toHaveCount(2);
    await expect(infoPage.files.nth(0)).toContainText("Schedule PDF");
    await expect(infoPage.files.nth(1)).toContainText("Map Image");
  });

  test("displays custom page links", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Test Con",
      },
      pages: [
        { id: "page-1", title: "Rules & Guidelines", body: "Some rules here" },
        { id: "page-2", title: "Venue Information", body: "Venue details" },
      ],
    });

    await infoPage.goto();

    await expect(infoPage.pageLinks).toHaveCount(2);
    await expect(infoPage.pageLinks.nth(0)).toContainText("Rules & Guidelines");
    await expect(infoPage.pageLinks.nth(1)).toContainText("Venue Information");
  });

  test("handles missing optional fields gracefully", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {
        name: "Minimal Con",
      },
    });

    await infoPage.goto();

    await expect(infoPage.name).toHaveText("Minimal Con");
    await expect(infoPage.description).not.toBeVisible();
    await expect(infoPage.websiteLink).not.toBeVisible();
    await expect(infoPage.externalLinks).toHaveCount(0);
    await expect(infoPage.files).toHaveCount(0);
  });

  test("navigates to info from main menu", async ({ page, schedulePage, mainMenu, infoPage }) => {
    await mockTime(page);
    await mockApi(page, {
      info: {
        name: "Test Convention",
        description: "Welcome to the convention!",
      },
      events: [{ id: "1", name: "Event 1" }],
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await mainMenu.navigateToInfo();

    await expect(infoPage.name).toHaveText("Test Convention");
    await expect(page).toHaveURL(/\/info$/);
  });

  test("shows default name when no name provided", async ({ page, infoPage }) => {
    await mockApi(page, {
      info: {},
    });

    await infoPage.goto();

    await expect(infoPage.name).toHaveText("FanJam");
  });
});
