use super::ModelMigrationCopy;
use super::ModelMigrationOutcome;
use super::ModelMigrationScreen;
use super::migration_copy_for_models;
use crate::custom_terminal::Terminal;
use crate::test_backend::VT100Backend;
use crate::tui::FrameRequester;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use insta::assert_snapshot;
use ratatui::layout::Rect;

#[test]
fn prompt_snapshot() {
    let width: u16 = 60;
    let height: u16 = 28;
    let backend = VT100Backend::new(width, height);
    let mut terminal = Terminal::with_options(backend).expect("terminal");
    terminal.set_viewport_area(Rect::new(0, 0, width, height));

    let screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-5.1-codex-mini",
            "gpt-5.1-codex-max",
            None,
            Some(
                "Upgrade to gpt-5.2-codex for the latest and greatest agentic coding model."
                    .to_string(),
            ),
            None,
            "gpt-5.1-codex-max".to_string(),
            Some("Codex-optimized flagship for deep and fast reasoning.".to_string()),
            true,
        ),
    );

    {
        let mut frame = terminal.get_frame();
        frame.render_widget_ref(&screen, frame.area());
    }
    terminal.flush().expect("flush");

    assert_snapshot!("model_migration_prompt", terminal.backend());
}

#[test]
fn prompt_snapshot_gpt5_family() {
    let backend = VT100Backend::new(65, 22);
    let mut terminal = Terminal::with_options(backend).expect("terminal");
    terminal.set_viewport_area(Rect::new(0, 0, 65, 22));

    let screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-5",
            "gpt-5.1",
            Some("https://www.codex.com/models/gpt-5.1".to_string()),
            None,
            None,
            "gpt-5.1".to_string(),
            Some("Broad world knowledge with strong general reasoning.".to_string()),
            false,
        ),
    );
    {
        let mut frame = terminal.get_frame();
        frame.render_widget_ref(&screen, frame.area());
    }
    terminal.flush().expect("flush");
    assert_snapshot!("model_migration_prompt_gpt5_family", terminal.backend());
}

#[test]
fn prompt_snapshot_gpt5_codex() {
    let backend = VT100Backend::new(60, 22);
    let mut terminal = Terminal::with_options(backend).expect("terminal");
    terminal.set_viewport_area(Rect::new(0, 0, 60, 22));

    let screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-5-codex",
            "gpt-5.1-codex-max",
            Some("https://www.codex.com/models/gpt-5.1-codex-max".to_string()),
            None,
            None,
            "gpt-5.1-codex-max".to_string(),
            Some("Codex-optimized flagship for deep and fast reasoning.".to_string()),
            false,
        ),
    );
    {
        let mut frame = terminal.get_frame();
        frame.render_widget_ref(&screen, frame.area());
    }
    terminal.flush().expect("flush");
    assert_snapshot!("model_migration_prompt_gpt5_codex", terminal.backend());
}

#[test]
fn prompt_snapshot_gpt5_codex_mini() {
    let backend = VT100Backend::new(60, 22);
    let mut terminal = Terminal::with_options(backend).expect("terminal");
    terminal.set_viewport_area(Rect::new(0, 0, 60, 22));

    let screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-5-codex-mini",
            "gpt-5.1-codex-mini",
            Some("https://www.codex.com/models/gpt-5.1-codex-mini".to_string()),
            None,
            None,
            "gpt-5.1-codex-mini".to_string(),
            Some("Optimized for codex. Cheaper, faster, but less capable.".to_string()),
            false,
        ),
    );
    {
        let mut frame = terminal.get_frame();
        frame.render_widget_ref(&screen, frame.area());
    }
    terminal.flush().expect("flush");
    assert_snapshot!("model_migration_prompt_gpt5_codex_mini", terminal.backend());
}

#[test]
fn escape_key_accepts_prompt() {
    let mut screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-old",
            "gpt-new",
            Some("https://www.codex.com/models/gpt-new".to_string()),
            None,
            None,
            "gpt-new".to_string(),
            Some("Latest recommended model for better performance.".to_string()),
            true,
        ),
    );

    screen.handle_key(KeyEvent::new(
        KeyCode::Esc,
        crossterm::event::KeyModifiers::NONE,
    ));
    assert!(screen.is_done());
    assert!(matches!(screen.outcome(), ModelMigrationOutcome::Accepted));
}

#[test]
fn selecting_use_existing_model_rejects_upgrade() {
    let mut screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        migration_copy_for_models(
            "gpt-old",
            "gpt-new",
            Some("https://www.codex.com/models/gpt-new".to_string()),
            None,
            None,
            "gpt-new".to_string(),
            Some("Latest recommended model for better performance.".to_string()),
            true,
        ),
    );

    screen.handle_key(KeyEvent::new(
        KeyCode::Down,
        crossterm::event::KeyModifiers::NONE,
    ));
    screen.handle_key(KeyEvent::new(
        KeyCode::Enter,
        crossterm::event::KeyModifiers::NONE,
    ));

    assert!(screen.is_done());
    assert!(matches!(screen.outcome(), ModelMigrationOutcome::Rejected));
}

#[test]
fn markdown_prompt_keeps_long_url_tail_visible_when_narrow() {
    let long_url = "https://example.test/api/v1/projects/alpha-team/releases/2026-02-17/builds/1234567890/artifacts/reports/performance/summary/detail/with/a/very/long/path/tail42";
    let screen = ModelMigrationScreen::new(
        FrameRequester::test_dummy(),
        ModelMigrationCopy {
            heading: Vec::new(),
            content: Vec::new(),
            can_opt_out: false,
            markdown: Some(long_url.to_string()),
        },
    );

    let backend = VT100Backend::new(40, 16);
    let mut terminal = Terminal::with_options(backend).expect("terminal");
    terminal.set_viewport_area(Rect::new(0, 0, 40, 16));

    {
        let mut frame = terminal.get_frame();
        frame.render_widget_ref(&screen, frame.area());
    }
    terminal.flush().expect("flush");

    let rendered = terminal.backend().to_string();
    assert!(
        rendered.contains("tail42"),
        "expected wrapped markdown URL tail to remain visible, got:\n{rendered}"
    );
}
