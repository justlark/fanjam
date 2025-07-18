// A hash algorithm that converts a string to a number.
const djb2Hash = (str: string) => {
  // This magic number is specified by the djb2 algorithm; we didn't choose it,
  // and shouldn't try to change it.
  let hash = 5381;

  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) + hash + str.charCodeAt(i);
  }

  return hash >>> 0;
};

// TODO: Rotate colors so that we don't use similar colors unless we have to.

const allPaletteColors = [
  "emerald",
  "green",
  "lime",
  "red",
  "orange",
  "amber",
  "yellow",
  "teal",
  "cyan",
  "sky",
  "blue",
  "indigo",
  "violet",
  "purple",
  "fuchsia",
  "pink",
  "rose",
];

const deterministicRandomColor = (str: string) => {
  const hash = djb2Hash(str);
  const index = hash % allPaletteColors.length;
  return allPaletteColors[index];
};

export const fgColorForString = (str: string, value: number) => {
  const color = deterministicRandomColor(str);
  return `text-${color}-${value}`;
};

export const bgColorForString = (str: string, value: number) => {
  const color = deterministicRandomColor(str);
  return `bg-${color}-${value}`;
};
