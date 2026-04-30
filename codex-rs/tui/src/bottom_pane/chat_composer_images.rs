use std::collections::HashSet;
use std::path::PathBuf;

use codex_protocol::models::local_image_label_text;
use codex_protocol::user_input::TextElement;
use ratatui::style::Stylize;
use ratatui::text::Line;

use super::LocalImageAttachment;
use super::textarea::TextArea;
use crate::bottom_pane::chat_composer::AttachedImage;

pub(crate) fn current_text_with_pending(
    textarea: &TextArea,
    pending_pastes: &[(String, String)],
) -> String {
    let mut text = textarea.text().to_string();
    for (placeholder, actual) in pending_pastes {
        if text.contains(placeholder) {
            text = text.replace(placeholder, actual);
        }
    }
    text
}

pub(crate) fn filter_pending_pastes(
    text: &str,
    pending_pastes: Vec<(String, String)>,
) -> Vec<(String, String)> {
    pending_pastes
        .into_iter()
        .filter(|(placeholder, _)| text.contains(placeholder))
        .collect()
}

#[cfg(test)]
pub(super) fn local_image_paths(attached_images: &[AttachedImage]) -> Vec<PathBuf> {
    attached_images.iter().map(|img| img.path.clone()).collect()
}

pub(super) fn local_images(attached_images: &[AttachedImage]) -> Vec<LocalImageAttachment> {
    attached_images
        .iter()
        .map(|img| LocalImageAttachment {
            placeholder: img.placeholder.clone(),
            path: img.path.clone(),
        })
        .collect()
}

pub(super) fn prune_attached_images_for_submission(
    attached_images: &mut Vec<AttachedImage>,
    text: &str,
    text_elements: &[TextElement],
) {
    if attached_images.is_empty() {
        return;
    }

    let image_placeholders: HashSet<&str> = text_elements
        .iter()
        .filter_map(|elem| elem.placeholder(text))
        .collect();
    attached_images.retain(|img| image_placeholders.contains(img.placeholder.as_str()));
}

pub(super) fn attach_image(
    textarea: &mut TextArea,
    attached_images: &mut Vec<AttachedImage>,
    remote_image_count: usize,
    path: PathBuf,
) {
    let image_number = remote_image_count + attached_images.len() + 1;
    let placeholder = local_image_label_text(image_number);
    textarea.insert_element(&placeholder);
    attached_images.push(AttachedImage { placeholder, path });
}

#[cfg(test)]
pub(super) fn take_recent_submission_images(
    attached_images: &mut Vec<AttachedImage>,
) -> Vec<PathBuf> {
    let images = std::mem::take(attached_images);
    images.into_iter().map(|img| img.path).collect()
}

pub(super) fn take_recent_submission_images_with_placeholders(
    attached_images: &mut Vec<AttachedImage>,
) -> Vec<LocalImageAttachment> {
    let images = std::mem::take(attached_images);
    images
        .into_iter()
        .map(|img| LocalImageAttachment {
            placeholder: img.placeholder,
            path: img.path,
        })
        .collect()
}

pub(super) fn remote_images_lines(
    remote_image_urls: &[String],
    selected_remote_image_index: Option<usize>,
) -> Vec<Line<'static>> {
    remote_image_urls
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let label = local_image_label_text(idx + 1);
            if selected_remote_image_index == Some(idx) {
                label.cyan().reversed().into()
            } else {
                label.cyan().into()
            }
        })
        .collect()
}

pub(super) fn remove_selected_remote_image(
    remote_image_urls: &mut Vec<String>,
    selected_remote_image_index: &mut Option<usize>,
    selected_index: usize,
) {
    if selected_index >= remote_image_urls.len() {
        *selected_remote_image_index = None;
        return;
    }

    remote_image_urls.remove(selected_index);
    *selected_remote_image_index = if remote_image_urls.is_empty() {
        None
    } else {
        Some(selected_index.min(remote_image_urls.len() - 1))
    };
}

pub(super) fn relabel_attached_images_and_update_placeholders(
    textarea: &mut TextArea,
    attached_images: &mut [AttachedImage],
    remote_image_count: usize,
) {
    for (idx, image) in attached_images.iter_mut().enumerate() {
        let expected = local_image_label_text(remote_image_count + idx + 1);
        let current = image.placeholder.clone();
        if current == expected {
            continue;
        }

        image.placeholder = expected.clone();
        let _renamed = textarea.replace_element_payload(&current, &expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-HELIOS-001
    #[test]
    fn filter_pending_pastes_keeps_only_visible_placeholders() {
        let filtered = filter_pending_pastes(
            "before [Pasted Content 10 chars] after",
            vec![
                (
                    "[Pasted Content 10 chars]".to_string(),
                    "abcdefghij".to_string(),
                ),
                ("[Pasted Content 5 chars]".to_string(), "12345".to_string()),
            ],
        );

        assert_eq!(
            filtered,
            vec![(
                "[Pasted Content 10 chars]".to_string(),
                "abcdefghij".to_string()
            )]
        );
    }
}
