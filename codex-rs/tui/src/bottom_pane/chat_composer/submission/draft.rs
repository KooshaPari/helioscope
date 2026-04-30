use super::*;
use crate::bottom_pane::text_manipulation;

#[derive(Clone, Debug)]
pub(super) struct SubmissionRestoreState {
    pub(super) input: String,
    pub(super) text_elements: Vec<TextElement>,
    pub(super) mention_bindings: Vec<MentionBinding>,
    local_image_paths: Vec<PathBuf>,
    pending_pastes: Vec<(String, String)>,
}

#[derive(Debug)]
pub(super) struct PreparedSubmission {
    pub(super) text: String,
    pub(super) text_elements: Vec<TextElement>,
}

pub(super) fn prepare_submission_draft(
    input: &str,
    text_elements: Vec<TextElement>,
    pending_pastes: &[(String, String)],
) -> PreparedSubmission {
    let (expanded_text, expanded_elements) = if pending_pastes.is_empty() {
        (input.to_string(), text_elements)
    } else {
        text_manipulation::expand_pending_pastes(input, text_elements, pending_pastes)
    };
    let trimmed_text = expanded_text.trim().to_string();
    let trimmed_elements =
        text_manipulation::trim_text_elements(&expanded_text, &trimmed_text, expanded_elements);
    PreparedSubmission {
        text: trimmed_text,
        text_elements: trimmed_elements,
    }
}

impl ChatComposer {
    pub(super) fn capture_submission_restore_state(&self) -> SubmissionRestoreState {
        SubmissionRestoreState {
            input: self.textarea.text().to_string(),
            text_elements: self.textarea.text_elements(),
            mention_bindings: self.snapshot_mention_bindings(),
            local_image_paths: self
                .attached_images
                .iter()
                .map(|img| img.path.clone())
                .collect(),
            pending_pastes: self.pending_pastes.clone(),
        }
    }

    pub(super) fn restore_submission_restore_state(&mut self, state: &SubmissionRestoreState) {
        self.set_text_content_with_mention_bindings(
            state.input.clone(),
            state.text_elements.clone(),
            state.local_image_paths.clone(),
            state.mention_bindings.clone(),
        );
        self.pending_pastes.clone_from(&state.pending_pastes);
        self.textarea.set_cursor(state.input.len());
    }
}
