import { test as base, expect } from "@playwright/test";
import { mockApi, mockTime, hoursFromNow } from "./common";
import { MyScheduleBanner, SchedulePage, StarredEvents } from "./fixtures";

type Fixtures = {
  schedulePage: SchedulePage;
  myScheduleBanner: MyScheduleBanner;
  starredEvents: StarredEvents;
};

export const test = base.extend<Fixtures>({
  schedulePage: async ({ page }, use) => {
    await use(new SchedulePage(page));
  },
  myScheduleBanner: async ({ page }, use) => {
    await use(new MyScheduleBanner(page));
  },
  starredEvents: async ({ page }, use) => {
    await use(new StarredEvents(page));
  },
});

test.describe("calendar export button", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Test Event 1",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
          location: { name: "Main Stage" },
          summary: "First test event",
        },
        {
          id: "2",
          name: "Test Event 2",
          start_time: hoursFromNow(3).toISOString(),
          end_time: hoursFromNow(4).toISOString(),
        },
        {
          // No end_time — should be excluded from the .ics.
          id: "3",
          name: "Test Event 3",
          start_time: hoursFromNow(5).toISOString(),
        },
      ],
      config: { use_calendar_export: true },
    });
  });

  test("downloads .ics with starred events", async ({
    page,
    schedulePage,
    myScheduleBanner,
    starredEvents,
  }) => {
    await schedulePage.goto();
    // Fast-forward must happen before set() so the debounced initial write
    // doesn't clobber the test's seeded value.
    await page.clock.fastForward(200);
    await starredEvents.set(["1", "2", "3"]);
    await page.goto("schedule?star=true");

    const downloadPromise = page.waitForEvent("download");
    await myScheduleBanner.downloadCalendar();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toBe("my-schedule.ics");

    const stream = await download.createReadStream();
    const chunks: Array<Buffer> = [];
    for await (const chunk of stream) {
      chunks.push(chunk as Buffer);
    }
    // Unfold per RFC 5545: long property values are split with CRLF + a
    // leading space/tab and must be rejoined before content assertions.
    const body = Buffer.concat(chunks)
      .toString("utf-8")
      .replace(/\r?\n[ \t]/g, "");

    // Both events with end_times are present.
    expect(body).toMatch(/SUMMARY:Test Event 1/);
    expect(body).toMatch(/SUMMARY:Test Event 2/);
    // The end-time-less event is skipped.
    expect(body).not.toMatch(/SUMMARY:Test Event 3/);

    // VEVENT count matches the exportable count.
    expect(body.match(/BEGIN:VEVENT/g) ?? []).toHaveLength(2);

    // Each entry's description ends with a link back to the app.
    expect(body).toMatch(/View in app: \S+\/events\/1/);
    expect(body).toMatch(/View in app: \S+\/events\/2/);

    // Dates serialize in UTC `Z` form.
    expect(body).toMatch(/DTSTART:\d{8}T\d{6}Z/);
    expect(body).toMatch(/DTEND:\d{8}T\d{6}Z/);
  });
});

test.describe("calendar export button (feature flag disabled)", () => {
  test.beforeEach(async ({ page }) => {
    await mockTime(page);
    await mockApi(page, {
      events: [
        {
          id: "1",
          name: "Test Event 1",
          start_time: hoursFromNow(1).toISOString(),
          end_time: hoursFromNow(2).toISOString(),
        },
      ],
      config: { use_calendar_export: false, use_schedule_sharing: false, use_schedule_sync: false },
    });
  });

  test("button is hidden when feature flag is disabled", async ({
    page,
    schedulePage,
    myScheduleBanner,
    starredEvents,
  }) => {
    // Star an event so the menu is gated only by the disabled feature flags,
    // not by the absence of starred events.
    await schedulePage.goto();
    await page.clock.fastForward(200);
    await starredEvents.set(["1"]);
    await page.goto("schedule?star=true");

    // With both share options disabled, the kebab menu that gates the calendar
    // download never renders.
    await expect(myScheduleBanner.optionsButton).toBeHidden();
  });
});
