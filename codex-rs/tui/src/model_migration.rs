use crate::key_hint;
use crate::markdown_render::render_markdown_text_with_width;
use crate::render::Insets;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use crate::render::renderable::RenderableExt as _;
use crate::selection_list::selection_option_row;
use crate::tui::FrameRequester;
use crate::tui::Tui;
use crate::tui::TuiEvent;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use ratatui::prelude::Stylize as _;
use ratatui::prelude::Widget;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::WidgetRef;
use ratatui::widgets::Wrap;
use tokio_stream::StreamExt;

mod copy;
pub(crate) use copy::ModelMigrationCopy;
pub(crate) use copy::migration_copy_for_models;

/// Outcome of the migration prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModelMigrationOutcome {
    Accepted,
    Rejected,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MigrationMenuOption {
    TryNewModel,
    UseExistingModel,
}

impl MigrationMenuOption {
    fn all() -> [Self; 2] {
        [Self::TryNewModel, Self::UseExistingModel]
    }

    fn label(self) -> &'static str {
        match self {
            Self::TryNewModel => "Try new model",
            Self::UseExistingModel => "Use existing model",
        }
    }
}

pub(crate) async fn run_model_migration_prompt(
    tui: &mut Tui,
    copy: ModelMigrationCopy,
) -> ModelMigrationOutcome {
    let alt = AltScreenGuard::enter(tui);
    let mut screen = ModelMigrationScreen::new(alt.tui.frame_requester(), copy);

    let _ = alt.tui.draw(u16::MAX, |frame| {
        frame.render_widget_ref(&screen, frame.area());
    });

    let events = alt.tui.event_stream();
    tokio::pin!(events);

    while !screen.is_done() {
        if let Some(event) = events.next().await {
            match event {
                TuiEvent::Key(key_event) => screen.handle_key(key_event),
                TuiEvent::Paste(_) => {}
                TuiEvent::Draw => {
                    let _ = alt.tui.draw(u16::MAX, |frame| {
                        frame.render_widget_ref(&screen, frame.area());
                    });
                }
            }
        } else {
            screen.accept();
            break;
        }
    }

    screen.outcome()
}

struct ModelMigrationScreen {
    request_frame: FrameRequester,
    copy: ModelMigrationCopy,
    done: bool,
    outcome: ModelMigrationOutcome,
    highlighted_option: MigrationMenuOption,
}

impl ModelMigrationScreen {
    fn new(request_frame: FrameRequester, copy: ModelMigrationCopy) -> Self {
        Self {
            request_frame,
            copy,
            done: false,
            outcome: ModelMigrationOutcome::Accepted,
            highlighted_option: MigrationMenuOption::TryNewModel,
        }
    }

    fn finish_with(&mut self, outcome: ModelMigrationOutcome) {
        self.outcome = outcome;
        self.done = true;
        self.request_frame.schedule_frame();
    }

    fn accept(&mut self) {
        self.finish_with(ModelMigrationOutcome::Accepted);
    }

    fn reject(&mut self) {
        self.finish_with(ModelMigrationOutcome::Rejected);
    }

    fn exit(&mut self) {
        self.finish_with(ModelMigrationOutcome::Exit);
    }

    fn confirm_selection(&mut self) {
        if self.copy.can_opt_out {
            match self.highlighted_option {
                MigrationMenuOption::TryNewModel => self.accept(),
                MigrationMenuOption::UseExistingModel => self.reject(),
            }
        } else {
            self.accept();
        }
    }

    fn highlight_option(&mut self, option: MigrationMenuOption) {
        if self.highlighted_option != option {
            self.highlighted_option = option;
            self.request_frame.schedule_frame();
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        if is_ctrl_exit_combo(key_event) {
            self.exit();
            return;
        }

        if self.copy.can_opt_out {
            self.handle_menu_key(key_event.code);
        } else if matches!(key_event.code, KeyCode::Esc | KeyCode::Enter) {
            self.accept();
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn outcome(&self) -> ModelMigrationOutcome {
        self.outcome
    }
}

impl WidgetRef for &ModelMigrationScreen {
    fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);

        let mut column = ColumnRenderable::new();
        column.push("");
        if let Some(markdown) = self.copy.markdown.as_ref() {
            self.render_markdown_content(markdown, area.width, &mut column);
        } else {
            column.push(self.heading_line());
            column.push(Line::from(""));
            self.render_content(&mut column);
        }
        if self.copy.can_opt_out {
            self.render_menu(&mut column);
        }

        column.render(area, buf);
    }
}

impl ModelMigrationScreen {
    fn handle_menu_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.highlight_option(MigrationMenuOption::TryNewModel);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.highlight_option(MigrationMenuOption::UseExistingModel);
            }
            KeyCode::Char('1') => {
                self.highlight_option(MigrationMenuOption::TryNewModel);
                self.accept();
            }
            KeyCode::Char('2') => {
                self.highlight_option(MigrationMenuOption::UseExistingModel);
                self.reject();
            }
            KeyCode::Enter | KeyCode::Esc => self.confirm_selection(),
            _ => {}
        }
    }

    fn heading_line(&self) -> Line<'static> {
        let mut heading = vec![Span::raw("> ")];
        heading.extend(self.copy.heading.iter().cloned());
        Line::from(heading)
    }

    fn render_content(&self, column: &mut ColumnRenderable) {
        self.render_lines(&self.copy.content, column);
    }

    fn render_lines(&self, lines: &[Line<'static>], column: &mut ColumnRenderable) {
        for line in lines {
            column.push(
                Paragraph::new(line.clone())
                    .wrap(Wrap { trim: false })
                    .inset(Insets::tlbr(0, 2, 0, 0)),
            );
        }
    }

    fn render_markdown_content(
        &self,
        markdown: &str,
        area_width: u16,
        column: &mut ColumnRenderable,
    ) {
        let horizontal_inset = 2;
        let content_width = area_width.saturating_sub(horizontal_inset);
        let wrap_width = (content_width > 0).then_some(content_width as usize);
        let rendered = render_markdown_text_with_width(markdown, wrap_width);
        for line in rendered.lines {
            column.push(
                Paragraph::new(line)
                    .wrap(Wrap { trim: false })
                    .inset(Insets::tlbr(0, horizontal_inset, 0, 0)),
            );
        }
    }

    fn render_menu(&self, column: &mut ColumnRenderable) {
        column.push(Line::from(""));
        column.push(
            Paragraph::new("Choose how you'd like Codex to proceed.")
                .wrap(Wrap { trim: false })
                .inset(Insets::tlbr(0, 2, 0, 0)),
        );
        column.push(Line::from(""));

        for (idx, option) in MigrationMenuOption::all().into_iter().enumerate() {
            column.push(selection_option_row(
                idx,
                option.label().to_string(),
                self.highlighted_option == option,
            ));
        }

        column.push(Line::from(""));
        column.push(
            Line::from(vec![
                "Use ".dim(),
                key_hint::plain(KeyCode::Up).into(),
                "/".dim(),
                key_hint::plain(KeyCode::Down).into(),
                " to move, press ".dim(),
                key_hint::plain(KeyCode::Enter).into(),
                " to confirm".dim(),
            ])
            .inset(Insets::tlbr(0, 2, 0, 0)),
        );
    }
}

// Render the prompt on the terminal's alternate screen so exiting or cancelling
// does not leave a large blank region in the normal scrollback. This does not
// change the prompt's appearance – only where it is drawn.
struct AltScreenGuard<'a> {
    tui: &'a mut Tui,
}

impl<'a> AltScreenGuard<'a> {
    fn enter(tui: &'a mut Tui) -> Self {
        let _ = tui.enter_alt_screen();
        Self { tui }
    }
}

impl Drop for AltScreenGuard<'_> {
    fn drop(&mut self) {
        let _ = self.tui.leave_alt_screen();
    }
}

fn is_ctrl_exit_combo(key_event: KeyEvent) -> bool {
    key_event.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key_event.code, KeyCode::Char('c') | KeyCode::Char('d'))
}

#[cfg(test)]
mod tests;
