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

const SYNC_CODE_ALPHABET = "abcdefghijklmnopqrstuvwxyz0123456789";
const SYNC_CODE_LENGTH = 12;

// Generate a random schedule sync code: 12 characters of [a-z0-9]. The server validates this same
// shape. We use reject sampling over `crypto.getRandomValues` so each character is drawn uniformly
// from the 36-char alphabet (256 % 36 != 0, so plain modulo would bias the low characters).
export const generateSyncCode = (): string => {
  const maxUnbiased = 256 - (256 % SYNC_CODE_ALPHABET.length);
  let code = "";

  while (code.length < SYNC_CODE_LENGTH) {
    const bytes = new Uint8Array(SYNC_CODE_LENGTH);
    crypto.getRandomValues(bytes);
    for (const byte of bytes) {
      if (byte < maxUnbiased) {
        code += SYNC_CODE_ALPHABET[byte % SYNC_CODE_ALPHABET.length];
        if (code.length === SYNC_CODE_LENGTH) break;
      }
    }
  }

  return code;
};
