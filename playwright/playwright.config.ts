import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["html", { open: "never" }]],
  projects: [
    {
      name: "desktop",
      use: {
        ...devices["Desktop Chrome"],
        ...devices["Desktop Firefox"],
        ...devices["Desktop Safari"],
      },
    },
    {
      name: "mobile",
      use: {
        ...devices["Pixel 5"],
        ...devices["iPhone 12"],
      },
    },
  ],
  use: {
    baseURL: `http://${process.env.CI ? "localhost" : "hostmachine"}:5173/app/000000/`,
    trace: "on-first-retry",
  },
  webServer: {
    command: process.env.CI ? "npm run --prefix ../client/ dev:test" : "just run-client",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
  },
});
