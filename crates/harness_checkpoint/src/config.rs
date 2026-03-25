//! Configuration snapshot

use crate::checkpoint::ConfigSnapshot;
use crate::error::{CheckpointError, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Snapshot configuration files
pub fn snapshot_config(config_paths: &[&str]) -> Result<ConfigSnapshot> {
    let mut files = Vec::new();
    let mut env_vars = HashMap::new();

    // Read specified config files
    for path_str in config_paths {
        let path = Path::new(path_str);

        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| CheckpointError::StorageError(e.to_string()))?;

            let content_hash = format!("{:x}", md5_hash(&content));
            let metadata =
                fs::metadata(path).map_err(|e| CheckpointError::StorageError(e.to_string()))?;

            files.push(crate::checkpoint::FileSnapshot {
                path: path_str.to_string(),
                content_hash,
                size_bytes: metadata.len(),
            });
        }
    }

    // Capture environment variables (filter sensitive ones)
    let sensitive_patterns = ["PASSWORD", "SECRET", "KEY", "TOKEN", "API_KEY"];

    for (key, value) in std::env::vars() {
        let is_sensitive = sensitive_patterns
            .iter()
            .any(|p| key.to_uppercase().contains(p));

        if !is_sensitive {
            env_vars.insert(key, value);
        }
    }

    Ok(ConfigSnapshot {
        files,
        env_vars,
        created_at: Utc::now(),
    })
}

/// Capture metrics baseline
pub fn capture_metrics_baseline() -> crate::checkpoint::MetricsBaseline {
    crate::checkpoint::MetricsBaseline {
        timestamp: Utc::now(),
        cpu_percent: get_cpu_usage(),
        memory_mb: Some(get_memory_usage()),
        latency_ms: None,
        error_rate: None,
    }
}

/// Simple hash function
fn md5_hash(data: &str) -> u128 {
    let mut hash: u128 = 0;
    for (i, byte) in data.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u128).wrapping_mul(i as u128));
    }
    hash
}

/// Get memory usage (approximate)
fn get_memory_usage() -> u64 {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("vm_stat").output() {
            let output = String::from_utf8_lossy(&output.stdout);
            for line in output.lines() {
                if line.contains("Pages active:") {
                    if let Some(val) = line.split(':').nth(1) {
                        let pages: u64 = val.trim().trim_end_matches('.').parse().unwrap_or(0);
                        return pages * 4096 / 1024 / 1024; // Convert to MB
                    }
                }
            }
        }
        0
    }

    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}

/// Get CPU usage (placeholder)
fn get_cpu_usage() -> Option<f64> {
    None // Would require platform-specific implementation
}
