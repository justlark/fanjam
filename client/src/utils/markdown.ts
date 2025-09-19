import * as commonmark from "commonmark";

export const renderMarkdown = (markdown: string): string => {
  const reader = new commonmark.Parser({ smart: true });
  const writer = new commonmark.HtmlRenderer({ safe: true });
  const parsed = reader.parse(markdown);
  return writer.render(parsed);
};
