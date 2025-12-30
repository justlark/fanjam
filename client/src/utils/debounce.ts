// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Debounced<T extends (...args: any[]) => any> = (...args: Parameters<T>) => void;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  delayMs: number,
): Debounced<T> & { cancel: () => void } {
  let timer: ReturnType<typeof setTimeout> | undefined;

  const debounced = (...args: Parameters<T>) => {
    if (timer !== undefined) {
      clearTimeout(timer);
    }

    timer = setTimeout(() => {
      fn(...args);
      timer = undefined;
    }, delayMs);
  };

  debounced.cancel = () => {
    if (timer !== undefined) {
      clearTimeout(timer);
      timer = undefined;
    }
  };

  return debounced;
}
