use super::*;

#[test]
fn blockquote_in_ordered_list_on_next_line() {
    // Blockquote begins on a new line within an ordered list item; it should
    // render inline on the same marker line.
    let md = "1.\n   > quoted\n";
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
    assert_eq!(lines, vec!["1. > quoted".to_string()]);
}

#[test]
fn blockquote_in_unordered_list_on_next_line() {
    // Blockquote begins on a new line within an unordered list item; it should
    // render inline on the same marker line.
    let md = "-\n  > quoted\n";
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
    assert_eq!(lines, vec!["- > quoted".to_string()]);
}

#[test]
fn blockquote_two_paragraphs_inside_ordered_list_has_blank_line() {
    // Two blockquote paragraphs inside a list item should be separated by a blank line.
    let md = "1.\n   > para 1\n   >\n   > para 2\n";
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
    assert_eq!(
        lines,
        vec![
            "1. > para 1".to_string(),
            "   > ".to_string(),
            "   > para 2".to_string(),
        ],
        "expected blockquote content to stay aligned after list marker"
    );
}

#[test]
fn blockquote_inside_nested_list() {
    let md = "1. A\n    - B\n      > inner\n";
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
    assert_eq!(lines, vec!["1. A", "    - B", "      > inner"]);
}

#[test]
fn list_item_text_then_blockquote() {
    let md = "1. before\n   > quoted\n";
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
    assert_eq!(lines, vec!["1. before", "   > quoted"]);
}

#[test]
fn list_item_blockquote_then_text() {
    let md = "1.\n   > quoted\n   after\n";
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
    assert_eq!(lines, vec!["1. > quoted", "   > after"]);
}

#[test]
fn list_item_text_blockquote_text() {
    let md = "1. before\n   > quoted\n   after\n";
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
    assert_eq!(lines, vec!["1. before", "   > quoted", "   > after"]);
}

#[test]
fn blockquote_with_code_block() {
    let md = "> ```\n> code\n> ```\n";
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
    assert_eq!(lines, vec!["> code".to_string()]);
}

#[test]
fn blockquote_with_multiline_code_block() {
    let md = "> ```\n> first\n> second\n> ```\n";
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
    assert_eq!(lines, vec!["> first", "> second"]);
}

#[test]
fn nested_blockquote_with_inline_and_fenced_code() {
    /*
    let md = \"> Nested quote with code:\n\
    > > Inner quote and `inline code`\n\
    > >\n\
    > > ```\n\
    > > # fenced code inside a quote\n\
    > > echo \"hello from a quote\"\n\
    > > ```\n";
    */
    let md = r#"> Nested quote with code:
> > Inner quote and `inline code`
> >
> > ```
> > # fenced code inside a quote
> > echo "hello from a quote"
> > ```
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
    assert_eq!(
        lines,
        vec![
            "> Nested quote with code:".to_string(),
            "> ".to_string(),
            "> > Inner quote and inline code".to_string(),
            "> > ".to_string(),
            "> > # fenced code inside a quote".to_string(),
            "> > echo \"hello from a quote\"".to_string(),
        ]
    );
}
