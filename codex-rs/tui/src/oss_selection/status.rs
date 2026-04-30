use std::io;
use std::time::Duration;

use codex_core::DEFAULT_LMSTUDIO_PORT;
use codex_core::DEFAULT_OLLAMA_PORT;

#[derive(Clone)]
pub(super) enum ProviderStatus {
    Running,
    NotRunning,
    Unknown,
}

pub(super) async fn check_lmstudio_status() -> ProviderStatus {
    match check_port_status(DEFAULT_LMSTUDIO_PORT).await {
        Ok(true) => ProviderStatus::Running,
        Ok(false) => ProviderStatus::NotRunning,
        Err(_) => ProviderStatus::Unknown,
    }
}

pub(super) async fn check_ollama_status() -> ProviderStatus {
    match check_port_status(DEFAULT_OLLAMA_PORT).await {
        Ok(true) => ProviderStatus::Running,
        Ok(false) => ProviderStatus::NotRunning,
        Err(_) => ProviderStatus::Unknown,
    }
}

async fn check_port_status(port: u16) -> io::Result<bool> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(io::Error::other)?;

    let url = format!("http://localhost:{port}");

    match client.get(&url).send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}
