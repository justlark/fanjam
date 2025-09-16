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

  async clearCategoryOrTag(name: string) {
    await this.description.getByRole("button").filter({ hasText: name }).click();
  }

  async clear() {
    await this.searchInput.clear();
  }
}

export class SchedulePage {
  private readonly page: Page;
  readonly events: Locator;
  readonly timeSlots: Locator;
  readonly hiddenNotice: Locator;
  readonly todayButton: Locator;
  readonly prevDayButton: Locator;
  readonly nextDayButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.events = page.getByTestId("schedule-event-link").filter({ visible: true });
    this.timeSlots = page.getByTestId("schedule-time-slot").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
    this.todayButton = page.getByTestId("schedule-today-button");
    this.prevDayButton = page.getByTestId("schedule-prev-day-button");
    this.nextDayButton = page.getByTestId("schedule-next-day-button");
  }

  async goto(day?: number | string) {
    await this.page.goto(day === undefined ? "schedule" : `schedule/${day}`);
  }

  async openEventDetailsPage(eventName: string) {
    await this.events.filter({ hasText: eventName }).click();

    if (isMobile()) {
      await new EventSummaryDrawer(this.page).openEventDetailsPage();
    }
  }

  async openEventSummaryDrawer(eventName: string) {
    if (isMobile()) {
      await this.events.filter({ hasText: eventName }).click();
    }
  }

  async toNextDay() {
    await this.nextDayButton.click();
  }

  async toPrevDay() {
    await this.prevDayButton.click();
  }

  async toToday() {
    await this.todayButton.click();
  }
}

export class EventSummaryDrawer {
  private readonly eventSummaryDrawer: Locator;
  private readonly eventSummaryDrawerCloseButton: Locator;
  private readonly eventSummaryDrawerExpandButton: Locator;
  private readonly tagbarCategoryLink: Locator;
  private readonly tagbarTagLinks: Locator;

  constructor(page: Page) {
    this.eventSummaryDrawer = page.getByTestId("event-summary-drawer").filter({ visible: true });
    this.eventSummaryDrawerCloseButton = this.eventSummaryDrawer.getByLabel("Close");
    this.eventSummaryDrawerExpandButton = this.eventSummaryDrawer.getByLabel("Expand");
    this.tagbarCategoryLink = this.eventSummaryDrawer
      .getByTestId("tagbar-category-link")
      .filter({ visible: true });
    this.tagbarTagLinks = this.eventSummaryDrawer
      .getByTestId("tagbar-tag-link")
      .filter({ visible: true });
  }

  async close() {
    await this.eventSummaryDrawerCloseButton.click();
  }

  async openEventDetailsPage() {
    await this.eventSummaryDrawerExpandButton.click();
  }

  async filterByCategory(name: string) {
    await this.tagbarCategoryLink.filter({ hasText: name }).click();
  }

  async filterByTag(name: string) {
    await this.tagbarTagLinks.filter({ hasText: name }).click();
  }
}

export class EventDetailsPage {
  private readonly backButton: Locator;
  private readonly tagbarCategoryLink: Locator;
  private readonly tagbarTagLinks: Locator;
  readonly starButton: Locator;

  constructor(page: Page) {
    this.backButton = page.getByTestId("event-details-back-button").filter({ visible: true });
    this.tagbarCategoryLink = page.getByTestId("tagbar-category-link").filter({ visible: true });
    this.tagbarTagLinks = page.getByTestId("tagbar-tag-link").filter({ visible: true });
    this.starButton = page.getByTestId("event-details-star-button").filter({ visible: true });
  }

  async toggleStar() {
    await this.starButton.click();
  }

  async navigateBack() {
    await this.backButton.click();
  }

  async filterByCategory(name: string) {
    await this.tagbarCategoryLink.filter({ hasText: name }).click();
  }

  async filterByTag(name: string) {
    await this.tagbarTagLinks.filter({ hasText: name }).click();
  }
}

export class ProgramPage {
  private readonly page: Page;
  readonly eventNames: Locator;
  readonly starButton: Locator;
  readonly hiddenNotice: Locator;
  readonly expandedEvents: Locator;

  constructor(page: Page) {
    this.page = page;
    this.eventNames = page.getByTestId("program-event-name").filter({ visible: true });
    this.starButton = page.getByTestId("program-event-star-button").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("program-past-events-hidden-notice");
    this.expandedEvents = this.page.getByTestId("program-event").filter({
      has: this.page.getByTestId("program-event-details-button").filter({ visible: true }),
    });
  }

  async goto() {
    await this.page.goto("program");
  }

  async toggleDayExpanded(index: number) {
    await this.page.getByTestId("program-day-expand-button").nth(index).click();
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

  eventName(eventName: string) {
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
