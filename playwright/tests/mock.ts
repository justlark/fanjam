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

export interface MockApiInfo extends Record<string, unknown> { }

export const mockApiInfo = async (page: Page, info: MockApiInfo) => {
  return mockApi(page, "/info", {
    name: null,
    description: null,
    website_url: null,
    links: [],
    files: [],
    ...info,
  });
};

export interface MockApiEvent extends Record<string, unknown> {
  id: string;
  name: string;
  start_time: string;
}

export const mockApiEvents = async (page: Page, events: Array<MockApiEvent>) => {
  return mockApi(page, "/events", {
    events: events.map((event) => ({
      summary: null,
      description: null,
      end_time: null,
      location: null,
      people: [],
      category: null,
      tags: [],
      ...event,
    })),
  });
};

export interface MockApiPage extends Record<string, unknown> {
  id: string;
}

export const mockApiPages = async (page: Page, pages: Array<MockApiPage>) => {
  return mockApi(page, "/pages", {
    pages: pages.map((page) => ({
      title: "",
      body: "",
      ...page,
    })),
  });
};
