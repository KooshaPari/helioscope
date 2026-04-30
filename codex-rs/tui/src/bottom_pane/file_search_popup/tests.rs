use super::*;
use pretty_assertions::assert_eq;

fn file_match(path: &str, indices: Option<Vec<u32>>) -> FileMatch {
    FileMatch {
        score: 42,
        path: PathBuf::from(path),
        root: PathBuf::from("/repo"),
        indices,
    }
}

fn render_lines(popup: &FileSearchPopup, width: u16) -> String {
    let height = popup.calculate_required_height();
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    popup.render_ref(area, &mut buf);

    (0..area.height)
        .map(|row| {
            let mut line = String::new();
            for col in 0..area.width {
                let symbol = buf[(col, row)].symbol();
                if symbol.is_empty() {
                    line.push(' ');
                } else {
                    line.push_str(symbol);
                }
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn required_height_includes_shared_menu_surface_padding() {
    let mut popup = FileSearchPopup::new();
    assert_eq!(popup.calculate_required_height(), 3);

    popup.set_query("src");
    popup.set_matches(
        "src",
        vec![
            file_match("src/main.rs", Some(vec![0, 1, 2])),
            file_match("src/lib.rs", None),
        ],
    );

    assert_eq!(popup.calculate_required_height(), 4);
}

#[test]
fn renders_file_matches_inside_shared_menu_surface() {
    let mut popup = FileSearchPopup::new();
    popup.set_query("src");
    popup.set_matches("src", vec![file_match("src/main.rs", Some(vec![0, 1, 2]))]);

    let rendered = render_lines(&popup, 32);
    let first_content_line = rendered
        .lines()
        .nth(1)
        .expect("menu surface should reserve a top padding line");

    assert!(
        first_content_line.starts_with("  src/main.rs"),
        "expected file match to render inside the shared horizontal inset: {rendered:?}",
    );
}

#[test]
fn ignores_stale_search_results() {
    let mut popup = FileSearchPopup::new();
    popup.set_query("src");
    popup.set_matches("other", vec![file_match("other.rs", None)]);

    assert!(popup.selected_match().is_none());
}
