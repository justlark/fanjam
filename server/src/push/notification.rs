//! Notification payload assembly: announcement title + markdown body
//! flattened to plain text and truncated to fit a push notification.

use pulldown_cmark::{Event, Parser, TagEnd};
use serde::Serialize;

/// Maximum length, in chars, of the plain-text notification body before we
/// ellipsize. Browsers themselves truncate further on most platforms; this
/// upper bound is just to keep the encrypted payload small and to avoid
/// shipping useless trailing markdown that the OS would clip anyway.
const MAX_BODY_CHARS: usize = 150;

/// Wire shape sent to the service worker inside the encrypted push body.
/// Fields match the keys the SW reads in its `push` handler.
#[derive(Debug, Serialize)]
pub struct Payload<'a> {
    pub title: &'a str,
    pub body: String,
    pub url: String,
    pub icon: Option<String>,
}

/// Flatten a Markdown string to a single line of plain text, then truncate
/// to roughly [`MAX_BODY_CHARS`] characters at a word boundary, appending
/// `…` if anything was dropped.
pub fn markdown_to_plain_text(markdown: &str) -> String {
    let mut buf = String::with_capacity(markdown.len().min(MAX_BODY_CHARS + 32));
    let mut needs_space = false;

    for event in Parser::new(markdown) {
        match event {
            Event::Text(text) | Event::Code(text) => {
                if needs_space && !buf.is_empty() {
                    buf.push(' ');
                }
                buf.push_str(&text);
                needs_space = false;
            }
            Event::SoftBreak | Event::HardBreak => {
                needs_space = true;
            }
            Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::Item
                | TagEnd::BlockQuote(_),
            ) => {
                needs_space = true;
            }
            _ => {}
        }
    }

    truncate_at_word_boundary(&buf, MAX_BODY_CHARS)
}

/// Truncate `s` so its char-count is at most `max_chars`. If we cut, back
/// up to the previous whitespace and append `…`. Char-based (not byte) so
/// multibyte titles don't get sliced mid-codepoint.
fn truncate_at_word_boundary(s: &str, max_chars: usize) -> String {
    let s = s.trim();
    if s.chars().count() <= max_chars {
        return s.to_string();
    }

    // Take `max_chars` chars and find the corresponding byte index.
    let byte_cutoff: usize = s
        .char_indices()
        .nth(max_chars)
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    let candidate = &s[..byte_cutoff];
    let trim_to = candidate
        .rfind(char::is_whitespace)
        .unwrap_or(candidate.len());

    let mut out = candidate[..trim_to].trim_end().to_string();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_strips_basic_markdown() {
        let input = "# Hello\n\nThis is **bold** and *italic* text.";
        let out = markdown_to_plain_text(input);
        assert_eq!(out, "Hello This is bold and italic text.");
    }

    #[test]
    fn plain_text_flattens_lists() {
        let input = "Things:\n\n- one\n- two\n- three";
        let out = markdown_to_plain_text(input);
        // Bullets become spaces between items.
        assert!(out.contains("one"));
        assert!(out.contains("two"));
        assert!(out.contains("three"));
        assert!(!out.contains('-'));
    }

    #[test]
    fn truncates_at_word_boundary_with_ellipsis() {
        // 200 chars of `ab ` repeats, far over the limit.
        let input = "ab ".repeat(70);
        let out = markdown_to_plain_text(&input);
        assert!(out.chars().count() <= MAX_BODY_CHARS + 1, "got {}", out);
        assert!(out.ends_with('…'));
        // No partial token before the ellipsis.
        let before_ellipsis = out.trim_end_matches('…');
        assert!(!before_ellipsis.ends_with('a'));
    }

    #[test]
    fn short_input_is_not_truncated() {
        let input = "Just a quick announcement.";
        assert_eq!(markdown_to_plain_text(input), "Just a quick announcement.");
    }

    #[test]
    fn handles_multibyte_chars_without_panicking() {
        // 100 emoji (4 bytes each = 400 bytes) but well within MAX_BODY_CHARS.
        let input = "🎉 ".repeat(50);
        let out = markdown_to_plain_text(&input);
        // No partial code points in the output.
        assert!(out.chars().all(|c| c == '🎉' || c == ' ' || c == '…'));
    }
}
