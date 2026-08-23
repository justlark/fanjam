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

interface DateParts {
  year: number;
  month: number;
  day: number;
}

// Returns the instant at which `minutesAfterMidnight` falls on the given
// calendar day in the given time zone.
const zonedTimeToInstant = (
  parts: DateParts,
  minutesAfterMidnight: number,
  timezone: string,
): Date => {
  const hour = Math.floor(minutesAfterMidnight / 60);
  const minute = minutesAfterMidnight % 60;

  const utcGuess = new Date(Date.UTC(parts.year, parts.month, parts.day, hour, minute, 0));

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

// Shifts a set of calendar date parts by whole days, normalizing the result
// (e.g. Sep 0 becomes Aug 31). This is calendar arithmetic, independent of any
// time zone.
const addCalendarDays = (parts: DateParts, days: number): DateParts => {
  const shifted = new Date(Date.UTC(parts.year, parts.month, parts.day + days));

  return {
    year: shifted.getUTCFullYear(),
    month: shifted.getUTCMonth(),
    day: shifted.getUTCDate(),
  };
};

// Returns the calendar day that `date` belongs to, accounting for the fact that
// the schedule day rolls over `cutoffMinutes` after midnight rather than at
// midnight itself.
const scheduleDayParts = (date: Date, timezone: string, cutoffMinutes: number): DateParts => {
  const dateParts = getDatePartsInTimezone(date, timezone);
  const timeParts = getTimePartsInTimezone(date, timezone);

  const minutesAfterMidnight = timeParts.hour * 60 + timeParts.minute;

  return minutesAfterMidnight < cutoffMinutes ? addCalendarDays(dateParts, -1) : dateParts;
};

// Returns the instant at which the schedule day containing `date` begins. Days
// roll over `cutoffMinutes` after midnight rather than at midnight, so with a
// 02:00 cutoff, a 01:00 Wednesday event belongs to Tuesday.
export const startOfScheduleDay = (date: Date, timezone: string, cutoffMinutes: number): Date => {
  return zonedTimeToInstant(
    scheduleDayParts(date, timezone, cutoffMinutes),
    cutoffMinutes,
    timezone,
  );
};

// Returns the half-open interval `[start, end)` of the schedule day containing
// `date`. The end is exclusive because a schedule day ends exactly where the
// next one begins, and because a day is not always 24 hours long across a DST
// transition.
const startAndEndOfDay = (
  date: Date,
  timezone: string,
  cutoffMinutes: number,
): { start: Date; end: Date } => {
  const parts = scheduleDayParts(date, timezone, cutoffMinutes);

  return {
    start: zonedTimeToInstant(parts, cutoffMinutes, timezone),
    end: zonedTimeToInstant(addCalendarDays(parts, 1), cutoffMinutes, timezone),
  };
};

// Parses an `HH:MM` 24-hour wall-clock time into minutes after midnight.
// Returns 0 (midnight) for a missing or malformed value.
export const parseDayCutoff = (value: string | undefined): number => {
  const match = /^(\d{1,2}):(\d{2})$/.exec(value?.trim() ?? "");

  if (!match) {
    return 0;
  }

  const hour = parseInt(match[1], 10);
  const minute = parseInt(match[2], 10);

  if (hour > 23 || minute > 59) {
    return 0;
  }

  return hour * 60 + minute;
};

export interface NamedDay {
  dayName: string;
  dateName: string;
  dayStart: Date;
  // Exclusive: the instant at which the next schedule day begins.
  dayEnd: Date;
}

// Groups a collection of event times into the schedule days they fall on,
// ordered chronologically. Day and date labels are derived from the start of
// the day rather than from any one event, so that an event in the small hours
// is labeled with the day it belongs to rather than the day it literally
// occurs on.
export const datesToDayNames = (
  formats: DatetimeFormats,
  dates: Iterable<Date>,
): Array<NamedDay> => {
  const namedDays: Map<number, NamedDay> = new Map();

  for (const date of dates) {
    const { start, end } = startAndEndOfDay(date, formats.timezone, formats.dayCutoffMinutes);
    const key = start.valueOf();

    if (namedDays.has(key)) {
      continue;
    }

    namedDays.set(key, {
      dayName: formats.longWeekday.format(start),
      dateName: formats.mediumDate.format(start),
      dayStart: start,
      dayEnd: end,
    });
  }

  return [...namedDays.entries()].sort(([a], [b]) => a - b).map(([, namedDay]) => namedDay);
};

// Whether `date` falls within the half-open interval `[start, end)`.
export const dateIsInRange = (date: Date, start: Date, end: Date): boolean => {
  return date >= start && date < end;
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
