import { type DatetimeFormats } from "@/composables/useDatetimeFormats";

export const localizeTime = (formats: DatetimeFormats, time: Date) =>
  formats.shortTime.format(time);

export const localizeDatetime = (formats: DatetimeFormats, datetime: Date) =>
  formats.shortDatetime.format(datetime);

// TODO: What if the start and end days are more than a week apart? Unlikely,
// but in that case, we ought to show the full date.
export const localizeTimeSpan = (formats: DatetimeFormats, start: Date, end: Date | undefined) => {
  const startDay = formats.shortWeekday.format(start);
  const endDay = end ? formats.shortWeekday.format(end) : undefined;

  const startTime = formats.shortTime.format(start);
  const endTime = end ? formats.shortTime.format(end) : undefined;

  if (!endTime || !endDay) {
    return `${startDay} ${startTime}`;
  } else if (startDay === endDay) {
    return `${startDay} ${startTime} - ${endTime}`;
  } else {
    return `${startDay} ${startTime} - ${endDay} ${endTime}`;
  }
};

export const dateIsBetween = (date: Date, start: Date, end: Date): boolean => {
  return date >= start && date <= end;
};

export const timeIsNearlyEqual = (a: Date, b: Date, thresholdMillis = 1000): boolean => {
  return Math.abs(a.valueOf() - b.valueOf()) <= thresholdMillis;
};

// Extracts year, month, and date components from a Date in a specific
// timezone.
const getDatePartsInTimezone = (
  date: Date,
  timezone: string,
): { year: number; month: number; day: number } => {
  const formatter = new Intl.DateTimeFormat("en-CA", {
    timeZone: timezone,
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });

  const parts = formatter.formatToParts(date);
  const year = parseInt(parts.find((p) => p.type === "year")?.value ?? "0", 10);
  const month = parseInt(parts.find((p) => p.type === "month")?.value ?? "0", 10) - 1;
  const day = parseInt(parts.find((p) => p.type === "day")?.value ?? "0", 10);

  return { year, month, day };
};

// Extracts hour, minute, and second components from a Date in a specific
// timezone.
const getTimePartsInTimezone = (
  date: Date,
  timezone: string,
): { hour: number; minute: number; second: number } => {
  const formatter = new Intl.DateTimeFormat("en-CA", {
    timeZone: timezone,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });

  const parts = formatter.formatToParts(date);
  const hour = parseInt(parts.find((p) => p.type === "hour")?.value ?? "0", 10);
  const minute = parseInt(parts.find((p) => p.type === "minute")?.value ?? "0", 10);
  const second = parseInt(parts.find((p) => p.type === "second")?.value ?? "0", 10);

  return { hour, minute, second };
};

// Returns midnight (00:00:00) of the same calendar day as `date` in the given
// time zone.
const midnightInTimeZone = (date: Date, timezone: string): Date => {
  const parts = getDatePartsInTimezone(date, timezone);
  const utcGuess = new Date(Date.UTC(parts.year, parts.month, parts.day, 0, 0, 0));

  const dateParts = getDatePartsInTimezone(utcGuess, timezone);
  const timeParts = getTimePartsInTimezone(utcGuess, timezone);

  const zonedMillis = Date.UTC(
    dateParts.year,
    dateParts.month,
    dateParts.day,
    timeParts.hour,
    timeParts.minute,
    timeParts.second,
  );

  return new Date(utcGuess.getTime() - (zonedMillis - utcGuess.getTime()));
};

const startAndEndOfDay = (date: Date, timezone: string): { start: Date; end: Date } => {
  const start = midnightInTimeZone(date, timezone);

  const nextDay = new Date(date);
  nextDay.setDate(nextDay.getDate() + 1);
  const nextMidnight = midnightInTimeZone(nextDay, timezone);

  const end = new Date(nextMidnight.getTime() - 1000);

  return { start, end };
};

export interface NamedDay {
  dayName: string;
  dateName: string;
  dayStart: Date;
  dayEnd: Date;
}

export const datesToDayNames = (formats: DatetimeFormats, dates: Set<Date>): Array<NamedDay> => {
  if (dates.size === 0) {
    return [];
  }

  const sortedDates = [...dates];
  sortedDates.sort((a, b) => a.valueOf() - b.valueOf());

  const namedDays: Array<NamedDay> = [];

  for (let i = 0; i < sortedDates.length; i++) {
    const start = sortedDates[i];

    const { start: startOfThisDay, end: endOfThisDay } = startAndEndOfDay(start, formats.timezone);

    for (let j = i + 1; j < sortedDates.length; j++) {
      const end = sortedDates[j];

      if (!dateIsBetween(end, start, endOfThisDay)) {
        break;
      }

      i++;
    }

    namedDays.push({
      dayName: formats.longWeekday.format(start),
      dateName: formats.mediumDate.format(start),
      dayStart: startOfThisDay,
      dayEnd: endOfThisDay,
    });
  }

  return namedDays;
};

export const groupByTime = <T, V>(
  values: Array<T>,
  getTime: (value: T) => Date,
  getKey: (time: Date) => string,
  getValue: (time: Date) => V,
): Array<[V, Array<T>]> => {
  // Maps in JS preserve insertion order, so as long as we sort the values, the
  // groups will be ordered temporally as well.
  const grouped: Map<string, [V, Array<T>]> = new Map();

  const sortedValues = [...values].sort((a, b) => getTime(a).valueOf() - getTime(b).valueOf());

  for (const value of sortedValues) {
    const time = getTime(value);
    const key = getKey(time);
    const resolvedValue = getValue(time);

    if (!grouped.has(key)) {
      grouped.set(key, [resolvedValue, []]);
    }

    const resolvedValueAndTime = grouped.get(key);
    if (resolvedValueAndTime) {
      resolvedValueAndTime[1].push(value);
    }
  }

  return [...grouped.values()];
};

export const isSameDay = (a: Date, b: Date, timezone: string): boolean => {
  const partsA = getDatePartsInTimezone(a, timezone);
  const partsB = getDatePartsInTimezone(b, timezone);

  return partsA.year === partsB.year && partsA.month === partsB.month && partsA.day === partsB.day;
};

export const earliest = (...dates: (Date | undefined)[]): Date | undefined => {
  return dates.reduce((a, b) => {
    if (a === undefined) return b;
    if (b === undefined) return a;
    return a.valueOf() < b.valueOf() ? a : b;
  }, undefined);
};

export const latest = (...dates: (Date | undefined)[]): Date | undefined => {
  return dates.reduce((a, b) => {
    if (a === undefined) return b;
    if (b === undefined) return a;
    return a.valueOf() > b.valueOf() ? a : b;
  }, undefined);
};
