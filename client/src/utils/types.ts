export const isNotNullish = <T>(value: T | null | undefined): value is T => {
  return value !== null && value !== undefined;
};

export const propIsNotNullish = <T, K extends keyof T>(
  key: K,
): ((obj: T) => obj is T & { [P in K]: NonNullable<T[K]> }) => {
  return (obj: T): obj is T & { [P in K]: NonNullable<T[K]> } => {
    return obj[key] !== null && obj[key] !== undefined;
  };
};
