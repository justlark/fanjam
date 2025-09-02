import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
  projects: [
    {
      name: "Chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "Firefox",
      use: { ...devices["Desktop Firefox"] },
    },

    {
      name: "WebKit",
      use: { ...devices["Desktop Safari"] },
    },
    {
      name: "Mobile Chrome",
      use: { ...devices["Pixel 5"] },
    },
    {
      name: "Mobile Safari",
      use: { ...devices["iPhone 12"] },
    },
  ],
  use: {
    baseURL: `http://${process.env.CI ? 'localhost' : 'hostmachine'}:5173/app/823685`,
    trace: "on-first-retry",
  },
  webServer: {
    command: process.env.CI ? "npm run --prefix ../client/ dev:test" : "just run-client",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
  },
});
