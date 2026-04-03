use super::*;

impl ChatComposer {
    pub(crate) fn sync_popups(&mut self) {
        self.sync_slash_command_elements();
        if !self.popups_enabled() {
            self.active_popup = ActivePopup::None;
            return;
        }
        let file_token = Self::current_at_token(&self.textarea);
        let browsing_history = self
            .history
            .should_handle_navigation(self.textarea.text(), self.textarea.cursor());
        // When browsing input history (shell-style Up/Down recall), skip all popup
        // synchronization so nothing steals focus from continued history navigation.
        if browsing_history {
            if self.current_file_query.is_some() {
                self.app_event_tx
                    .send(AppEvent::StartFileSearch(String::new()));
                self.current_file_query = None;
            }
            self.active_popup = ActivePopup::None;
            return;
        }
        let mention_token = self.current_mention_token();

        let allow_command_popup =
            self.slash_commands_enabled() && file_token.is_none() && mention_token.is_none();
        self.sync_command_popup(allow_command_popup);

        if matches!(self.active_popup, ActivePopup::Command(_)) {
            if self.current_file_query.is_some() {
                self.app_event_tx
                    .send(AppEvent::StartFileSearch(String::new()));
                self.current_file_query = None;
            }
            self.dismissed_file_popup_token = None;
            self.dismissed_mention_popup_token = None;
            return;
        }

        if let Some(token) = mention_token {
            if self.current_file_query.is_some() {
                self.app_event_tx
                    .send(AppEvent::StartFileSearch(String::new()));
                self.current_file_query = None;
            }
            self.sync_mention_popup(token);
            return;
        }
        self.dismissed_mention_popup_token = None;

        if let Some(token) = file_token {
            self.sync_file_search_popup(token);
            return;
        }

        if self.current_file_query.is_some() {
            self.app_event_tx
                .send(AppEvent::StartFileSearch(String::new()));
            self.current_file_query = None;
        }
        self.dismissed_file_popup_token = None;
        if matches!(
            self.active_popup,
            ActivePopup::File(_) | ActivePopup::Skill(_)
        ) {
            self.active_popup = ActivePopup::None;
        }
    }

    /// Keep slash command elements aligned with the current first line.
    fn sync_slash_command_elements(&mut self) {
        if !self.slash_commands_enabled() {
            return;
        }
        let text = self.textarea.text();
        let first_line_end = text.find('\n').unwrap_or(text.len());
        let first_line = &text[..first_line_end];
        let desired_range = self.slash_command_element_range(first_line);
        // Slash commands are only valid at byte 0 of the first line.
        // Any slash-shaped element not matching the current desired prefix is stale.
        let mut has_desired = false;
        let mut stale_ranges = Vec::new();
        for elem in self.textarea.text_elements() {
            let Some(payload) = elem.placeholder(text) else {
                continue;
            };
            if payload.strip_prefix('/').is_none() {
                continue;
            }
            let range = elem.byte_range.start..elem.byte_range.end;
            if desired_range.as_ref() == Some(&range) {
                has_desired = true;
            } else {
                stale_ranges.push(range);
            }
        }

        for range in stale_ranges {
            self.textarea.remove_element_range(range);
        }

        if let Some(range) = desired_range
            && !has_desired
        {
            self.textarea.add_element_range(range);
        }
    }

    fn slash_command_element_range(&self, first_line: &str) -> Option<Range<usize>> {
        let (name, _rest, _rest_offset) = parse_slash_name(first_line)?;
        if name.contains('/') {
            return None;
        }
        let element_end = 1 + name.len();
        let has_space_after = first_line
            .get(element_end..)
            .and_then(|tail| tail.chars().next())
            .is_some_and(char::is_whitespace);
        if !has_space_after {
            return None;
        }
        if self.is_known_slash_name(name) {
            Some(0..element_end)
        } else {
            None
        }
    }

    fn is_known_slash_name(&self, name: &str) -> bool {
        let is_builtin = slash_commands::find_builtin_command(
            name,
            self.collaboration_modes_enabled,
            self.connectors_enabled,
            self.personality_command_enabled,
            self.realtime_conversation_enabled,
            self.audio_device_selection_enabled,
            self.windows_degraded_sandbox_active,
        )
        .is_some();
        if is_builtin {
            return true;
        }
        if let Some(rest) = name.strip_prefix(PROMPTS_CMD_PREFIX)
            && let Some(prompt_name) = rest.strip_prefix(':')
        {
            return self
                .custom_prompts
                .iter()
                .any(|prompt| prompt.name == prompt_name);
        }
        false
    }

    /// If the cursor is currently within a slash command on the first line,
    /// extract the command name and the rest of the line after it.
    /// Returns None if the cursor is outside a slash command.
    fn slash_command_under_cursor(first_line: &str, cursor: usize) -> Option<(&str, &str)> {
        if !first_line.starts_with('/') {
            return None;
        }

        let name_start = 1usize;
        let name_end = first_line[name_start..]
            .find(char::is_whitespace)
            .map(|idx| name_start + idx)
            .unwrap_or_else(|| first_line.len());

        if cursor > name_end {
            return None;
        }

        let name = &first_line[name_start..name_end];
        let rest_start = first_line[name_end..]
            .find(|c: char| !c.is_whitespace())
            .map(|idx| name_end + idx)
            .unwrap_or(name_end);
        let rest = &first_line[rest_start..];

        Some((name, rest))
    }

    /// Heuristic for whether the typed slash command looks like a valid
    /// prefix for any known command (built-in or custom prompt).
    /// Empty names only count when there is no extra content after the '/'.
    fn looks_like_slash_prefix(&self, name: &str, rest_after_name: &str) -> bool {
        if !self.slash_commands_enabled() {
            return false;
        }
        if name.is_empty() {
            return rest_after_name.is_empty();
        }

        if slash_commands::has_builtin_prefix(
            name,
            self.collaboration_modes_enabled,
            self.connectors_enabled,
            self.personality_command_enabled,
            self.realtime_conversation_enabled,
            self.audio_device_selection_enabled,
            self.windows_degraded_sandbox_active,
        ) {
            return true;
        }

        self.custom_prompts.iter().any(|prompt| {
            fuzzy_match(&format!("{PROMPTS_CMD_PREFIX}:{}", prompt.name), name).is_some()
        })
    }

    /// Synchronize `self.command_popup` with the current text in the
    /// textarea. This must be called after every modification that can change
    /// the text so the popup is shown/updated/hidden as appropriate.
    fn sync_command_popup(&mut self, allow: bool) {
        if !allow {
            if matches!(self.active_popup, ActivePopup::Command(_)) {
                self.active_popup = ActivePopup::None;
            }
            return;
        }
        // Determine whether the caret is inside the initial '/name' token on the first line.
        let text = self.textarea.text();
        let first_line_end = text.find('\n').unwrap_or(text.len());
        let first_line = &text[..first_line_end];
        let cursor = self.textarea.cursor();
        let caret_on_first_line = cursor <= first_line_end;

        let is_editing_slash_command_name = caret_on_first_line
            && Self::slash_command_under_cursor(first_line, cursor)
                .is_some_and(|(name, rest)| self.looks_like_slash_prefix(name, rest));

        // If the cursor is currently positioned within an `@token`, prefer the
        // file-search popup over the slash popup so users can insert a file path
        // as an argument to the command (e.g., "/review @docs/...").
        if Self::current_at_token(&self.textarea).is_some() {
            if matches!(self.active_popup, ActivePopup::Command(_)) {
                self.active_popup = ActivePopup::None;
            }
            return;
        }
        match &mut self.active_popup {
            ActivePopup::Command(popup) => {
                if is_editing_slash_command_name {
                    popup.on_composer_text_change(first_line.to_string());
                } else {
                    self.active_popup = ActivePopup::None;
                }
            }
            _ => {
                if is_editing_slash_command_name {
                    let collaboration_modes_enabled = self.collaboration_modes_enabled;
                    let connectors_enabled = self.connectors_enabled;
                    let personality_command_enabled = self.personality_command_enabled;
                    let realtime_conversation_enabled = self.realtime_conversation_enabled;
                    let audio_device_selection_enabled = self.audio_device_selection_enabled;
                    let mut command_popup = CommandPopup::new(
                        self.custom_prompts.clone(),
                        CommandPopupFlags {
                            collaboration_modes_enabled,
                            connectors_enabled,
                            personality_command_enabled,
                            realtime_conversation_enabled,
                            audio_device_selection_enabled,
                            windows_degraded_sandbox_active: self.windows_degraded_sandbox_active,
                        },
                    );
                    command_popup.on_composer_text_change(first_line.to_string());
                    self.active_popup = ActivePopup::Command(command_popup);
                }
            }
        }
    }
    pub(crate) fn set_custom_prompts(&mut self, prompts: Vec<CustomPrompt>) {
        self.custom_prompts = prompts.clone();
        if let ActivePopup::Command(popup) = &mut self.active_popup {
            popup.set_prompts(prompts);
        }
    }

    /// Synchronize `self.file_search_popup` with the current text in the textarea.
    /// Note this is only called when self.active_popup is NOT Command.
    fn sync_file_search_popup(&mut self, query: String) {
        // If user dismissed popup for this exact query, don't reopen until text changes.
        if self.dismissed_file_popup_token.as_ref() == Some(&query) {
            return;
        }

        if query.is_empty() {
            self.app_event_tx
                .send(AppEvent::StartFileSearch(String::new()));
        } else {
            self.app_event_tx
                .send(AppEvent::StartFileSearch(query.clone()));
        }

        match &mut self.active_popup {
            ActivePopup::File(popup) => {
                if query.is_empty() {
                    popup.set_empty_prompt();
                } else {
                    popup.set_query(&query);
                }
            }
            _ => {
                let mut popup = FileSearchPopup::new();
                if query.is_empty() {
                    popup.set_empty_prompt();
                } else {
                    popup.set_query(&query);
                }
                self.active_popup = ActivePopup::File(popup);
            }
        }

        if query.is_empty() {
            self.current_file_query = None;
        } else {
            self.current_file_query = Some(query);
        }
        self.dismissed_file_popup_token = None;
    }

    fn sync_mention_popup(&mut self, query: String) {
        if self.dismissed_mention_popup_token.as_ref() == Some(&query) {
            return;
        }

        let mentions = self.mention_items();
        if mentions.is_empty() {
            self.active_popup = ActivePopup::None;
            return;
        }

        match &mut self.active_popup {
            ActivePopup::Skill(popup) => {
                popup.set_query(&query);
                popup.set_mentions(mentions);
            }
            _ => {
                let mut popup = SkillPopup::new(mentions);
                popup.set_query(&query);
                self.active_popup = ActivePopup::Skill(popup);
            }
        }
    }

    fn mention_items(&self) -> Vec<MentionItem> {
        let mut mentions = Vec::new();

        if let Some(skills) = self.skills.as_ref() {
            for skill in skills {
                let display_name = skill_display_name(skill).to_string();
                let description = skill_description(skill);
                let skill_name = skill.name.clone();
                let search_terms = if display_name == skill.name {
                    vec![skill_name.clone()]
                } else {
                    vec![skill_name.clone(), display_name.clone()]
                };
                mentions.push(MentionItem {
                    display_name,
                    description,
                    insert_text: format!("${skill_name}"),
                    search_terms,
                    path: Some(skill.path_to_skills_md.to_string_lossy().into_owned()),
                    category_tag: (skill.scope == codex_protocol::protocol::SkillScope::Repo)
                        .then(|| "[Repo]".to_string()),
                });
            }
        }

        if self.connectors_enabled
            && let Some(snapshot) = self.connectors_snapshot.as_ref()
        {
            for connector in &snapshot.connectors {
                if !connector.is_accessible || !connector.is_enabled {
                    continue;
                }
                let display_name = connectors::connector_display_label(connector);
                let description = Some(Self::connector_brief_description(connector));
                let slug = codex_core::connectors::connector_mention_slug(connector);
                let search_terms = vec![display_name.clone(), connector.id.clone(), slug.clone()];
                let connector_id = connector.id.as_str();
                mentions.push(MentionItem {
                    display_name: display_name.clone(),
                    description,
                    insert_text: format!("${slug}"),
                    search_terms,
                    path: Some(format!("app://{connector_id}")),
                    category_tag: Some("[App]".to_string()),
                });
            }
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        for mention in &mentions {
            *counts.entry(mention.insert_text.clone()).or_insert(0) += 1;
        }
        for mention in &mut mentions {
            if counts.get(&mention.insert_text).copied().unwrap_or(0) <= 1 {
                mention.category_tag = None;
            }
        }

        mentions
    }

    fn connector_brief_description(connector: &AppInfo) -> String {
        Self::connector_description(connector).unwrap_or_default()
    }

    fn connector_description(connector: &AppInfo) -> Option<String> {
        connector
            .description
            .as_deref()
            .map(str::trim)
            .filter(|description| !description.is_empty())
            .map(str::to_string)
    }
}
