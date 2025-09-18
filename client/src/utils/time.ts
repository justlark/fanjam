import { type DatetimeFormats } from "@/composables/useDatetimeFormats";

const daysInAWeek = 7;

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

const dateRange = (dates: Set<Date>): { start: Date; end: Date } => {
  const sortedDates = [...dates];
  sortedDates.sort((a, b) => a.valueOf() - b.valueOf());

  const [start, end] = [sortedDates[0], sortedDates[sortedDates.length - 1]];

  return { start, end };
};

const daysBetween = (start: Date, end: Date): number => {
  const dayMillis = 24 * 60 * 60 * 1000;
  return Math.round(Math.abs(start.valueOf() - end.valueOf()) / dayMillis);
};

const startAndEndOfDay = (date: Date): { start: Date; end: Date } => {
  const start = new Date(date.getFullYear(), date.getMonth(), date.getDate());
  const end = new Date(start);
  end.setHours(23, 59, 59);
  return { start, end };
};

export interface NamedDay {
  dayName: string;
  dayStart: Date;
  dayEnd: Date;
}

export const datesToDayNames = (formats: DatetimeFormats, dates: Set<Date>): Array<NamedDay> => {
  if (dates.size === 0) {
    return [];
  }

  const { start: earliest, end: latest } = dateRange(dates);
  const dayRange = daysBetween(earliest, latest);
  const rangeIsLongerThanAWeek = dayRange >= daysInAWeek;

  const sortedDates = [...dates];
  sortedDates.sort((a, b) => a.valueOf() - b.valueOf());

  const namedDays: Array<NamedDay> = [];

  for (let i = 0; i < sortedDates.length; i++) {
    const start = sortedDates[i];

    const dayName = rangeIsLongerThanAWeek
      ? formats.shortDate.format(start)
      : formats.longWeekday.format(start);

    const { start: startOfThisDay, end: endOfThisDay } = startAndEndOfDay(start);

    for (let j = i + 1; j < sortedDates.length; j++) {
      const end = sortedDates[j];

      if (!dateIsBetween(end, start, endOfThisDay)) {
        break;
      }

      i++;
    }

    namedDays.push({
      dayName,
      dayStart: startOfThisDay,
      dayEnd: endOfThisDay,
    });
  }

  return namedDays;
};

export const groupByTime = <T>(
  formats: DatetimeFormats,
  values: Array<T>,
  getTime: (value: T) => Date,
): Map<string, Array<T>> => {
  // Maps in JS preserve insertion order, so as long as we sort the values, the
  // groups will be ordered temporally as well.
  const grouped: Map<string, Array<T>> = new Map();

  const sortedValues = [...values].sort((a, b) => getTime(a).valueOf() - getTime(b).valueOf());

  for (const value of sortedValues) {
    const time = getTime(value);
    const timeString = localizeTime(formats, time);

    if (!grouped.has(timeString)) {
      grouped.set(timeString, []);
    }

    grouped.get(timeString)?.push(value);
  }

  return grouped;
};

export const isSameDay = (a: Date, b: Date): boolean =>
  a.getFullYear() === b.getFullYear() &&
  a.getMonth() === b.getMonth() &&
  a.getDate() === b.getDate();
