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
