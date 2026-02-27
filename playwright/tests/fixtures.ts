import { type Locator, type Page } from "@playwright/test";
import { envId, isMobile } from "./common";

export class FilterMenu {
  private readonly filterMenuButton: Locator;
  private readonly hidePastEventsButton: Locator;
  private readonly hideNotStarredEventsButton: Locator;
  private readonly categoryFilterList: Locator;
  private readonly tagFilterList: Locator;
  readonly searchInput: Locator;
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
  readonly dayName: Locator;
  readonly viewSelector: Locator;
  readonly noEventsNotice: Locator;

  constructor(page: Page) {
    this.page = page;
    this.events = page.getByTestId("schedule-event-link").filter({ visible: true });
    this.timeSlots = page.getByTestId("schedule-time-slot").filter({ visible: true });
    this.hiddenNotice = page.getByTestId("schedule-past-events-hidden-notice");
    this.todayButton = page.getByTestId("schedule-today-button");
    this.prevDayButton = page.getByTestId("schedule-prev-day-button");
    this.nextDayButton = page.getByTestId("schedule-next-day-button");
    this.dayName = page.getByTestId("schedule-day-name");
    this.viewSelector = page.getByTestId("schedule-view-selector");
    this.noEventsNotice = page.getByTestId("schedule-no-events");
  }

  async goto(day?: number | string) {
    await this.page.goto(day === undefined ? "schedule" : `schedule/${day}`);
  }

  async openEventDetailsPage(eventName: string) {
    await this.events.filter({ hasText: new RegExp(`^(Starred: )?${eventName}$`) }).click();

    if (isMobile()) {
      await new EventSummaryDrawer(this.page).openEventDetailsPage();
    }
  }

  async openEventSummaryDrawer(eventName: string) {
    if (isMobile()) {
      await this.events.filter({ hasText: eventName }).click();
    }
  }

  async toByDayView() {
    await this.viewSelector.getByRole("button").filter({ hasText: "By Day" }).click();
  }

  async toAllEventsView() {
    await this.viewSelector.getByRole("button").filter({ hasText: "All Events" }).click();
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
  private readonly drawerCloseButton: Locator;
  private readonly drawerExpandButton: Locator;
  private readonly tagbarCategoryLink: Locator;
  private readonly tagbarTagLinks: Locator;
  readonly starButton: Locator;

  constructor(page: Page) {
    this.eventSummaryDrawer = page.getByTestId("event-summary-drawer").filter({ visible: true });
    this.drawerCloseButton = this.eventSummaryDrawer.getByTestId("event-summary-close-button");
    this.drawerExpandButton = this.eventSummaryDrawer.getByTestId("event-summary-show-more-button");
    this.starButton = this.eventSummaryDrawer.getByTestId("event-summary-star-button");
    this.tagbarCategoryLink = this.eventSummaryDrawer
      .getByTestId("tagbar-category-link")
      .filter({ visible: true });
    this.tagbarTagLinks = this.eventSummaryDrawer
      .getByTestId("tagbar-tag-link")
      .filter({ visible: true });
  }

  async close() {
    await this.drawerCloseButton.click();
  }

  async openEventDetailsPage() {
    await this.drawerExpandButton.click();
  }

  async toggleStar() {
    await this.starButton.click();
  }

  async filterByCategory(name: string) {
    await this.tagbarCategoryLink.filter({ hasText: name }).click();
  }

  async filterByTag(name: string) {
    await this.tagbarTagLinks.filter({ hasText: name }).click();
  }
}

export class EventDetailsPage {
  private readonly page: Page;
  private readonly backButton: Locator;
  private readonly tagbarCategoryLink: Locator;
  private readonly tagbarTagLinks: Locator;
  readonly name: Locator;
  readonly time: Locator;
  readonly hosts: Locator;
  readonly location: Locator;
  readonly personLinks: Locator;
  readonly locationLinks: Locator;
  readonly starButton: Locator;
  readonly summary: Locator;
  readonly description: Locator;
  readonly noDescription: Locator;
  readonly content: Locator;

  constructor(page: Page) {
    this.page = page;
    this.backButton = page.getByTestId("event-details-back-button").filter({ visible: true });
    this.tagbarCategoryLink = page.getByTestId("tagbar-category-link").filter({ visible: true });
    this.tagbarTagLinks = page.getByTestId("tagbar-tag-link").filter({ visible: true });
    this.name = page.getByTestId("event-details-name");
    this.time = page.getByTestId("event-details-time");
    this.hosts = page.getByTestId("event-details-hosts");
    this.location = page.getByTestId("event-details-location");
    this.personLinks = page.getByTestId("event-details-person-link").filter({ visible: true });
    this.locationLinks = page.getByTestId("event-details-location-link").filter({ visible: true });
    this.starButton = page.getByTestId("event-details-star-button").filter({ visible: true });
    this.summary = page.getByTestId("event-details-summary");
    this.description = page.getByTestId("event-details-description");
    this.noDescription = page.getByTestId("event-details-no-description");
    this.content = page.getByTestId("event-details-content");
  }

  async goto(eventId: string) {
    await this.page.goto(`events/${eventId}`);
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

export class StarredEvents {
  private readonly page: Page;
  private readonly envId: string;

  constructor(page: Page) {
    this.page = page;
    this.envId = envId;
  }

  async get(): Promise<Array<string>> {
    // We debounce `localStorage.setItem` in some parts of the app, which
    // relies on `setTimeout`. If we're mocking the clock, that will never
    // resolve. So, we fast-forward the clock here to account for that and make
    // sure any calls to `localStorage.setItem` have landed.
    await this.page.clock.fastForward(1000);

    const rawJson = await this.page.evaluate(
      (envId) => localStorage.getItem(`starred:${envId}`),
      this.envId,
    );
    return rawJson ? JSON.parse(rawJson) : [];
  }

  async set(eventIds: Array<string>) {
    await this.page.evaluate(
      ([envId, eventIds]) => localStorage.setItem(`starred:${envId}`, JSON.stringify(eventIds)),
      [this.envId, eventIds],
    );
  }
}

export class AnnouncementsPage {
  private readonly page: Page;
  private readonly backButton: Locator;
  readonly link: Locator;
  readonly noDetailsNotice: Locator;
  readonly createdTime: Locator;
  readonly updatedTime: Locator;
  readonly attachmentsList: Locator;
  readonly emptyNotice: Locator;

  constructor(page: Page) {
    this.page = page;
    this.link = page.getByTestId("announcement-link").filter({ visible: true });
    this.createdTime = page.getByTestId("announcement-created-time");
    this.updatedTime = page.getByTestId("announcement-updated-time");
    this.noDetailsNotice = page.getByTestId("announcement-no-details-notice");
    this.attachmentsList = page.getByTestId("announcement-attachments-list");
    this.backButton = page.getByTestId("announcement-back-button");
    this.emptyNotice = page.getByTestId("announcements-empty");
  }

  async goto() {
    await this.page.goto("announcements");
  }

  async openDetails(index: number) {
    await this.link.nth(index).click();
  }

  async navigateBack() {
    await this.backButton.click();
  }
}

export class MainMenu {
  readonly mainMenuButton: Locator;
  readonly menu: Locator;
  readonly feedbackCallout: Locator;
  readonly scheduleLink: Locator;
  readonly myScheduleLink: Locator;
  readonly announcementsLink: Locator;
  readonly infoLink: Locator;

  constructor(page: Page) {
    this.mainMenuButton = page.getByTestId("main-menu-button");
    this.menu = isMobile()
      ? page.getByTestId("main-menu-drawer")
      : page.getByTestId("main-menu-sidebar");
    this.feedbackCallout = this.menu.getByTestId("feedback-callout");
    this.scheduleLink = this.menu.getByRole("link", { name: "Schedule", exact: true });
    this.myScheduleLink = this.menu.getByRole("link", { name: "My Schedule", exact: true });
    this.announcementsLink = this.menu.getByRole("link", { name: "Announcements", exact: true });
    this.infoLink = this.menu.getByRole("link", { name: "Info", exact: true });
  }

  async open() {
    await this.mainMenuButton.click();
  }

  async navigateToSchedule() {
    await this.scheduleLink.click();
  }

  async navigateToMySchedule() {
    await this.myScheduleLink.click();
  }

  async navigateToInfo() {
    await this.infoLink.click();
  }

  async navigateToAnnouncements() {
    await this.announcementsLink.click();
  }
}

export class InfoPage {
  private readonly page: Page;
  readonly name: Locator;
  readonly description: Locator;
  readonly websiteLink: Locator;
  readonly linksList: Locator;
  readonly externalLinks: Locator;
  readonly files: Locator;
  readonly pageLinks: Locator;

  constructor(page: Page) {
    this.page = page;
    this.name = page.getByTestId("info-page-name");
    this.description = page.getByTestId("info-page-description");
    this.websiteLink = page.getByTestId("info-page-website");
    this.linksList = page.getByTestId("links-list");
    this.externalLinks = page.getByTestId("links-list-external-link");
    this.files = page.getByTestId("links-list-file");
    this.pageLinks = page.getByTestId("links-list-page");
  }

  async goto() {
    await this.page.goto("info");
  }
}

export class CustomPage {
  private readonly page: Page;
  readonly viewer: Locator;
  readonly title: Locator;
  readonly body: Locator;
  readonly noDetailsNotice: Locator;
  readonly backButton: Locator;
  readonly files: Locator;

  constructor(page: Page) {
    this.page = page;
    this.viewer = page.getByTestId("page-viewer");
    this.title = page.getByTestId("page-viewer-title");
    this.body = page.getByTestId("page-viewer-body");
    this.noDetailsNotice = page.getByTestId("page-viewer-no-details");
    this.backButton = page.getByTestId("page-viewer-back-button");
    this.files = page.getByTestId("links-list-file");
  }

  async goto(pageId: string) {
    await this.page.goto(`pages/${pageId}`);
  }

  async navigateBack() {
    await this.backButton.click();
  }
}

export class SiteNav {
  readonly heading: Locator;
  readonly copyLinkButton: Locator;
  readonly refreshButton: Locator;
  readonly errorState: Locator;

  constructor(page: Page) {
    this.heading = page.getByTestId("site-nav-heading");
    this.copyLinkButton = page.getByTestId("site-nav-copy-link");
    this.refreshButton = page.getByTestId("site-nav-refresh");
    this.errorState = page.getByTestId("site-nav-error-state");
  }

  async copyLink() {
    await this.copyLinkButton.click();
  }

  async refresh() {
    await this.refreshButton.click();
  }
}

export class ShareDialog {
  readonly dialog: Locator;
  readonly description: Locator;
  readonly urlInput: Locator;
  readonly copyButton: Locator;
  readonly shareScheduleButton: Locator;

  constructor(page: Page) {
    this.dialog = page.getByTestId("share-dialog");
    this.description = this.dialog.getByTestId("share-dialog-description");
    this.urlInput = this.dialog.getByTestId("link-share-dialog-url");
    this.copyButton = this.dialog.getByTestId("link-share-dialog-copy-button");
    this.shareScheduleButton = this.dialog.getByTestId("share-dialog-share-schedule-button");
  }

  async copyLink() {
    await this.copyButton.click();
  }

  async openScheduleShare() {
    await this.shareScheduleButton.click();
  }
}

export class ScheduleShareDialog {
  readonly dialog: Locator;
  readonly urlInput: Locator;
  readonly copyButton: Locator;

  constructor(page: Page) {
    this.dialog = page.getByTestId("schedule-share-dialog");
    this.urlInput = this.dialog.getByTestId("link-share-dialog-url");
    this.copyButton = this.dialog.getByTestId("link-share-dialog-copy-button");
  }

  async copyLink() {
    await this.copyButton.click();
  }
}

export class ShareViewFooter {
  private readonly page: Page;
  readonly footer: Locator;
  readonly exitButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.footer = page.getByTestId("share-view-footer");
    this.exitButton = page.getByTestId("share-view-footer-exit-button");
  }

  async exit() {
    await this.exitButton.click();
    await this.page.getByTestId("schedule-share-options-return-button").click();
  }
}

export class ScheduleShareOptionsDialog {
  readonly dialog: Locator;
  readonly returnButton: Locator;
  readonly addButton: Locator;

  constructor(page: Page) {
    this.dialog = page.getByTestId("schedule-share-options-dialog");
    this.returnButton = page.getByTestId("schedule-share-options-return-button");
    this.addButton = page.getByTestId("schedule-share-options-add-button");
  }

  async returnToMySchedule() {
    await this.returnButton.click();
  }

  async addToMySchedule() {
    await this.addButton.click();
  }
}

export class ReadAnnouncements {
  private readonly page: Page;
  private readonly envId: string;

  constructor(page: Page) {
    this.page = page;
    this.envId = envId;
  }

  async get(): Promise<Array<string>> {
    await this.page.clock.fastForward(1000);

    const rawJson = await this.page.evaluate(
      (envId) => localStorage.getItem(`announcements:${envId}`),
      this.envId,
    );
    return rawJson ? JSON.parse(rawJson) : [];
  }

  async set(announcementIds: Array<string>) {
    await this.page.evaluate(
      ([envId, announcementIds]) =>
        localStorage.setItem(`announcements:${envId}`, JSON.stringify(announcementIds)),
      [this.envId, announcementIds],
    );
  }
}
