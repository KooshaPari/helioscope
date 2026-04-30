use super::*;

impl BottomPaneView for ListSelectionView {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event {
            // Some terminals (or configurations) send Control key chords as
            // C0 control characters without reporting the CONTROL modifier.
            // Handle fallbacks for Ctrl-P/N here so navigation works everywhere.
            KeyEvent {
                code: KeyCode::Up, ..
            }
            | KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('\u{0010}'),
                modifiers: KeyModifiers::NONE,
                ..
            } /* ^P */ => self.move_up(),
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } if !self.is_searchable => self.move_up(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('n'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('\u{000e}'),
                modifiers: KeyModifiers::NONE,
                ..
            } /* ^N */ => self.move_down(),
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } if !self.is_searchable => self.move_down(),
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } if self.is_searchable => {
                self.search_query.pop();
                self.apply_filter();
            }
            KeyEvent {
                code: KeyCode::Esc, ..
            } => {
                self.on_ctrl_c();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            } if self.is_searchable
                && !modifiers.contains(KeyModifiers::CONTROL)
                && !modifiers.contains(KeyModifiers::ALT) =>
            {
                self.search_query.push(c);
                self.apply_filter();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            } if !self.is_searchable
                && !modifiers.contains(KeyModifiers::CONTROL)
                && !modifiers.contains(KeyModifiers::ALT) =>
            {
                if let Some(idx) = c
                    .to_digit(10)
                    .map(|d| d as usize)
                    .and_then(|d| d.checked_sub(1))
                    && idx < self.items.len()
                    && self
                        .items
                        .get(idx)
                        .is_some_and(|item| item.disabled_reason.is_none() && !item.is_disabled)
                {
                    self.state.selected_idx = Some(idx);
                    self.accept();
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.accept(),
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn view_id(&self) -> Option<&'static str> {
        self.view_id
    }

    fn on_ctrl_c(&mut self) -> CancellationEvent {
        if let Some(cb) = &self.on_cancel {
            cb(&self.app_event_tx);
        }
        self.complete = true;
        CancellationEvent::Handled
    }
}
