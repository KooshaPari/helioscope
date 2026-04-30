use super::*;

#[test]
fn empty() {
    assert_eq!(render_markdown_text(""), Text::default());
}

#[test]
fn paragraph_single() {
    assert_eq!(
        render_markdown_text("Hello, world!"),
        Text::from("Hello, world!")
    );
}

#[test]
fn paragraph_soft_break() {
    assert_eq!(
        render_markdown_text("Hello\nWorld"),
        Text::from_iter(["Hello", "World"])
    );
}

#[test]
fn paragraph_multiple() {
    assert_eq!(
        render_markdown_text("Paragraph 1\n\nParagraph 2"),
        Text::from_iter(["Paragraph 1", "", "Paragraph 2"])
    );
}

#[test]
fn headings() {
    let md = "# Heading 1\n## Heading 2\n### Heading 3\n#### Heading 4\n##### Heading 5\n###### Heading 6\n";
    let text = render_markdown_text(md);
    let expected = Text::from_iter([
        Line::from_iter(["# ".bold().underlined(), "Heading 1".bold().underlined()]),
        Line::default(),
        Line::from_iter(["## ".bold(), "Heading 2".bold()]),
        Line::default(),
        Line::from_iter(["### ".bold().italic(), "Heading 3".bold().italic()]),
        Line::default(),
        Line::from_iter(["#### ".italic(), "Heading 4".italic()]),
        Line::default(),
        Line::from_iter(["##### ".italic(), "Heading 5".italic()]),
        Line::default(),
        Line::from_iter(["###### ".italic(), "Heading 6".italic()]),
    ]);
    assert_eq!(text, expected);
}
