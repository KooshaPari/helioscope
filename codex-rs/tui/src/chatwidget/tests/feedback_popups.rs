use super::*;

#[tokio::test]
async fn feedback_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    // Open the feedback category selection popup via slash command.
    chat.dispatch_command(SlashCommand::Feedback);

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("feedback_selection_popup", popup);
}

#[tokio::test]
async fn feedback_upload_consent_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    // Open the consent popup directly for a chosen category.
    chat.open_feedback_consent(crate::app_event::FeedbackCategory::Bug);

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("feedback_upload_consent_popup", popup);
}
