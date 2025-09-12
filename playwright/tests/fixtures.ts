import { type Locator, type Page } from "@playwright/test";
import { envId, isMobile } from "./common";

export class FilterMenu {
  private readonly filterMenuButton: Locator;
  private readonly hidePastEventsButton: Locator;
  private readonly hideNotStarredEventsButton: Locator;
  private readonly categoryFilterList: Locator;
  private readonly tagFilterList: Locator;
  private readonly searchInput: Locator;
  readonly description: Locator;

  constructor(page: Page) {
    this.filterMenuButton = page.getByTestId("filter-menu-button");
    this.hidePastEventsButton = page.getByTestId("hide-past-events-button");
    this.hideNotStarredEventsButton = page.getByTestId("hide-not-starred-events-button");
    this.categoryFilterList = page.getByTestId("category-filter-list");
    this.tagFilterList = page.getByTestId("tag-filter-list");
    this.searchInput = page.getByTestId("filter-search-input");
    this.description = page.getByTestId("filter-description");
  }

  async toggleOpen() {
    await this.filterMenuButton.click();
  }

  async toggleHidePastEvents() {
    await this.hidePastEventsButton.click();
  }

  async toggleHideNotStarredEvents() {
    await this.hideNotStarredEventsButton.click();
  }

  async toggleCategory(category: string) {
    await this.categoryFilterList.getByRole("button", { name: category }).click();
  }

  async toggleTag(tag: string) {
    await this.tagFilterList.getByRole("button", { name: tag }).click();
  }

  async search(text: string) {
    await this.searchInput.fill(text);
  }

  async clear() {
    await this.searchInput.clear();
  }
}

export class SchedulePage {
  private readonly page: Page;
  private readonly eventSummaryDrawer: Locator;
  readonly events: Locator;
  readonly timeSlots: Locator;
  readonly hiddenNotice: Locator;
  readonly todayButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.eventSummaryDrawer = page.getByTestId("event-summary-drawer-expand-button");
    this.events = page.getByTestId("schedule-event-link").filter({ visible: true });
    this.timeSlots = page.getByTestId("schedule-time-slot").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
    this.todayButton = page.getByTestId("schedule-today-button");
  }

  async goto(day?: number | string) {
    await this.page.goto(day === undefined ? "schedule" : `schedule/${day}`);
  }

  async openEventDetailsPage(eventName: string) {
    await this.events.filter({ hasText: eventName }).first().click();

    if (isMobile()) {
      await this.eventSummaryDrawer.click();
    }
  }

  async toNextDay() {
    await this.page.getByTestId("schedule-next-day-button").click();
  }

  async toPrevDay() {
    await this.page.getByTestId("schedule-prev-day-button").click();
  }

  async toToday() {
    await this.todayButton.click();
  }
}

export class EventDetailsPage {
  readonly starButton: Locator;
  private readonly backButton: Locator;

  constructor(page: Page) {
    this.starButton = page.getByTestId("event-details-star-button").filter({ visible: true });
    this.backButton = page.getByTestId("event-details-back-button").filter({ visible: true });
  }

  async toggleStar() {
    await this.starButton.click();
  }

  async navigateBack() {
    await this.backButton.click();
  }
}

export class ProgramPage {
  private readonly page: Page;
  readonly eventNames: Locator;
  readonly starButton: Locator;
  readonly hiddenNotice: Locator;

  constructor(page: Page) {
    this.page = page;
    this.eventNames = page.getByTestId("program-event-name").filter({ visible: true });
    this.starButton = page.getByTestId("program-event-star-button").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("program-past-events-hidden-notice");
  }

  async goto() {
    await this.page.goto("program");
  }

  async toggleEventExpanded(eventName: string) {
    await this.page
      .getByTestId("program-event")
      .filter({ has: this.page.getByTestId("program-event-name").filter({ hasText: eventName }) })
      .getByTestId("program-event-expand-button")
      .click();
  }

  async openEventDetailsPage(eventName: string) {
    await this.page
      .getByTestId("program-event")
      .filter({ has: this.page.getByTestId("program-event-name").filter({ hasText: eventName }) })
      .getByTestId("program-event-details-button")
      .click();
  }

  eventName(eventName: string): Locator {
    return this.page.getByTestId("program-event-name").filter({ hasText: eventName });
  }

  async toggleStar() {
    await this.starButton.click();
  }
}

export class StarredEvents {
  private readonly page: Page;
  private readonly envId: string;

  constructor(page: Page) {
    this.page = page;
    this.envId = envId;
  }

  async get(): Promise<Array<string>> {
    const rawJson = await this.page.evaluate(
      (envId) => localStorage.getItem(`starred:${envId}`),
      this.envId,
    );
    return rawJson ? JSON.parse(rawJson) : [];
  }

  async set(eventIds: Array<string>) {
    this.page.evaluate(
      ([envId, eventIds]) => localStorage.setItem(`starred:${envId}`, JSON.stringify(eventIds)),
      [this.envId, eventIds],
    );
  }
}
