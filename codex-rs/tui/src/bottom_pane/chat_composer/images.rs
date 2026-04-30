use std::path::PathBuf;

use ratatui::text::Line;

use crate::bottom_pane::LocalImageAttachment;

use super::ChatComposer;
use super::chat_composer_images;

impl ChatComposer {
    #[cfg(test)]
    pub(crate) fn local_image_paths(&self) -> Vec<PathBuf> {
        chat_composer_images::local_image_paths(&self.attached_images)
    }

    pub(crate) fn local_images(&self) -> Vec<LocalImageAttachment> {
        chat_composer_images::local_images(&self.attached_images)
    }

    /// Insert an attachment placeholder and track it for the next submission.
    pub fn attach_image(&mut self, path: PathBuf) {
        chat_composer_images::attach_image(
            &mut self.textarea,
            &mut self.attached_images,
            self.remote_image_urls.len(),
            path,
        );
    }

    #[cfg(test)]
    pub fn take_recent_submission_images(&mut self) -> Vec<PathBuf> {
        chat_composer_images::take_recent_submission_images(&mut self.attached_images)
    }

    pub fn take_recent_submission_images_with_placeholders(&mut self) -> Vec<LocalImageAttachment> {
        chat_composer_images::take_recent_submission_images_with_placeholders(
            &mut self.attached_images,
        )
    }

    pub(in crate::bottom_pane) fn remote_images_lines(&self, _width: u16) -> Vec<Line<'static>> {
        chat_composer_images::remote_images_lines(
            &self.remote_image_urls,
            self.selected_remote_image_index,
        )
    }

    pub(super) fn remove_selected_remote_image(&mut self, selected_index: usize) {
        chat_composer_images::remove_selected_remote_image(
            &mut self.remote_image_urls,
            &mut self.selected_remote_image_index,
            selected_index,
        );
        self.relabel_attached_images_and_update_placeholders();
        self.sync_popups();
    }

    pub(super) fn relabel_attached_images_and_update_placeholders(&mut self) {
        chat_composer_images::relabel_attached_images_and_update_placeholders(
            &mut self.textarea,
            &mut self.attached_images,
            self.remote_image_urls.len(),
        );
    }
}
