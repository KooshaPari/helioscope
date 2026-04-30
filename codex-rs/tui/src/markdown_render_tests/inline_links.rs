use super::*;

#[test]
fn inline_code() {
    let text = render_markdown_text("Example of `Inline code`");
    let expected = Line::from_iter(["Example of ".into(), "Inline code".cyan()]).into();
    assert_eq!(text, expected);
}

#[test]
fn strong() {
    assert_eq!(
        render_markdown_text("**Strong**"),
        Text::from(Line::from("Strong".bold()))
    );
}

#[test]
fn emphasis() {
    assert_eq!(
        render_markdown_text("*Emphasis*"),
        Text::from(Line::from("Emphasis".italic()))
    );
}

#[test]
fn strikethrough() {
    assert_eq!(
        render_markdown_text("~~Strikethrough~~"),
        Text::from(Line::from("Strikethrough".crossed_out()))
    );
}

#[test]
fn strong_emphasis() {
    let text = render_markdown_text("**Strong *emphasis***");
    let expected = Text::from(Line::from_iter([
        "Strong ".bold(),
        "emphasis".bold().italic(),
    ]));
    assert_eq!(text, expected);
}

#[test]
fn link() {
    let text = render_markdown_text("[Link](https://example.com)");
    let expected = Text::from(Line::from_iter([
        "Link".into(),
        " (".into(),
        "https://example.com".cyan().underlined(),
        ")".into(),
    ]));
    assert_eq!(text, expected);
}

#[test]
fn load_location_suffix_regexes() {
    let _colon = &*COLON_LOCATION_SUFFIX_RE;
    let _hash = &*HASH_LOCATION_SUFFIX_RE;
}

#[test]
fn file_link_hides_destination() {
    let text = render_markdown_text(
        "[codex-rs/tui/src/markdown_render.rs](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs)",
    );
    let expected = Text::from(Line::from_iter([
        "codex-rs/tui/src/markdown_render.rs".cyan()
    ]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_appends_line_number_when_label_lacks_it() {
    let text = render_markdown_text(
        "[markdown_render.rs](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs:74)",
    );
    let expected = Text::from(Line::from_iter(["markdown_render.rs".cyan(), ":74".cyan()]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_uses_label_for_line_number() {
    let text = render_markdown_text(
        "[markdown_render.rs:74](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs:74)",
    );
    let expected = Text::from(Line::from_iter(["markdown_render.rs:74".cyan()]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_appends_hash_anchor_when_label_lacks_it() {
    let text = render_markdown_text(
        "[markdown_render.rs](file:///Users/example/code/codex/codex-rs/tui/src/markdown_render.rs#L74C3)",
    );
    let expected = Text::from(Line::from_iter([
        "markdown_render.rs".cyan(),
        ":74:3".cyan(),
    ]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_uses_label_for_hash_anchor() {
    let text = render_markdown_text(
        "[markdown_render.rs#L74C3](file:///Users/example/code/codex/codex-rs/tui/src/markdown_render.rs#L74C3)",
    );
    let expected = Text::from(Line::from_iter(["markdown_render.rs#L74C3".cyan()]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_appends_range_when_label_lacks_it() {
    let text = render_markdown_text(
        "[markdown_render.rs](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs:74:3-76:9)",
    );
    let expected = Text::from(Line::from_iter([
        "markdown_render.rs".cyan(),
        ":74:3-76:9".cyan(),
    ]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_uses_label_for_range() {
    let text = render_markdown_text(
        "[markdown_render.rs:74:3-76:9](/Users/example/code/codex/codex-rs/tui/src/markdown_render.rs:74:3-76:9)",
    );
    let expected = Text::from(Line::from_iter(["markdown_render.rs:74:3-76:9".cyan()]));
    assert_eq!(text, expected);
}

#[test]
fn file_link_appends_hash_range_when_label_lacks_it() {
    let text = render_markdown_text(
        "[markdown_render.rs](file:///Users/example/code/codex/codex-rs/tui/src/markdown_render.rs#L74C3-L76C9)",
    );
    let expected = Text::from(Line::from_iter([
        "markdown_render.rs".cyan(),
        ":74:3-76:9".cyan(),
    ]));
    assert_eq!(text, expected);
}

#[test]
fn multiline_file_link_label_after_styled_prefix_does_not_panic() {
    let text = render_markdown_text(
        "**bold** plain [foo\nbar](file:///Users/example/code/codex/codex-rs/tui/src/markdown_render.rs#L74C3)",
    );
    let expected = Text::from_iter([
        Line::from_iter(["bold".bold(), " plain ".into(), "foo".cyan()]),
        Line::from_iter(["bar".cyan(), ":74:3".cyan()]),
    ]);
    assert_eq!(text, expected);
}

#[test]
fn file_link_uses_label_for_hash_range() {
    let text = render_markdown_text(
        "[markdown_render.rs#L74C3-L76C9](file:///Users/example/code/codex/codex-rs/tui/src/markdown_render.rs#L74C3-L76C9)",
    );
    let expected = Text::from(Line::from_iter(["markdown_render.rs#L74C3-L76C9".cyan()]));
    assert_eq!(text, expected);
}

#[test]
fn url_link_shows_destination() {
    let text = render_markdown_text("[docs](https://example.com/docs)");
    let expected = Text::from(Line::from_iter([
        "docs".into(),
        " (".into(),
        "https://example.com/docs".cyan().underlined(),
        ")".into(),
    ]));
    assert_eq!(text, expected);
}
