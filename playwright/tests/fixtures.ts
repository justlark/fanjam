import { type Locator, type Page } from "@playwright/test";
import { envId } from "./common";

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
}

export class SchedulePage {
  readonly events: Locator;
  readonly hiddenNotice: Locator;

  constructor(page: Page) {
    this.events = page.getByTestId("schedule-event-link").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
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
    const rawJson = await this.page.evaluate((envId) => localStorage.getItem(`starred:${envId}`), this.envId)
    return rawJson ? JSON.parse(rawJson) : [];
  }

  async set(eventIds: Array<string>) {
    this.page.evaluate((envId) => localStorage.setItem(`starred:${envId}`, JSON.stringify(eventIds)), this.envId);
  }
}
