import { djb2Hash } from "./encoding";

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
  "indigo",

  // We're not using this color because it's the primary color for the theme.

  // "blue",
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
