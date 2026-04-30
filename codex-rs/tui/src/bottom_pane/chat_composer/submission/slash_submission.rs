use super::*;

#[derive(Clone, Copy)]
pub(super) struct SlashCommandLookupOptions {
    pub(super) collaboration_modes_enabled: bool,
    pub(super) connectors_enabled: bool,
    pub(super) personality_command_enabled: bool,
    pub(super) realtime_conversation_enabled: bool,
    pub(super) audio_device_selection_enabled: bool,
    pub(super) windows_degraded_sandbox_active: bool,
}

pub(super) fn is_known_slash_command_name(
    name: &str,
    custom_prompts: &[CustomPrompt],
    options: SlashCommandLookupOptions,
) -> bool {
    if slash_commands::find_builtin_command(
        name,
        options.collaboration_modes_enabled,
        options.connectors_enabled,
        options.personality_command_enabled,
        options.realtime_conversation_enabled,
        options.audio_device_selection_enabled,
        options.windows_degraded_sandbox_active,
    )
    .is_some()
    {
        return true;
    }

    let prompt_prefix = format!("{PROMPTS_CMD_PREFIX}:");
    name.strip_prefix(&prompt_prefix)
        .map(|prompt_name| {
            custom_prompts
                .iter()
                .any(|prompt| prompt.name == prompt_name)
        })
        .unwrap_or(false)
}

pub(super) fn unrecognized_slash_command_message(name: &str) -> String {
    format!(r#"Unrecognized command '/{name}'. Type "/" for a list of supported commands."#)
}

pub(super) fn slash_command_args_elements(
    rest: &str,
    rest_offset: usize,
    text_elements: &[TextElement],
) -> Vec<TextElement> {
    if rest.is_empty() || text_elements.is_empty() {
        return Vec::new();
    }
    text_elements
        .iter()
        .filter_map(|elem| {
            if elem.byte_range.end <= rest_offset {
                return None;
            }
            let start = elem.byte_range.start.saturating_sub(rest_offset);
            let mut end = elem.byte_range.end.saturating_sub(rest_offset);
            if start >= rest.len() {
                return None;
            }
            end = end.min(rest.len());
            (start < end).then_some(elem.map_range(|_| ByteRange { start, end }))
        })
        .collect()
}
