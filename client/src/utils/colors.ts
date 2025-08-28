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

const allPaletteColors = [
  "red",
  "green",
  "cyan",
  "purple",
  "yellow",
  "pink",
  "lime",
  "orange",
  "rose",
  "amber",
  "fuchsia",
  "teal",
  "sky",
  "violet",
  "emerald",

  // We're not using these colors because they're the primary colors for light
  // and dark mode respectively.

  // "blue",
  // "indigo",
];

// To avoid accidentally choosing similar colors, the palette above is sorted,
// and we choose random colors by popping them off the palette in order,
// cycling through if we run out.

const deterministicRandomColor = (seed: string, bank: ReadonlyArray<string>) => {
  const allHashes = bank.map(djb2Hash);
  allHashes.sort();

  const thisHash = djb2Hash(seed);
  const index = allHashes.indexOf(thisHash);

  return allPaletteColors[index % allPaletteColors.length];
};

export const newFgColor = (seed: string, bank: ReadonlyArray<string>, colorValue: number) => {
  const color = deterministicRandomColor(seed, bank);
  return `text-${color}-${colorValue.toString()}`;
};

export const newBgColor = (seed: string, bank: ReadonlyArray<string>, colorValue: number) => {
  const color = deterministicRandomColor(seed, bank);
  return `bg-${color}-${colorValue.toString()}`;
};

export const newOutlineColor = (seed: string, bank: ReadonlyArray<string>, colorValue: number) => {
  const color = deterministicRandomColor(seed, bank);
  return `outline-${color}-${colorValue.toString()}`;
};
