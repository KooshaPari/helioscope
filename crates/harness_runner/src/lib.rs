//! Runner module - Optimized process execution
//! Features: Timeout, streaming, environment isolation

use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::instrument;

/// Runner configuration
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub working_dir: Option<String>,
    pub timeout_secs: Option<u64>,
    pub env: std::collections::HashMap<String, String>,
    pub shell: bool,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            working_dir: None,
            timeout_secs: Some(30),
            env: std::collections::HashMap::new(),
            shell: false,
        }
    }
}

/// Process runner with environment control
pub struct Runner {
    config: RunnerConfig,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            config: RunnerConfig::default(),
        }
    }

    pub fn with_config(config: RunnerConfig) -> Self {
        Self { config }
    }

    pub fn with_working_dir(mut self, dir: &str) -> Self {
        self.config.working_dir = Some(dir.to_string());
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.config.env.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.config.timeout_secs = Some(secs);
        self
    }

    pub fn with_shell(mut self, shell: bool) -> Self {
        self.config.shell = shell;
        self
    }

    /// Run command and get result
    #[instrument(name = "runner_run", skip(self, args))]
    pub async fn run(&self, cmd: &str, args: &[&str]) -> Result<RunResult, RunError> {
        let start = Instant::now();

        let mut cmd = if self.config.shell {
            let mut c = Command::new("sh");
            c.arg("-c").arg(format!("{} {}", cmd, args.join(" ")));
            c
        } else {
            let mut c = Command::new(cmd);
            c.args(args);
            c
        };

        if let Some(ref dir) = self.config.working_dir {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.config.env {
            cmd.env(k, v);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = match self.config.timeout_secs {
            Some(timeout) => {
                match tokio::time::timeout(Duration::from_secs(timeout), cmd.output()).await {
                    Ok(Ok(output)) => output,
                    Ok(Err(e)) => return Err(RunError::IoError(e.to_string())),
                    Err(_) => return Err(RunError::Timeout(timeout)),
                }
            }
            None => cmd
                .output()
                .await
                .map_err(|e| RunError::IoError(e.to_string()))?,
        };

        let duration = start.elapsed();

        Ok(RunResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration,
        })
    }

    /// Run with stdin input
    pub async fn run_with_input(
        &self,
        cmd: &str,
        args: &[&str],
        input: &str,
    ) -> Result<RunResult, RunError> {
        let mut cmd = if self.config.shell {
            let mut c = Command::new("sh");
            c.arg("-c").arg(format!("{} {}", cmd, args.join(" ")));
            c
        } else {
            let mut c = Command::new(cmd);
            c.args(args);
            c
        };

        if let Some(ref dir) = self.config.working_dir {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.config.env {
            cmd.env(k, v);
        }

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| RunError::IoError(e.to_string()))?;

        if let Some(ref mut stdin) = child.stdin {
            stdin
                .write_all(input.as_bytes())
                .await
                .map_err(|e| RunError::IoError(e.to_string()))?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| RunError::IoError(e.to_string()))?;

        Ok(RunResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration: Duration::ZERO,
        })
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

/// Run result with metadata
#[derive(Debug, Clone)]
pub struct RunResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
}

impl RunResult {
    pub fn output(&self) -> String {
        if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            self.stdout.clone()
        }
    }

    pub fn output_lines(&self) -> Vec<String> {
        self.output().lines().map(|s| s.to_string()).collect()
    }
}

/// Run errors
#[derive(Debug)]
pub enum RunError {
    IoError(String),
    Timeout(u64),
    NotFound,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::IoError(s) => write!(f, "IO error: {}", s),
            RunError::Timeout(s) => write!(f, "Timeout after {}s", s),
            RunError::NotFound => write!(f, "Command not found"),
        }
    }
}

impl std::error::Error for RunError {}
