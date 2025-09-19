import test, { Page } from "@playwright/test";

const newRandomId = () => Math.random().toString(10).substring(2, 6);

export const stub = {};
export const envId = "000000";

const mockApiResponse = async (page: Page, endpoint: string, body: unknown) => {
  const url = `https://api-test.fanjam.live/apps/*/${endpoint.replace(/^\//, "")}`;

  await page.route(url, async (route) => {
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        retry_after_ms: null,
        value: body,
      }),
    });
  });
};

const mockApiInfo = async (page: Page, info: Record<string, unknown>) => {
  const data = {
    name: null,
    description: null,
    website_url: null,
    links: [],
    files: [],
    ...info,
  };

  await mockApiResponse(page, "/info", data);

  return data;
};

const mockApiEvents = async (page: Page, events: Array<Record<string, unknown>>) => {
  const data = events.map((event) => ({
    id: newRandomId(),
    name: "Test Event",
    start_time: NOW.toISOString(),
    summary: null,
    description: null,
    end_time: null,
    location: null,
    people: [],
    category: null,
    tags: [],
    ...event,
  }));

  await mockApiResponse(page, "/events", {
    events: data,
  });

  return data;
};

const mockApiPages = async (page: Page, pages: Array<Record<string, unknown>>) => {
  const data = pages.map((page) => ({
    id: newRandomId(),
    title: "Test Page",
    body: "",
    ...page,
  }));

  await mockApiResponse(page, "/pages", {
    pages: data,
  });

  return data;
};

const mockApiAnnouncements = async (page: Page, announcements: Array<Record<string, unknown>>) => {
  const now = NOW.toISOString();

  const data = announcements.map((page) => ({
    id: newRandomId(),
    title: "Test Announcement",
    body: "",
    attachments: [],
    created_at: now,
    updated_at: now,
    ...page,
  }));

  await mockApiResponse(page, "/announcements", {
    announcements: data,
  });

  return data;
};

const mockApiConfig = async (page: Page, config: Record<string, unknown>) => {
  await mockApiResponse(page, "/config", config);

  return config;
};

export const mockApi = async (
  page: Page,
  data: {
    info?: Record<string, unknown>;
    events?: Array<Record<string, unknown>>;
    pages?: Array<Record<string, unknown>>;
    announcements?: Array<Record<string, unknown>>;
    config?: Record<string, unknown>;
  },
) => ({
  info: await mockApiInfo(page, data.info ?? {}),
  events: await mockApiEvents(page, data.events ?? []),
  pages: await mockApiPages(page, data.pages ?? []),
  announcements: await mockApiAnnouncements(page, data.announcements ?? []),
  config: await mockApiConfig(page, data.config ?? {}),
});

export const isDesktop = () => test.info().project.name === "desktop";
export const isMobile = () => test.info().project.name === "mobile";

const NOW = new Date("2025-09-01T09:00:00Z");

export const hoursFromNow = (hours: number): Date => {
  const newDate = new Date(NOW);
  newDate.setHours(newDate.getHours() + hours);
  return newDate;
};

export const mockTime = async (page: Page) => {
  await page.clock.setFixedTime(NOW);
};
