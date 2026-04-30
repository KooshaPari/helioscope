use super::*;

#[test]
fn code_block_known_lang_has_syntax_colors() {
    let text = render_markdown_text("```rust\nfn main() {}\n```\n");
    let content: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    // Content should be preserved; ignore trailing empty line from highlighting.
    let content: Vec<&str> = content
        .iter()
        .map(std::string::String::as_str)
        .filter(|s| !s.is_empty())
        .collect();
    assert_eq!(content, vec!["fn main() {}"]);

    // At least one span should have non-default style (syntax highlighting).
    let has_colored_span = text
        .lines
        .iter()
        .flat_map(|l| l.spans.iter())
        .any(|sp| sp.style.fg.is_some());
    assert!(
        has_colored_span,
        "expected syntax-highlighted spans with color"
    );
}

#[test]
fn code_block_unknown_lang_plain() {
    let text = render_markdown_text("```xyzlang\nhello world\n```\n");
    let content: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    let content: Vec<&str> = content
        .iter()
        .map(std::string::String::as_str)
        .filter(|s| !s.is_empty())
        .collect();
    assert_eq!(content, vec!["hello world"]);

    // No syntax coloring for unknown language — all spans have default style.
    let has_colored_span = text
        .lines
        .iter()
        .flat_map(|l| l.spans.iter())
        .any(|sp| sp.style.fg.is_some());
    assert!(
        !has_colored_span,
        "expected no syntax coloring for unknown lang"
    );
}

#[test]
fn code_block_no_lang_plain() {
    let text = render_markdown_text("```\nno lang specified\n```\n");
    let content: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    let content: Vec<&str> = content
        .iter()
        .map(std::string::String::as_str)
        .filter(|s| !s.is_empty())
        .collect();
    assert_eq!(content, vec!["no lang specified"]);
}

#[test]
fn code_block_multiple_lines_root() {
    let md = "```\nfirst\nsecond\n```\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["", "first"]),
        Line::from_iter(["", "second"]),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn code_block_indented() {
    let md = "    function greet() {\n      console.log(\"Hi\");\n    }\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["    ", "function greet() {"]),
        Line::from_iter(["    ", "  console.log(\"Hi\");"]),
        Line::from_iter(["    ", "}"]),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn code_block_with_inner_triple_backticks_outer_four() {
    let md = r#"````text
Here is a code block that shows another fenced block:

```md
# Inside fence
- bullet
- `inline code`
```
````
"#;
    let text = render_markdown_text(md);
    let lines: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    // Filter empty trailing lines for stability; the code block may or may
    // not emit a trailing blank depending on the highlighting path.
    let trimmed: Vec<&str> = {
        let mut v: Vec<&str> = lines.iter().map(std::string::String::as_str).collect();
        while v.last() == Some(&"") {
            v.pop();
        }
        v
    };
    assert_eq!(
        trimmed,
        vec![
            "Here is a code block that shows another fenced block:",
            "",
            "```md",
            "# Inside fence",
            "- bullet",
            "- `inline code`",
            "```",
        ]
    );
}

#[test]
fn code_block_inside_unordered_list_item_is_indented() {
    let md = "- Item\n\n  ```\n  code line\n  ```\n";
    let text = render_markdown_text(md);
    let lines: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    assert_eq!(lines, vec!["- Item", "", "  code line"]);
}

#[test]
fn code_block_multiple_lines_inside_unordered_list() {
    let md = "- Item\n\n  ```\n  first\n  second\n  ```\n";
    let text = render_markdown_text(md);
    let lines: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    assert_eq!(lines, vec!["- Item", "", "  first", "  second"]);
}

#[test]
fn code_block_inside_unordered_list_item_multiple_lines() {
    let md = "- Item\n\n  ```\n  first\n  second\n  ```\n";
    let text = render_markdown_text(md);
    let lines: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    assert_eq!(lines, vec!["- Item", "", "  first", "  second"]);
}

#[test]
fn code_block_preserves_trailing_blank_lines() {
    // A fenced code block with an intentional trailing blank line must keep it.
    let md = "```rust\nfn main() {}\n\n```\n";
    let text = render_markdown_text(md);
    let content: Vec<String> = text
        .lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>()
        })
        .collect();
    // Should have: "fn main() {}" then "" (the blank line).
    // Filter only to content lines (skip leading/trailing empty from rendering).
    assert!(
        content.iter().any(|c| c == "fn main() {}"),
        "expected code line, got {content:?}"
    );
    // The trailing blank line inside the fence should be preserved.
    let code_start = content.iter().position(|c| c == "fn main() {}").unwrap();
    assert!(
        content.len() > code_start + 1,
        "expected a line after 'fn main() {{}}' but content ends: {content:?}"
    );
    assert_eq!(
        content[code_start + 1],
        "",
        "trailing blank line inside code fence was lost: {content:?}"
    );
}
