import { test as base, expect } from "@playwright/test";
import { mockApi, isMobile } from "./common";
import { MainMenu, SchedulePage } from "./fixtures";

type Fixtures = {
  mainMenu: MainMenu;
  schedulePage: SchedulePage;
};

export const test = base.extend<Fixtures>({
  mainMenu: async ({ page }, use) => {
    await use(new MainMenu(page));
  },
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
});

test.describe("feedback", () => {
  test("the feedback callout is visible when configured and enabled", async ({
    page,
    mainMenu,
    schedulePage,
  }) => {
    await mockApi(page, {
      config: {
        use_feedback: true,
        feedback_icon: "some-icon",
        feedback_title: "Feedback",
        feedback_detail: "Feedback detail",
        feedback_url: "https://example.com",
      },
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.feedbackCallout).toBeVisible();
  });

  test("the feedback callout is not visible when configured but disabled", async ({
    page,
    mainMenu,
    schedulePage,
  }) => {
    await mockApi(page, {
      config: {
        use_feedback: false,
        feedback_icon: "some-icon",
        feedback_title: "Feedback",
        feedback_detail: "Feedback detail",
        feedback_url: "https://example.com",
      },
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.feedbackCallout).not.toBeVisible();
  });

  test("the feedback callout is not visible when enabled but not configured", async ({
    page,
    mainMenu,
    schedulePage,
  }) => {
    await mockApi(page, {
      config: {
        use_feedback: true,
      },
    });

    await schedulePage.goto();

    if (isMobile()) {
      await mainMenu.open();
    }

    await expect(mainMenu.feedbackCallout).not.toBeVisible();
  });
});
