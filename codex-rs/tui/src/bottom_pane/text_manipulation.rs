//! Text manipulation utilities for the chat composer.
//!
//! This module provides stateless, composable functions for handling text transformations
//! related to:
//!
//! - Placeholder expansion for large pastes
//! - Text element byte-range rebasing after trimming or expansion
//! - Newline normalization (CRLF → LF)
//! - Text boundary detection for character clamping

use codex_protocol::user_input::ByteRange;
use codex_protocol::user_input::TextElement;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Normalize newlines from various platforms to Unix-style LF.
///
/// Handles both Windows (CRLF) and old Mac (CR) line endings.
pub(crate) fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Clamp a position to a valid UTF-8 character boundary.
///
/// If the position falls in the middle of a multi-byte character, this function
/// backs up to the start of that character. This prevents accidental corruption
/// of UTF-8 sequences when manipulating text.
pub(crate) fn clamp_to_char_boundary(text: &str, pos: usize) -> usize {
    if pos >= text.len() {
        return text.len();
    }

    let mut clamped = pos;
    while clamped > 0 && !text.is_char_boundary(clamped) {
        clamped -= 1;
    }
    clamped
}

/// Adjust text element byte ranges after trimming whitespace.
///
/// When you trim leading or trailing whitespace from text, the byte positions of
/// any text elements (like placeholders or mentions) need to be rebased relative to
/// the new trimmed text.
///
/// This function filters out any elements that fall entirely outside the trimmed range
/// and adjusts the byte ranges of remaining elements.
///
/// # Arguments
///
/// * `original` - The original untrimmed text
/// * `trimmed` - The trimmed version of `original`
/// * `elements` - Text elements with byte ranges relative to `original`
///
/// # Returns
///
/// A new vector of text elements with byte ranges adjusted relative to `trimmed`.
pub(crate) fn trim_text_elements(
    original: &str,
    trimmed: &str,
    elements: Vec<TextElement>,
) -> Vec<TextElement> {
    if trimmed.is_empty() || elements.is_empty() {
        return Vec::new();
    }

    let trimmed_start = original.len().saturating_sub(original.trim_start().len());
    let trimmed_end = trimmed_start.saturating_add(trimmed.len());

    elements
        .into_iter()
        .filter_map(|elem| {
            let start = elem.byte_range.start;
            let end = elem.byte_range.end;

            // Skip elements entirely outside the trimmed range.
            if end <= trimmed_start || start >= trimmed_end {
                return None;
            }

            // Rebase byte ranges relative to the trimmed text.
            let new_start = start.saturating_sub(trimmed_start);
            let new_end = end.saturating_sub(trimmed_start).min(trimmed.len());

            if new_start >= new_end {
                return None;
            }

            let placeholder = trimmed.get(new_start..new_end).map(str::to_string);
            Some(TextElement::new(
                ByteRange {
                    start: new_start,
                    end: new_end,
                },
                placeholder,
            ))
        })
        .collect()
}

/// Expand large-paste placeholders and rebuild text element byte ranges.
///
/// Large pastes are initially stored as placeholders in the textarea (for UI performance),
/// and the actual content is kept in `pending_pastes`. When preparing a submission, this
/// function replaces placeholders with their actual content and rebases all other text
/// elements accordingly.
///
/// # Arguments
///
/// * `text` - The current text with placeholder references
/// * `elements` - Text elements (placeholders, mentions, etc.) with byte ranges into `text`
/// * `pending_pastes` - Map of placeholder strings → actual paste content
///
/// # Returns
///
/// A tuple of (expanded_text, rebased_elements) where:
/// - `expanded_text` has all placeholders replaced with actual content
/// - `rebased_elements` have byte ranges adjusted for the new text layout
pub(crate) fn expand_pending_pastes(
    text: &str,
    mut elements: Vec<TextElement>,
    pending_pastes: &[(String, String)],
) -> (String, Vec<TextElement>) {
    if pending_pastes.is_empty() || elements.is_empty() {
        return (text.to_string(), elements);
    }

    // Stage 1: Index pending paste payloads by placeholder for deterministic replacements.
    let mut pending_by_placeholder: HashMap<&str, VecDeque<&str>> = HashMap::new();
    for (placeholder, actual) in pending_pastes {
        pending_by_placeholder
            .entry(placeholder.as_str())
            .or_default()
            .push_back(actual.as_str());
    }

    // Stage 2: Sort elements by byte position.
    elements.sort_by_key(|elem| elem.byte_range.start);

    let mut rebuilt = String::with_capacity(text.len());
    let mut rebuilt_elements = Vec::with_capacity(elements.len());
    let mut cursor = 0usize;

    for elem in elements {
        let start = elem.byte_range.start.min(text.len());
        let end = elem.byte_range.end.min(text.len());

        if start > end {
            continue;
        }

        // Append any non-element text before this element.
        if start > cursor {
            rebuilt.push_str(&text[cursor..start]);
        }

        let elem_text = &text[start..end];
        let placeholder = elem.placeholder(text).map(str::to_string);

        // Check if this is a pending-paste placeholder.
        let replacement = placeholder
            .as_deref()
            .and_then(|ph| pending_by_placeholder.get_mut(ph))
            .and_then(VecDeque::pop_front);

        if let Some(actual) = replacement {
            // Inline the actual paste content; don't track a placeholder element.
            rebuilt.push_str(actual);
        } else {
            // Keep non-paste elements, updating their byte ranges.
            let new_start = rebuilt.len();
            rebuilt.push_str(elem_text);
            let new_end = rebuilt.len();
            let placeholder = placeholder.or_else(|| Some(elem_text.to_string()));

            rebuilt_elements.push(TextElement::new(
                ByteRange {
                    start: new_start,
                    end: new_end,
                },
                placeholder,
            ));
        }

        cursor = end;
    }

    // Stage 5: Append any trailing text after the last element.
    if cursor < text.len() {
        rebuilt.push_str(&text[cursor..]);
    }

    (rebuilt, rebuilt_elements)
}

/// Detect whether a string looks like a file path to an image.
///
/// Used to automatically attach images when pasting file paths instead of inserting
/// the path as plain text.
pub(crate) fn is_image_path(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    path_lower.ends_with(".png")
        || path_lower.ends_with(".jpg")
        || path_lower.ends_with(".jpeg")
        || path_lower.ends_with(".gif")
        || path_lower.ends_with(".webp")
        || path_lower.ends_with(".bmp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_newlines_handles_crlf() {
<<<<<<< HEAD
        assert_eq!(normalize_newlines("hello\r\nworld"), "hello\nworld");
=======
        assert_eq!(
            normalize_newlines("hello\r\nworld"),
            "hello\nworld"
        );
>>>>>>> origin/main
    }

    #[test]
    fn normalize_newlines_handles_cr() {
        assert_eq!(normalize_newlines("hello\rworld"), "hello\nworld");
    }

    #[test]
    fn normalize_newlines_preserves_lf() {
        assert_eq!(normalize_newlines("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn clamp_to_char_boundary_already_aligned() {
        let text = "hello";
        assert_eq!(clamp_to_char_boundary(text, 3), 3);
    }

    #[test]
    fn clamp_to_char_boundary_past_end() {
        let text = "hello";
        assert_eq!(clamp_to_char_boundary(text, 100), 5);
    }

    #[test]
    fn clamp_to_char_boundary_multibyte() {
        let text = "hello🌍"; // 🌍 is 4 bytes
        // Positions: h(0) e(1) l(2) l(3) o(4) 🌍(5-8)
        // If we try to clamp to position 6 (middle of 🌍), it should back up to 5
        let result = clamp_to_char_boundary(text, 6);
        assert_eq!(result, 5);
    }

    #[test]
    fn trim_text_elements_filters_outside_range() {
        let original = "  hello world  ";
        let trimmed = "hello world";
        let elements = vec![
            TextElement::new(ByteRange { start: 0, end: 2 }, Some("leading".to_string())),
            TextElement::new(ByteRange { start: 2, end: 7 }, Some("hello".to_string())),
<<<<<<< HEAD
            TextElement::new(
                ByteRange { start: 13, end: 15 },
                Some("trailing".to_string()),
            ),
=======
            TextElement::new(ByteRange { start: 13, end: 15 }, Some("trailing".to_string())),
>>>>>>> origin/main
        ];

        let result = trim_text_elements(original, trimmed, elements);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].byte_range.start, 0);
        assert_eq!(result[0].byte_range.end, 5);
    }

    #[test]
    fn expand_pending_pastes_replaces_placeholders() {
        let text = "[Paste #1] [Paste #2]";
        let elements = vec![
            TextElement::new(
                ByteRange { start: 0, end: 10 },
                Some("[Paste #1]".to_string()),
            ),
            TextElement::new(
                ByteRange { start: 11, end: 21 },
                Some("[Paste #2]".to_string()),
            ),
        ];
        let pending_pastes = vec![
            ("[Paste #1]".to_string(), "large content here".to_string()),
            ("[Paste #2]".to_string(), "more content".to_string()),
        ];

        let (expanded, new_elements) = expand_pending_pastes(&text, elements, &pending_pastes);

        assert_eq!(expanded, "large content here more content");
        assert_eq!(new_elements.len(), 0); // Both placeholders were expanded
    }

    #[test]
    fn expand_pending_pastes_preserves_non_paste_elements() {
        let text = "text [Paste #1] more";
<<<<<<< HEAD
        let elements = vec![TextElement::new(
            ByteRange { start: 5, end: 15 },
            Some("[Paste #1]".to_string()),
        )];
=======
        let elements = vec![
            TextElement::new(
                ByteRange { start: 5, end: 15 },
                Some("[Paste #1]".to_string()),
            ),
        ];
>>>>>>> origin/main
        let pending_pastes = vec![("[Paste #1]".to_string(), "EXPANDED".to_string())];

        let (expanded, new_elements) = expand_pending_pastes(&text, elements, &pending_pastes);

        assert_eq!(expanded, "text EXPANDED more");
        assert_eq!(new_elements.len(), 0);
    }

    #[test]
    fn is_image_path_detects_common_formats() {
        assert!(is_image_path("/path/to/image.png"));
        assert!(is_image_path("C:\\path\\image.jpg"));
        assert!(is_image_path("image.JPEG"));
        assert!(is_image_path("screenshot.gif"));
        assert!(is_image_path("photo.webp"));

        assert!(!is_image_path("/path/to/document.txt"));
        assert!(!is_image_path("archive.zip"));
    }
}
