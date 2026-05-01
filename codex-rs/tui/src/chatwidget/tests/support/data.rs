use super::super::*;

pub(crate) fn invalid_value(
    candidate: impl Into<String>,
    allowed: impl Into<String>,
) -> ConstraintError {
    ConstraintError::InvalidValue {
        field_name: "<unknown>",
        candidate: candidate.into(),
        allowed: allowed.into(),
        requirement_source: RequirementSource::Unknown,
    }
}

pub(crate) fn snapshot(percent: f64) -> RateLimitSnapshot {
    RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: percent,
            window_minutes: Some(60),
            resets_at: None,
        }),
        secondary: None,
        credits: None,
        plan_type: None,
    }
}

pub(crate) fn make_token_info(total_tokens: i64, context_window: i64) -> TokenUsageInfo {
    fn usage(total_tokens: i64) -> TokenUsage {
        TokenUsage {
            total_tokens,
            ..TokenUsage::default()
        }
    }

    TokenUsageInfo {
        total_token_usage: usage(total_tokens),
        last_token_usage: usage(total_tokens),
        model_context_window: Some(context_window),
    }
}

pub(crate) fn get_available_model(chat: &ChatWidget, model: &str) -> ModelPreset {
    let models = chat
        .models_manager
        .try_list_models()
        .expect("models lock available");
    models
        .iter()
        .find(|&preset| preset.model == model)
        .cloned()
        .unwrap_or_else(|| panic!("{model} preset not found"))
}
