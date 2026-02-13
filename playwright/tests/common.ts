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
      body: JSON.stringify(body),
    });
  });
};

const mockWrappedApiResponse = async (page: Page, endpoint: string, body: unknown) => {
  await mockApiResponse(page, endpoint, {
    stale: false,
    value: body,
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

  await mockWrappedApiResponse(page, "/info", data);

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

  await mockWrappedApiResponse(page, "/events", {
    events: data,
  });

  return data;
};

const mockApiPages = async (page: Page, pages: Array<Record<string, unknown>>) => {
  const data = pages.map((page) => ({
    id: newRandomId(),
    title: "Test Page",
    body: "",
    files: [],
    ...page,
  }));

  await mockWrappedApiResponse(page, "/pages", {
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

  await mockWrappedApiResponse(page, "/announcements", {
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

export const shiftTimeByHours = async (page: Page, hours: number) => {
  await page.clock.setFixedTime(hoursFromNow(hours));
};

export const mockApiError = async (
  page: Page,
  endpoint: string,
  statusCode: number,
  message?: string,
) => {
  const url = `https://api-test.fanjam.live/apps/*/${endpoint.replace(/^\//, "")}`;

  await page.route(url, async (route) => {
    await route.fulfill({
      status: statusCode,
      contentType: "application/json",
      body: JSON.stringify({ error: message ?? "Internal Server Error" }),
    });
  });
};

export const mockInfoError = async (page: Page, statusCode: number = 500) => {
  await mockApiError(page, "/info", statusCode);
};

export const mockWrappedApiResponseSequence = async (
  page: Page,
  endpoint: string,
  responses: Array<{ stale: boolean; body: unknown }>,
) => {
  let callCount = 0;
  const url = `https://api-test.fanjam.live/apps/*/${endpoint.replace(/^\//, "")}`;

  await page.route(url, async (route) => {
    const index = Math.min(callCount, responses.length - 1);
    callCount++;
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        stale: responses[index].stale,
        value: responses[index].body,
      }),
    });
  });
};

export const countRequestsTo = (page: Page, endpoint: string): { count: number } => {
  const counter = { count: 0 };
  const pattern = new RegExp(endpoint.replace(/^\//, ""));
  page.on("request", (request) => {
    if (pattern.test(request.url())) counter.count++;
  });
  return counter;
};
