import { Page } from "@playwright/test";

const newRandomId = () => Math.random().toString(10).substring(2, 6);

export const stub = {};

const mockApiResponse = async (page: Page, endpoint: string, body: unknown) => {
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

const mockApiInfo = async (page: Page, info: Record<string, unknown>) => mockApiResponse(page, "/info", {
  name: null,
  description: null,
  website_url: null,
  links: [],
  files: [],
  ...info,
});

const mockApiEvents = async (page: Page, events: Array<Record<string, unknown>>) => mockApiResponse(page, "/events", {
  events: events.map((event) => ({
    id: newRandomId(),
    name: "Test Event",
    start_time: new Date().toISOString(),
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

const mockApiPages = async (page: Page, pages: Array<Record<string, unknown>>) => mockApiResponse(page, "/pages", {
  pages: pages.map((page) => ({
    id: newRandomId(),
    title: "",
    body: "",
    ...page,
  })),
});

export const mockApi = async (path: Page, data: { info?: Record<string, unknown>, events?: Array<Record<string, unknown>>, pages?: Array<Record<string, unknown>> }) => Promise.all([
  mockApiInfo(path, data.info ?? {}),
  mockApiEvents(path, data.events ?? []),
  mockApiPages(path, data.pages ?? []),
]);
