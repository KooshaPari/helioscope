use super::*;

#[test]
fn blockquote_single() {
    let text = render_markdown_text("> Blockquote");
    let expected = Text::from(Line::from_iter(["> ", "Blockquote"]).green());
    assert_eq!(text, expected);
}

#[test]
fn blockquote_soft_break() {
    // Soft break via lazy continuation should render as a new line in blockquotes.
    let text = render_markdown_text("> This is a blockquote\nwith a soft break\n");
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
            "> This is a blockquote".to_string(),
            "> with a soft break".to_string()
        ]
    );
}

#[test]
fn blockquote_multiple_with_break() {
    let text = render_markdown_text("> Blockquote 1\n\n> Blockquote 2\n");
    let expected = Text::from_iter([
        Line::from_iter(["> ", "Blockquote 1"]).green(),
        Line::default(),
        Line::from_iter(["> ", "Blockquote 2"]).green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn blockquote_three_paragraphs_short_lines() {
    let md = "> one\n>\n> two\n>\n> three\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["> ", "one"]).green(),
        Line::from_iter(["> "]).green(),
        Line::from_iter(["> ", "two"]).green(),
        Line::from_iter(["> "]).green(),
        Line::from_iter(["> ", "three"]).green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn blockquote_nested_two_levels() {
    let md = "> Level 1\n>> Level 2\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["> ", "Level 1"]).green(),
        Line::from_iter(["> "]).green(),
        Line::from_iter(["> ", "> ", "Level 2"]).green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn blockquote_with_list_items() {
    let md = "> - item 1\n> - item 2\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["> ", "- ", "item 1"]).green(),
        Line::from_iter(["> ", "- ", "item 2"]).green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn blockquote_with_ordered_list() {
    let md = "> 1. first\n> 2. second\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(vec![
            Span::from("> "),
            "1. ".light_blue(),
            Span::from("first"),
        ])
        .green(),
        Line::from_iter(vec![
            Span::from("> "),
            "2. ".light_blue(),
            Span::from("second"),
        ])
        .green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn blockquote_list_then_nested_blockquote() {
    let md = "> - parent\n>   > child\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["> ", "- ", "parent"]).green(),
        Line::from_iter(["> ", "  ", "> ", "child"]).green(),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn list_item_with_inline_blockquote_on_same_line() {
    let md = "1. > quoted\n";
    let text = render_markdown_text(md);
    let mut lines = text.lines.iter();
    let first = lines.next().expect("one line");
    // Expect content to include the ordered marker, a space, "> ", and the text
    let s: String = first.spans.iter().map(|sp| sp.content.clone()).collect();
    assert_eq!(s, "1. > quoted");
}

#[test]
fn blockquote_surrounded_by_blank_lines() {
    let md = "foo\n\n> bar\n\nbaz\n";
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
            "foo".to_string(),
            "".to_string(),
            "> bar".to_string(),
            "".to_string(),
            "baz".to_string(),
        ]
    );
}

#[test]
fn blockquote_with_heading_and_paragraph() {
    let md = "> # Heading\n> paragraph text\n";
    let text = render_markdown_text(md);
    // Validate on content shape; styling is handled elsewhere
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
            "> # Heading".to_string(),
            "> ".to_string(),
            "> paragraph text".to_string(),
        ]
    );
}

#[test]
fn blockquote_heading_inherits_heading_style() {
    let text = render_markdown_text("> # test header\n> in blockquote\n");
    assert_eq!(
        text.lines,
        [
            Line::from_iter([
                "> ".into(),
                "# ".bold().underlined(),
                "test header".bold().underlined(),
            ])
            .green(),
            Line::from_iter(["> "]).green(),
            Line::from_iter(["> ", "in blockquote"]).green(),
        ]
    );
}
