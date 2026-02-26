export const encodeBase64url = (text: string): string => {
  const bytes = new TextEncoder().encode(text);
  // @ts-expect-error This browser API is still very new API.
  return bytes.toBase64({
    alphabet: "base64url",
    omitPadding: true,
  });
};
