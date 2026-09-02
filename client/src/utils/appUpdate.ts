// Ask the browser to re-fetch the service worker script and see whether a
// newer build is waiting. Otherwise, this only happens on a full page reload.
export const checkForAppUpdate = async (): Promise<void> => {
  if (!("serviceWorker" in navigator)) return;

  try {
    const registration = await navigator.serviceWorker.getRegistration();
    await registration?.update();
  } catch {
    // Deliberately swallow the error.
  }
};

// Minimum time to wait between polling for a new client bundle.
const checkIntervalMs = ((): number | undefined => {
  const raw = Number(import.meta.env.VITE_SW_RELOAD_CHECK_INTERVAL_MS);
  return Number.isFinite(raw) && raw > 0 ? raw : undefined;
})();

let lastCheckedAt = 0;

const JSON_MEDIA_TYPE = new RegExp("^application/json\\s*(;|$)", "i");

// The version of the client bundle the server is currently serving.
const fetchDeployedVersion = async (): Promise<string | undefined> => {
  try {
    const response = await fetch("/version.json", { cache: "no-store" });
    if (!response.ok) return undefined;

    // A missing asset returns the HTML of the app shell, meaning a 200 OK does
    // not necessarily mean we got the JSON we expected.
    if (!JSON_MEDIA_TYPE.test(response.headers.get("Content-Type") ?? "")) return undefined;

    const body: unknown = await response.json();
    const version = (body as { version?: unknown }).version;

    return typeof version === "string" ? version : undefined;
  } catch {
    return undefined;
  }
};

// Check whether the deployed bundle is a different version from the one the
// client is currently running. If so, update the service worker.
export const maybeCheckForAppUpdate = async ({ force = false } = {}): Promise<void> => {
  if (!("serviceWorker" in navigator)) return;
  if (!navigator.onLine) return;

  // The service worker is only registered when the app is deployed.
  const registration = await navigator.serviceWorker.getRegistration();
  if (!registration) return;

  // A new build is already downloaded and waiting for the user to accept the
  // update prompt.
  if (registration.waiting) return;

  const now = Date.now();

  if (!force) {
    if (checkIntervalMs === undefined) return;
    if (now - lastCheckedAt < checkIntervalMs) return;
  }

  lastCheckedAt = now;

  if (__BUILD_VERSION === undefined) return;

  const deployed = await fetchDeployedVersion();
  if (deployed === undefined || deployed === __BUILD_VERSION) return;

  await checkForAppUpdate();
};
