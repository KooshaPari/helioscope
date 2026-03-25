use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use codex_protocol::protocol::RealtimeAudioFrame;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU16;

pub struct RecordedAudio {
    pub data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub struct VoiceCapture;

pub(crate) struct RecordingMeterState;

pub(crate) struct RealtimeAudioPlayer;

impl VoiceCapture {
    pub fn start() -> Result<Self, String> {
        Err("voice input is unavailable in this build".to_string())
    }

    pub fn start_realtime(_tx: AppEventSender) -> Result<Self, String> {
        Err("voice input is unavailable in this build".to_string())
    }

    pub fn stop(self) -> Result<RecordedAudio, String> {
        Err("voice input is unavailable in this build".to_string())
    }

    pub fn data_arc(&self) -> Arc<Mutex<Vec<i16>>> {
        Arc::new(Mutex::new(Vec::new()))
    }

    pub fn stopped_flag(&self) -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(true))
    }

    pub fn sample_rate(&self) -> u32 {
        0
    }

    pub fn channels(&self) -> u16 {
        0
    }

    pub fn last_peak_arc(&self) -> Arc<AtomicU16> {
        Arc::new(AtomicU16::new(0))
    }
}

impl RecordingMeterState {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn next_text(&mut self, _peak: u16) -> String {
        "⠤⠤⠤⠤".to_string()
    }
}

impl RealtimeAudioPlayer {
    pub(crate) fn start() -> Result<Self, String> {
        Err("voice output is unavailable in this build".to_string())
    }

    pub(crate) fn enqueue_frame(&self, _frame: &RealtimeAudioFrame) -> Result<(), String> {
        Err("voice output is unavailable in this build".to_string())
    }

    pub(crate) fn clear(&self) {}
}

pub fn transcribe_async(
    id: String,
    _audio: RecordedAudio,
    _context: Option<String>,
    tx: AppEventSender,
) {
    tx.send(AppEvent::TranscriptionFailed {
        id,
        error: "voice input is unavailable in this build".to_string(),
    });
}
