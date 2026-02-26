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
