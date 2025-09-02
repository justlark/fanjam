import { Page } from "@playwright/test";

export const mockApi = async (page: Page, endpoint: string, body: unknown) => {
  const url = `https://api-test.fanjam.live/apps/*/${endpoint.replace(/^\//, "")}`;

  await page.route(url, async route => {
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        retry_after_ms: null,
        value: body,
      }),
    });
  });
}
