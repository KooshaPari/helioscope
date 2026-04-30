/// Entering review mode uses the hint provided by the review request.
#[tokio::test]
async fn entered_review_mode_uses_request_hint() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "review-start".into(),
        msg: EventMsg::EnteredReviewMode(ReviewRequest {
            target: ReviewTarget::BaseBranch {
                branch: "feature".to_string(),
            },
            user_facing_hint: Some("feature branch".to_string()),
        }),
    });

    let cells = drain_insert_history(&mut rx);
    let banner = lines_to_single_string(cells.last().expect("review banner"));
    assert_eq!(banner, ">> Code review started: feature branch <<\n");
    assert!(chat.is_review_mode);
}

/// Entering review mode renders the current changes banner when requested.
#[tokio::test]
async fn entered_review_mode_defaults_to_current_changes_banner() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "review-start".into(),
        msg: EventMsg::EnteredReviewMode(ReviewRequest {
            target: ReviewTarget::UncommittedChanges,
            user_facing_hint: None,
        }),
    });

    let cells = drain_insert_history(&mut rx);
    let banner = lines_to_single_string(cells.last().expect("review banner"));
    assert_eq!(banner, ">> Code review started: current changes <<\n");
    assert!(chat.is_review_mode);
}

/// Exiting review restores the pre-review context window indicator.
#[tokio::test]
async fn review_restores_context_window_indicator() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    let context_window = 13_000;
    let pre_review_tokens = 12_700; // ~30% remaining after subtracting baseline.
    let review_tokens = 12_030; // ~97% remaining after subtracting baseline.

    chat.handle_codex_event(Event {
        id: "token-before".into(),
        msg: EventMsg::TokenCount(TokenCountEvent {
            info: Some(make_token_info(pre_review_tokens, context_window)),
            rate_limits: None,
        }),
    });
    assert_eq!(chat.bottom_pane.context_window_percent(), Some(30));

    chat.handle_codex_event(Event {
        id: "review-start".into(),
        msg: EventMsg::EnteredReviewMode(ReviewRequest {
            target: ReviewTarget::BaseBranch {
                branch: "feature".to_string(),
            },
            user_facing_hint: Some("feature branch".to_string()),
        }),
    });

    chat.handle_codex_event(Event {
        id: "token-review".into(),
        msg: EventMsg::TokenCount(TokenCountEvent {
            info: Some(make_token_info(review_tokens, context_window)),
            rate_limits: None,
        }),
    });
    assert_eq!(chat.bottom_pane.context_window_percent(), Some(97));

    chat.handle_codex_event(Event {
        id: "review-end".into(),
        msg: EventMsg::ExitedReviewMode(ExitedReviewModeEvent {
            review_output: None,
        }),
    });
    let _ = drain_insert_history(&mut rx);

    assert_eq!(chat.bottom_pane.context_window_percent(), Some(30));
    assert!(!chat.is_review_mode);
}

/// Receiving a TokenCount event without usage clears the context indicator.
#[tokio::test]
async fn token_count_none_resets_context_indicator() {
    let (mut chat, _rx, _ops) = make_chatwidget_manual(None).await;

    let context_window = 13_000;
    let pre_compact_tokens = 12_700;

    chat.handle_codex_event(Event {
        id: "token-before".into(),
        msg: EventMsg::TokenCount(TokenCountEvent {
            info: Some(make_token_info(pre_compact_tokens, context_window)),
            rate_limits: None,
        }),
    });
    assert_eq!(chat.bottom_pane.context_window_percent(), Some(30));

    chat.handle_codex_event(Event {
        id: "token-cleared".into(),
        msg: EventMsg::TokenCount(TokenCountEvent {
            info: None,
            rate_limits: None,
        }),
    });
    assert_eq!(chat.bottom_pane.context_window_percent(), None);
}

#[tokio::test]
async fn context_indicator_shows_used_tokens_when_window_unknown() {
    let (mut chat, _rx, _ops) = make_chatwidget_manual(Some("unknown-model")).await;

    chat.config.model_context_window = None;
    let auto_compact_limit = 200_000;
    chat.config.model_auto_compact_token_limit = Some(auto_compact_limit);

    // No model window, so the indicator should fall back to showing tokens used.
    let total_tokens = 106_000;
    let token_usage = TokenUsage {
        total_tokens,
        ..TokenUsage::default()
    };
    let token_info = TokenUsageInfo {
        total_token_usage: token_usage.clone(),
        last_token_usage: token_usage,
        model_context_window: None,
    };

    chat.handle_codex_event(Event {
        id: "token-usage".into(),
        msg: EventMsg::TokenCount(TokenCountEvent {
            info: Some(token_info),
            rate_limits: None,
        }),
    });

    assert_eq!(chat.bottom_pane.context_window_percent(), None);
    assert_eq!(
        chat.bottom_pane.context_window_used_tokens(),
        Some(total_tokens)
    );
}

#[cfg_attr(
    target_os = "macos",
    ignore = "system configuration APIs are blocked under macOS seatbelt"
)]
#[tokio::test]
async fn helpers_are_available_and_do_not_panic() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let cfg = test_config().await;
    let resolved_model = codex_core::test_support::get_model_offline(cfg.model.as_deref());
    let otel_manager = test_otel_manager(&cfg, resolved_model.as_str());
    let thread_manager = Arc::new(
        codex_core::test_support::thread_manager_with_models_provider(
            CodexAuth::from_api_key("test"),
            cfg.model_provider.clone(),
        ),
    );
    let auth_manager =
        codex_core::test_support::auth_manager_from_auth(CodexAuth::from_api_key("test"));
    let init = ChatWidgetInit {
        config: cfg,
        frame_requester: FrameRequester::test_dummy(),
        app_event_tx: tx,
        initial_user_message: None,
        enhanced_keys_supported: false,
        auth_manager,
        models_manager: thread_manager.get_models_manager(),
        feedback: codex_feedback::CodexFeedback::new(),
        is_first_run: true,
        feedback_audience: FeedbackAudience::External,
        model: Some(resolved_model),
        startup_tooltip_override: None,
        status_line_invalid_items_warned: Arc::new(AtomicBool::new(false)),
        otel_manager,
    };
    let mut w = ChatWidget::new(init, thread_manager);
    // Basic construction sanity.
    let _ = &mut w;
}

#[tokio::test]
async fn worked_elapsed_from_resets_when_timer_restarts() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    assert_eq!(chat.worked_elapsed_from(5), 5);
    assert_eq!(chat.worked_elapsed_from(9), 4);
    // Simulate status timer resetting (e.g., status indicator recreated for a new task).
    assert_eq!(chat.worked_elapsed_from(3), 3);
    assert_eq!(chat.worked_elapsed_from(7), 4);
}

#[tokio::test]
async fn replayed_thread_rollback_emits_ordered_app_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;

    chat.replay_initial_messages(vec![EventMsg::ThreadRolledBack(ThreadRolledBackEvent {
        num_turns: 2,
    })]);

    let mut saw = false;
    while let Ok(event) = rx.try_recv() {
        if let AppEvent::ApplyThreadRollback { num_turns } = event {
            saw = true;
            assert_eq!(num_turns, 2);
            break;
        }
    }

    assert!(saw, "expected replay rollback app event");
}

// (removed experimental resize snapshot test)
