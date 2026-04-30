use codex_chatgpt::connectors::AppInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RealtimeAudioDeviceKind {
    Microphone,
    Speaker,
}

impl RealtimeAudioDeviceKind {
    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Microphone => "Microphone",
            Self::Speaker => "Speaker",
        }
    }

    pub(crate) fn noun(self) -> &'static str {
        match self {
            Self::Microphone => "microphone",
            Self::Speaker => "speaker",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConnectorsSnapshot {
    pub(crate) connectors: Vec<AppInfo>,
}

/// The exit strategy requested by the UI layer.
///
/// Most user-initiated exits should use `ShutdownFirst` so core cleanup runs and the UI exits only
/// after core acknowledges completion. `Immediate` is an escape hatch for cases where shutdown has
/// already completed (or is being bypassed) and the UI loop should terminate right away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExitMode {
    /// Shutdown core and exit after completion.
    ShutdownFirst,
    /// Exit the UI loop immediately without waiting for shutdown.
    ///
    /// This skips `Op::Shutdown`, so any in-flight work may be dropped and
    /// cleanup that normally runs before `ShutdownComplete` can be missed.
    Immediate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeedbackCategory {
    BadResult,
    GoodResult,
    Bug,
    SafetyCheck,
    Other,
}
