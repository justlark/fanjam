import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  // If there is a major, systemic breakage, we don't need to run every test.
  maxFailures: process.env.CI ? 20 : 0,
  reporter: [["list"], ["html", { open: "never" }]],
  projects: [
    {
      name: "desktop-chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "desktop-firefox",
      use: { ...devices["Desktop Firefox"] },
    },
    {
      name: "mobile-safari",
      use: { ...devices["iPhone 15 Pro"] },
    },
    {
      name: "mobile-chrome",
      use: { ...devices["Pixel 7"] },
    },
  ],
  use: {
    baseURL: "http://localhost:5173/app/playwright/",
    trace: "on-first-retry",
  },
  webServer: {
    command: "npm run --prefix ../client/ dev:playwright",
    url: "http://localhost:5173",
    reuseExistingServer: false,
  },
});
