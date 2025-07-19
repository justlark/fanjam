const shortTimeFormat = new Intl.DateTimeFormat(undefined, { timeStyle: "short" });
const shortDateFormat = new Intl.DateTimeFormat(undefined, { dateStyle: "short" });
const shortWeekdayFormat = new Intl.DateTimeFormat(undefined, { weekday: "short" });
const longWeekdayFormat = new Intl.DateTimeFormat(undefined, { weekday: "long" });

export const localizeTime = (time: Date) => shortTimeFormat.format(time);

// TODO: What if the start and end days are more than a week apart? Unlikely,
// but in that case, we ought to show the full date.
export const localizeTimeSpan = (start: Date, end: Date | undefined) => {
  const startDay = start ? shortWeekdayFormat.format(start) : undefined;
  const endDay = end ? shortWeekdayFormat.format(end) : undefined;

  const startTime = start ? shortTimeFormat.format(start) : undefined;
  const endTime = end ? shortTimeFormat.format(end) : undefined;

  if (!end) {
    return `${startDay} ${startTime}`;
  } else if (startDay === endDay) {
    return `${startDay} ${startTime} - ${endTime}`;
  } else {
    return `${startDay} ${startTime} - ${endDay} ${endTime}`;
  }
};

const daysBetween = (start: Date, end: Date): number => {
  const dayMillis = 24 * 60 * 60 * 1000;
  return Math.round((start.valueOf() - end.valueOf()) / dayMillis);
};

export interface NamedDay {
  dayName: string;
  times: Set<Date>;
}

// TODO: Optimize this.
export const dateRangeToDayNames = (dates: Set<Date>): Array<NamedDay> => {
  const daysInAWeek = 7;

  const sortedDates = [...dates];
  sortedDates.sort();

  const [start, end] = [sortedDates[0], sortedDates[sortedDates.length - 1]];
  const dayRange = daysBetween(start, end);

  return sortedDates.reduce((namedDays, date) => {
    // If the range of dates is less than a week, use the names of the days of
    // the week. Otherwise, use the short localized form of the date.
    const dayName =
      dayRange < daysInAWeek ? longWeekdayFormat.format(date) : shortDateFormat.format(date);

    const times = namedDays.find((d) => d.dayName === dayName)?.times;

    if (times) {
      times.add(date);
    } else {
      namedDays.push({ dayName, times: new Set([date]) });
    }

    return namedDays;
  }, [] as Array<NamedDay>);
};
