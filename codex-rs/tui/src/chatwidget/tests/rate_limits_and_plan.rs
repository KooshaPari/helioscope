use super::*;
use pretty_assertions::assert_eq;

include!("rate_limits_and_plan/rate_limit_state.rs");
include!("rate_limits_and_plan/rate_limit_switch_prompt.rs");

#[tokio::test]
async fn rate_limit_switch_prompt_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
    );

    chat.on_rate_limit_snapshot(Some(snapshot(92.0)));
    chat.maybe_show_pending_rate_limit_prompt();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("rate_limit_switch_prompt_popup", popup);
}

#[tokio::test]
async fn plan_implementation_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.open_plan_implementation_prompt();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("plan_implementation_popup", popup);
}

#[tokio::test]
async fn plan_implementation_popup_no_selected_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.open_plan_implementation_prompt();
    chat.handle_key_event(KeyEvent::from(KeyCode::Down));

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("plan_implementation_popup_no_selected", popup);
}

include!("rate_limits_and_plan/plan_submission_intro.rs");
include!("rate_limits_and_plan/plan_reasoning.rs");
include!("rate_limits_and_plan/plan_submission_and_flow.rs");
