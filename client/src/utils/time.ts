const shortTimeFormat = new Intl.DateTimeFormat(undefined, { timeStyle: "short" });
const shortDateFormat = new Intl.DateTimeFormat(undefined, { dateStyle: "short" });
const shortWeekdayFormat = new Intl.DateTimeFormat(undefined, { weekday: "short" });
const longWeekdayFormat = new Intl.DateTimeFormat(undefined, { weekday: "long" });
const daysInAWeek = 7;

export const localizeTime = (time: Date) => shortTimeFormat.format(time);

// TODO: What if the start and end days are more than a week apart? Unlikely,
// but in that case, we ought to show the full date.
export const localizeTimeSpan = (start: Date, end: Date | undefined) => {
  const startDay = shortWeekdayFormat.format(start);
  const endDay = end ? shortWeekdayFormat.format(end) : undefined;

  const startTime = shortTimeFormat.format(start);
  const endTime = end ? shortTimeFormat.format(end) : undefined;

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
  sortedDates.sort();

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

export const datesToDayNames = (dates: Set<Date>): Array<NamedDay> => {
  if (dates.size === 0) {
    return [];
  }

  const { start: earliest, end: latest } = dateRange(dates);
  const dayRange = daysBetween(earliest, latest);
  const rangeIsLongerThanAWeek = dayRange >= daysInAWeek;

  const sortedDates = [...dates];
  sortedDates.sort();

  const namedDays: Array<NamedDay> = [];

  for (let i = 0; i < sortedDates.length; i++) {
    const start = sortedDates[i];

    const dayName = rangeIsLongerThanAWeek
      ? shortDateFormat.format(start)
      : longWeekdayFormat.format(start);

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
  values: Array<T>,
  getTime: (value: T) => Date | undefined,
): Map<string, Array<T>> => {
  const grouped: Map<string, Array<T>> = new Map();

  for (const value of values) {
    const time = getTime(value);
    if (!time) {
      continue;
    }

    const timeString = localizeTime(time);

    if (!grouped.has(timeString)) {
      grouped.set(timeString, []);
    }

    grouped.get(timeString)?.push(value);
  }

  return grouped;
};
