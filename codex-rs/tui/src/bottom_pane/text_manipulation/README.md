# Text Manipulation Module

This module provides stateless, composable utilities for text processing operations in the chat composer.

## Functions

### `normalize_newlines(text: &str) -> String`

Converts platform-specific line endings to Unix-style LF (`\n`).

**Handles:**
- Windows: `\r\n` → `\n`
- Mac (old): `\r` → `\n`
- Unix: `\n` → `\n` (unchanged)

**Use Case:** Normalizing pasted content from clipboard or external sources.

```rust
let pasted = "hello\r\nworld";
let normalized = normalize_newlines(pasted);
assert_eq!(normalized, "hello\nworld");
```

### `clamp_to_char_boundary(text: &str, pos: usize) -> usize`

Clamps a byte position to a valid UTF-8 character boundary.

**Why?** Prevents corruption of multi-byte UTF-8 sequences when manipulating text at arbitrary positions.

**Behavior:**
- If position is at a valid boundary, returns unchanged
- If position is in the middle of a multi-byte character, backs up to the character start
- If position is past the end, returns text length

**Use Case:** Cursor positioning, text splitting, substring extraction.

```rust
let text = "hello🌍";  // 🌍 is 4 bytes at positions 5-8
let safe_pos = clamp_to_char_boundary(text, 6);  // Middle of 🌍
assert_eq!(safe_pos, 5);  // Backed up to character start
```

### `trim_text_elements(...) -> Vec<TextElement>`

Rebases text element byte ranges after whitespace trimming.

**Why?** When you trim leading/trailing whitespace, text elements (mentions, placeholders) need updated byte positions.

**What it does:**
1. Identifies the start/end of trimmed region
2. Filters out elements entirely outside the trimmed range
3. Adjusts byte ranges of remaining elements

**Use Case:** Preparing text for submission by trimming and rebasing associated metadata.

```rust
let original = "  hello world  ";
let trimmed = "hello world";
let elements = vec![
    TextElement::new(
        ByteRange { start: 2, end: 7 },
        Some("hello".to_string()),
    ),
];
let rebased = trim_text_elements(original, trimmed, elements);
// Byte ranges now relative to "hello world" instead of original
```

### `expand_pending_pastes(...) -> (String, Vec<TextElement>)`

Replaces large-paste placeholders with actual content and rebases all elements.

**Why?** Large pastes (>1000 chars) are initially stored as placeholders for UI performance. On submission, they must be expanded and all element byte ranges rebased.

**Algorithm (5 stages):**
1. **Index:** Create HashMap of placeholder → actual content
2. **Sort:** Order elements by byte position
3. **Walk:** Iterate through text, replacing placeholders and rebuilding
4. **Keep:** Preserve non-paste elements with updated byte ranges
5. **Append:** Add any trailing text after the last element

**Use Case:** Finalizing text submission when paste buffers are present.

```rust
let text = "[Paste #1] hello";
let elements = vec![
    TextElement::new(ByteRange { start: 0, end: 10 }, Some("[Paste #1]".to_string())),
];
let pending = vec![
    ("[Paste #1]".to_string(), "large content here".to_string()),
];
let (expanded, new_elements) = expand_pending_pastes(&text, elements, &pending);
assert_eq!(expanded, "large content here hello");
```

### `is_image_path(path: &str) -> bool`

Detects whether a string looks like a file path to an image.

**Supported formats:** PNG, JPG, JPEG, GIF, WebP, BMP (case-insensitive)

**Use Case:** Automatically attaching images when pasting file paths.

```rust
assert!(is_image_path("/path/to/photo.png"));
assert!(is_image_path("C:\\Users\\photo.JPG"));
assert!(!is_image_path("document.txt"));
```

## Design Decisions

### Why Stateless Functions?

Instead of a `TextManipulation` struct with methods:

✅ **Easier to test** — no setup/teardown
✅ **More composable** — functions can be chained
✅ **Reusable** — no coupling to textarea or composer state
✅ **Better for parallelization** — immutable inputs

### Why Explicit Parameters?

Instead of accessing `self.textarea` or `self.pending_pastes`:

✅ **Referential transparency** — same inputs always produce same outputs
✅ **Clear dependencies** — function signature shows what it needs
✅ **Testability** — no mocking required
✅ **Reusability** — can be called from anywhere

## Testing

The module includes 11 comprehensive tests covering:

- **Newline normalization:** CRLF, CR, mixed
- **Boundary clamping:** ASCII, multi-byte UTF-8, edge cases
- **Element trimming:** Filtering, rebasing, empty cases
- **Placeholder expansion:** Single/multiple, interleaved elements
- **Image detection:** All formats, case insensitivity

Run tests with:

```bash
cargo test --lib bottom_pane::text_manipulation
```

## Future Improvements

- [ ] SIMD-optimized normalization for very large pastes
- [ ] Lazy byte range rebasing to avoid reallocations
- [ ] Custom image format registry (pluggable detection)
- [ ] Stateful `TextManipulator` struct if needed for batch operations
