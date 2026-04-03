use super::super::*;

impl ChatWidget {
    pub(crate) fn set_plan_mode_reasoning_effort(&mut self, effort: Option<ReasoningEffortConfig>) {
        self.config.plan_mode_reasoning_effort = effort;
        if self.collaboration_modes_enabled()
            && let Some(mask) = self.active_collaboration_mask.as_mut()
            && mask.mode == Some(ModeKind::Plan)
        {
            if let Some(effort) = effort {
                mask.reasoning_effort = Some(Some(effort));
            } else if let Some(plan_mask) =
                collaboration_modes::plan_mask(self.models_manager.as_ref())
            {
                mask.reasoning_effort = plan_mask.reasoning_effort;
            }
        }
    }

    /// Set the reasoning effort in the stored collaboration mode.
    pub(crate) fn set_reasoning_effort(&mut self, effort: Option<ReasoningEffortConfig>) {
        self.current_collaboration_mode =
            self.current_collaboration_mode
                .with_updates(None, Some(effort), None);
        if self.collaboration_modes_enabled()
            && let Some(mask) = self.active_collaboration_mask.as_mut()
            && mask.mode != Some(ModeKind::Plan)
        {
            mask.reasoning_effort = Some(effort);
        }
    }

    /// Set the personality in the widget's config copy.
    pub(crate) fn set_personality(&mut self, personality: Personality) {
        self.config.personality = Some(personality);
    }

    /// Set the model in the widget's config copy and stored collaboration mode.
    pub(crate) fn set_model(&mut self, model: &str) {
        self.current_collaboration_mode =
            self.current_collaboration_mode
                .with_updates(Some(model.to_string()), None, None);
        if self.collaboration_modes_enabled()
            && let Some(mask) = self.active_collaboration_mask.as_mut()
        {
            mask.model = Some(model.to_string());
        }
        self.refresh_model_display();
    }

    pub(crate) fn current_model(&self) -> &str {
        if !self.collaboration_modes_enabled() {
            return self.current_collaboration_mode.model();
        }
        self.active_collaboration_mask
            .as_ref()
            .and_then(|mask| mask.model.as_deref())
            .unwrap_or_else(|| self.current_collaboration_mode.model())
    }

    fn current_model_supports_personality(&self) -> bool {
        let model = self.current_model();
        self.models_manager
            .try_list_models()
            .ok()
            .and_then(|models| {
                models
                    .into_iter()
                    .find(|preset| preset.model == model)
                    .map(|preset| preset.supports_personality)
            })
            .unwrap_or(false)
    }

    /// Return whether the effective model currently advertises image-input support.
    ///
    /// We intentionally default to `true` when model metadata cannot be read so transient catalog
    /// failures do not hard-block user input in the UI.
    fn current_model_supports_images(&self) -> bool {
        let model = self.current_model();
        self.models_manager
            .try_list_models()
            .ok()
            .and_then(|models| {
                models
                    .into_iter()
                    .find(|preset| preset.model == model)
                    .map(|preset| preset.input_modalities.contains(&InputModality::Image))
            })
            .unwrap_or(true)
    }

    fn sync_image_paste_enabled(&mut self) {
        let enabled = self.current_model_supports_images();
        self.bottom_pane.set_image_paste_enabled(enabled);
    }

    fn effective_reasoning_effort(&self) -> Option<ReasoningEffortConfig> {
        if !self.collaboration_modes_enabled() {
            return self.current_collaboration_mode.reasoning_effort();
        }
        let current_effort = self.current_collaboration_mode.reasoning_effort();
        self.active_collaboration_mask
            .as_ref()
            .and_then(|mask| mask.reasoning_effort)
            .unwrap_or(current_effort)
    }

    fn effective_collaboration_mode(&self) -> CollaborationMode {
        if !self.collaboration_modes_enabled() {
            return self.current_collaboration_mode.clone();
        }
        self.active_collaboration_mask.as_ref().map_or_else(
            || self.current_collaboration_mode.clone(),
            |mask| self.current_collaboration_mode.apply_mask(mask),
        )
    }

    fn refresh_model_display(&mut self) {
        let effective = self.effective_collaboration_mode();
        self.session_header.set_model(effective.model());
        self.sync_image_paste_enabled();
    }

    fn model_display_name(&self) -> &str {
        let model = self.current_model();
        if model.is_empty() {
            DEFAULT_MODEL_DISPLAY_NAME
        } else {
            model
        }
    }

    fn personality_label(personality: Personality) -> &'static str {
        match personality {
            Personality::None => "None",
            Personality::Friendly => "Friendly",
            Personality::Pragmatic => "Pragmatic",
        }
    }

    fn personality_description(personality: Personality) -> &'static str {
        match personality {
            Personality::None => "No personality instructions.",
            Personality::Friendly => "Warm, collaborative, and helpful.",
            Personality::Pragmatic => "Concise, task-focused, and direct.",
        }
    }
}
