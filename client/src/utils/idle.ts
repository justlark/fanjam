// Defer work until the browser has nothing better to do, so background
// refreshes never compete with rendering the page the user actually asked for.
//
// `requestIdleCallback` is missing on old version of Safari, so fall back to a
// short timeout.
export const onIdle = (callback: () => void, timeoutMs = 2000): void => {
  if (typeof requestIdleCallback === "function") {
    requestIdleCallback(
      () => {
        callback();
      },
      { timeout: timeoutMs },
    );
  } else {
    setTimeout(callback, 200);
  }
};
