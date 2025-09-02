import { Page } from "@playwright/test";

const newRandomId = () => Math.random().toString(10).substring(2, 6);

export const stub = {};

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

export const mockApiInfo = async (page: Page, info: Record<string, unknown>) => {
  return mockApi(page, "/info", {
    name: null,
    description: null,
    website_url: null,
    links: [],
    files: [],
    ...info,
  });
};

const randomMinimalEvent = () => ({
  id: newRandomId(),
  name: "Test Event",
  start_time: new Date().toISOString(),
});

export const mockApiEvents = async (page: Page, events: Array<Record<string, unknown>>) => {
  return mockApi(page, "/events", {
    events: events.map((event) => ({
      summary: null,
      description: null,
      end_time: null,
      location: null,
      people: [],
      category: null,
      tags: [],
      ...randomMinimalEvent(),
      ...event,
    })),
  });
};

const randomMinimalPage = () => ({
  id: newRandomId(),
});

export const mockApiPages = async (page: Page, pages: Array<Record<string, unknown>>) => {
  return mockApi(page, "/pages", {
    pages: pages.map((page) => ({
      title: "",
      body: "",
      ...randomMinimalPage(),
      ...page,
    })),
  });
};
