use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;

use crate::markdown_render::COLON_LOCATION_SUFFIX_RE;
use crate::markdown_render::HASH_LOCATION_SUFFIX_RE;
use crate::markdown_render::render_markdown_text;
use insta::assert_snapshot;

#[path = "markdown_render_tests/basics.rs"]
mod basics;
#[path = "markdown_render_tests/blockquote_lists.rs"]
mod blockquote_lists;
#[path = "markdown_render_tests/blockquotes.rs"]
mod blockquotes;
#[path = "markdown_render_tests/code_blocks.rs"]
mod code_blocks;
#[path = "markdown_render_tests/html_rules.rs"]
mod html_rules;
#[path = "markdown_render_tests/inline_links.rs"]
mod inline_links;
#[path = "markdown_render_tests/lists.rs"]
mod lists;

#[test]
fn markdown_render_file_link_snapshot() {
    let text = render_markdown_text(
        "See [markdown_render.rs:74](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs:74).",
    );
    let rendered = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered);
}

#[test]
fn markdown_render_complex_snapshot() {
    let md = r#"# H1: Markdown Streaming Test
Intro paragraph with bold **text**, italic *text*, and inline code `x=1`.
Combined bold-italic ***both*** and escaped asterisks \*literal\*.
Auto-link: <https://example.com> and reference link [ref][r1].
Link with title: [hover me](https://example.com "Example") and mailto <mailto:test@example.com>.
Image: ![alt text](https://example.com/img.png "Title")
> Blockquote level 1
>> Blockquote level 2 with `inline code`
- Unordered list item 1
  - Nested bullet with italics _inner_
- Unordered list item 2 with ~~strikethrough~~
1. Ordered item one
2. Ordered item two with sublist:
   1) Alt-numbered subitem
- [ ] Task: unchecked
- [x] Task: checked with link [home](https://example.org)
---
Table below (alignment test):
| Left | Center | Right |
|:-----|:------:|------:|
| a    |   b    |     c |
Inline HTML: <sup>sup</sup> and <sub>sub</sub>.
HTML block:
<div style="border:1px solid #ccc;padding:2px">inline block</div>
Escapes: \_underscores\_, backslash \\, ticks ``code with `backtick` inside``.
Emoji shortcodes: :sparkles: :tada: (if supported).
Hard break test (line ends with two spaces)  
Next line should be close to previous.
Footnote reference here[^1] and another[^longnote].
Horizontal rule with asterisks:
***
Fenced code block (JSON):
```json
{ "a": 1, "b": [true, false] }
```
Fenced code with tildes and triple backticks inside:
~~~markdown
To close ``` you need tildes.
~~~
Indented code block:
    for i in range(3): print(i)
Definition-like list:
Term
: Definition with `code`.
Character entities: &amp; &lt; &gt; &quot; &#39;
[^1]: This is the first footnote.
[^longnote]: A longer footnote with a link to [Rust](https://www.rust-lang.org/).
Escaped pipe in text: a \| b \| c.
URL with parentheses: [link](https://example.com/path_(with)_parens).
[r1]: https://example.com/ref "Reference link title"
"#;

    let text = render_markdown_text(md);
    // Convert to plain text lines for snapshot (ignore styles)
    let rendered = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered);
}
