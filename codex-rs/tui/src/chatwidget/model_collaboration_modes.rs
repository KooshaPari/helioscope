use super::super::*;

impl ChatWidget {
    pub(crate) fn open_collaboration_modes_popup(&mut self) {
        let presets = collaboration_modes::presets_for_tui(self.models_manager.as_ref());
        if presets.is_empty() {
            self.add_info_message(
                "No collaboration modes are available right now.".to_string(),
                None,
            );
            return;
        }

        let current_kind = self
            .active_collaboration_mask
            .as_ref()
            .and_then(|mask| mask.mode)
            .or_else(|| {
                collaboration_modes::default_mask(self.models_manager.as_ref())
                    .and_then(|mask| mask.mode)
            });
        let items: Vec<SelectionItem> = presets
            .into_iter()
            .map(|mask| {
                let name = mask.name.clone();
                let is_current = current_kind == mask.mode;
                let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                    tx.send(AppEvent::UpdateCollaborationMode(mask.clone()));
                })];
                SelectionItem {
                    name,
                    is_current,
                    actions,
                    dismiss_on_select: true,
                    ..Default::default()
                }
            })
            .collect();

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some("Select Collaboration Mode".to_string()),
            subtitle: Some("Pick a collaboration preset.".to_string()),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        });
    }

    pub(crate) fn model_selection_actions(
        model_for_action: String,
        effort_for_action: Option<ReasoningEffortConfig>,
        should_prompt_plan_mode_scope: bool,
    ) -> Vec<SelectionAction> {
        vec![Box::new(move |tx| {
            if should_prompt_plan_mode_scope {
                tx.send(AppEvent::OpenPlanReasoningScopePrompt {
                    model: model_for_action.clone(),
                    effort: effort_for_action,
                });
                return;
            }

            tx.send(AppEvent::UpdateModel(model_for_action.clone()));
            tx.send(AppEvent::UpdateReasoningEffort(effort_for_action));
            tx.send(AppEvent::PersistModelSelection {
                model: model_for_action.clone(),
                effort: effort_for_action,
            });
        })]
    }

    pub(crate) fn should_prompt_plan_mode_reasoning_scope(
        &self,
        selected_model: &str,
        selected_effort: Option<ReasoningEffortConfig>,
    ) -> bool {
        if !self.collaboration_modes_enabled()
            || self.active_mode_kind() != ModeKind::Plan
            || selected_model != self.current_model()
        {
            return false;
        }

        selected_effort != self.effective_reasoning_effort()
            || selected_model != self.current_collaboration_mode.model()
            || selected_effort != self.current_collaboration_mode.reasoning_effort()
    }

    pub(crate) fn open_plan_reasoning_scope_prompt(
        &mut self,
        model: String,
        effort: Option<ReasoningEffortConfig>,
    ) {
        let reasoning_phrase = match effort {
            Some(ReasoningEffortConfig::None) => "no reasoning".to_string(),
            Some(selected_effort) => {
                format!(
                    "{} reasoning",
                    Self::reasoning_effort_label(selected_effort).to_lowercase()
                )
            }
            None => "the selected reasoning".to_string(),
        };
        let plan_only_description = format!("Always use {reasoning_phrase} in Plan mode.");
        let plan_reasoning_source = if let Some(plan_override) =
            self.config.plan_mode_reasoning_effort
        {
            format!(
                "user-chosen Plan override ({})",
                Self::reasoning_effort_label(plan_override).to_lowercase()
            )
        } else if let Some(plan_mask) = collaboration_modes::plan_mask(self.models_manager.as_ref())
        {
            match plan_mask.reasoning_effort.flatten() {
                Some(plan_effort) => format!(
                    "built-in Plan default ({})",
                    Self::reasoning_effort_label(plan_effort).to_lowercase()
                ),
                None => "built-in Plan default (no reasoning)".to_string(),
            }
        } else {
            "built-in Plan default".to_string()
        };
        let all_modes_description = format!(
            "Set the global default reasoning level and the Plan mode override. This replaces the current {plan_reasoning_source}."
        );
        let subtitle = format!("Choose where to apply {reasoning_phrase}.");

        let plan_only_actions: Vec<SelectionAction> = vec![Box::new({
            let model = model.clone();
            move |tx| {
                tx.send(AppEvent::UpdateModel(model.clone()));
                tx.send(AppEvent::UpdatePlanModeReasoningEffort(effort));
                tx.send(AppEvent::PersistPlanModeReasoningEffort(effort));
            }
        })];
        let all_modes_actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
            tx.send(AppEvent::UpdateModel(model.clone()));
            tx.send(AppEvent::UpdateReasoningEffort(effort));
            tx.send(AppEvent::UpdatePlanModeReasoningEffort(effort));
            tx.send(AppEvent::PersistPlanModeReasoningEffort(effort));
            tx.send(AppEvent::PersistModelSelection {
                model: model.clone(),
                effort,
            });
        })];

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(PLAN_MODE_REASONING_SCOPE_TITLE.to_string()),
            subtitle: Some(subtitle),
            footer_hint: Some(standard_popup_hint_line()),
            items: vec![
                SelectionItem {
                    name: PLAN_MODE_REASONING_SCOPE_PLAN_ONLY.to_string(),
                    description: Some(plan_only_description),
                    actions: plan_only_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: PLAN_MODE_REASONING_SCOPE_ALL_MODES.to_string(),
                    description: Some(all_modes_description),
                    actions: all_modes_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
    }
}
