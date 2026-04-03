use super::*;

impl ChatComposer {
    pub(crate) fn set_remote_image_urls(&mut self, urls: Vec<String>) {
        self.remote_image_urls = urls;
        self.selected_remote_image_index = None;
        chat_composer_images::relabel_attached_images_and_update_placeholders(
            &mut self.textarea,
            &mut self.attached_images,
            self.remote_image_urls.len(),
        );
        self.sync_popups();
    }

    pub(crate) fn remote_image_urls(&self) -> Vec<String> {
        self.remote_image_urls.clone()
    }

    pub(crate) fn take_remote_image_urls(&mut self) -> Vec<String> {
        let urls = std::mem::take(&mut self.remote_image_urls);
        self.selected_remote_image_index = None;
        chat_composer_images::relabel_attached_images_and_update_placeholders(
            &mut self.textarea,
            &mut self.attached_images,
            self.remote_image_urls.len(),
        );
        self.sync_popups();
        urls
    }

    pub(crate) fn remote_images_lines(&self, _width: u16) -> Vec<Line<'static>> {
        chat_composer_images::remote_images_lines(
            &self.remote_image_urls,
            self.selected_remote_image_index,
        )
    }

    fn clear_remote_image_selection(&mut self) {
        self.selected_remote_image_index = None;
    }

    fn remove_selected_remote_image(&mut self, selected_index: usize) {
        chat_composer_images::remove_selected_remote_image(
            &mut self.remote_image_urls,
            &mut self.selected_remote_image_index,
            selected_index,
        );
        chat_composer_images::relabel_attached_images_and_update_placeholders(
            &mut self.textarea,
            &mut self.attached_images,
            self.remote_image_urls.len(),
        );
        self.sync_popups();
    }

    pub(crate) fn handle_remote_image_selection_key(
        &mut self,
        key_event: &KeyEvent,
    ) -> Option<(InputResult, bool)> {
        if self.remote_image_urls.is_empty()
            || key_event.modifiers != KeyModifiers::NONE
            || key_event.kind != KeyEventKind::Press
        {
            return None;
        }

        match key_event.code {
            KeyCode::Up => {
                if let Some(selected) = self.selected_remote_image_index {
                    self.selected_remote_image_index = Some(selected.saturating_sub(1));
                    Some((InputResult::None, true))
                } else if self.textarea.cursor() == 0 {
                    self.selected_remote_image_index = Some(self.remote_image_urls.len() - 1);
                    Some((InputResult::None, true))
                } else {
                    None
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.selected_remote_image_index {
                    if selected + 1 < self.remote_image_urls.len() {
                        self.selected_remote_image_index = Some(selected + 1);
                    } else {
                        self.clear_remote_image_selection();
                    }
                    Some((InputResult::None, true))
                } else {
                    None
                }
            }
            KeyCode::Delete | KeyCode::Backspace => {
                if let Some(selected) = self.selected_remote_image_index {
                    self.remove_selected_remote_image(selected);
                    Some((InputResult::None, true))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
