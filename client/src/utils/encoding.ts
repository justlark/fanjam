// A hash algorithm that converts a string to a number.
export const djb2Hash = (str: string) => {
  // This magic number is specified by the djb2 algorithm; we didn't choose it,
  // and shouldn't try to change it.
  let hash = 5381;

  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) + hash + str.charCodeAt(i);
  }

  return hash >>> 0;
};

export const decodeBase64urlBytes = (encoded: string): Uint8Array => {
  // @ts-expect-error This browser API is still very new.
  return Uint8Array.fromBase64(encoded, {
    alphabet: "base64url",
    omitPadding: true,
  });
};

export const encodeBase64url = (text: string): string => {
  const bytes = new TextEncoder().encode(text);
  // @ts-expect-error This browser API is still very new.
  return bytes.toBase64({
    alphabet: "base64url",
    omitPadding: true,
  });
};

export const decodeBase64url = (encoded: string): string => {
  // @ts-expect-error This browser API is still very new.
  const bytes = Uint8Array.fromBase64(encoded, {
    alphabet: "base64url",
    omitPadding: true,
  });
  return new TextDecoder().decode(bytes);
};
