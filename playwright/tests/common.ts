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
    start_time: new Date().toISOString(),
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
    title: "",
    body: "",
    ...page,
  }));

  await mockApiResponse(page, "/pages", {
    pages: data,
  });

  return data;
};

export const mockApi = async (
  path: Page,
  data: {
    info?: Record<string, unknown>;
    events?: Array<Record<string, unknown>>;
    pages?: Array<Record<string, unknown>>;
  },
) => ({
  info: await mockApiInfo(path, data.info ?? {}),
  events: await mockApiEvents(path, data.events ?? []),
  pages: await mockApiPages(path, data.pages ?? []),
});

export const isDesktop = () => test.info().project.name === "desktop";
export const isMobile = () => test.info().project.name === "mobile";

// Make sure all tests have a common definition of "now".
const now = new Date();

export const hoursFromNow = (hours: number) => {
  const newDate = new Date(now);
  newDate.setHours(newDate.getHours() + hours);
  return newDate;
};

export const daysFromNow = (days: number) => {
  return hoursFromNow(days * 24);
};
