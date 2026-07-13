//! Plugin API for custom detectors.
//!
//! A "plugin" is any executable the user configures: BugRadar spawns it once
//! per tick per active source, writes one JSON line describing that source's
//! current window to its stdin, and reads one JSON line of anomalies back
//! from its stdout. This is the same shape as an mdBook preprocessor or a
//! pre-commit hook, not a dynamically loaded `.so`/`.dll`: Rust gives no ABI
//! stability guarantee across compiler versions, so a `dlopen`'d plugin
//! compiled with a different rustc than the host BugRadar binary is
//! undefined behavior waiting to happen. A subprocess boundary avoids that
//! entirely, costs a few milliseconds of process-spawn overhead per tick,
//! and lets a detector be written in any language.

use std::process::Stdio;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::models::anomaly::{Anomaly, AnomalyKind};
use super::rolling_window::SourceWindow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDetectorConfig {
    pub id: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_timeout_ms() -> u64 {
    3000
}

/// An owned, cheap-to-clone snapshot of a `SourceWindow`. Captured while
/// holding the window's lock, then handed to `run_external_detector` after
/// the lock is dropped, so a slow or hanging plugin process never blocks
/// other sources from being processed on the same tick.
#[derive(Debug, Clone)]
pub struct SourceWindowSnapshot {
    pub total_entries: u64,
    pub error_count_last_tick: u64,
    pub warn_count_in_window: usize,
    pub error_rate_mean: f64,
    pub latency_samples_ms: Vec<f64>,
    pub recent_messages: Vec<String>,
}

impl SourceWindowSnapshot {
    pub fn capture(window: &SourceWindow) -> Self {
        Self {
            total_entries: window.total_entries,
            error_count_last_tick: window.error_count_last_tick,
            warn_count_in_window: window.warn_window.len(),
            error_rate_mean: window.error_window.mean(),
            latency_samples_ms: window.latency_window.iter().map(|(_, v)| *v).collect(),
            recent_messages: window.recent_messages.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct DetectorRequest<'a> {
    source_id: &'a str,
    total_entries: u64,
    error_count_last_tick: u64,
    warn_count_in_window: usize,
    error_rate_mean: f64,
    latency_samples_ms: &'a [f64],
    recent_messages: &'a [String],
}

#[derive(Debug, Default, Deserialize)]
struct DetectorResponse {
    #[serde(default)]
    anomalies: Vec<ExternalAnomaly>,
}

#[derive(Debug, Deserialize)]
struct ExternalAnomaly {
    label: String,
    value: f64,
    #[serde(default)]
    baseline: f64,
    #[serde(default)]
    contributing_entries: Vec<String>,
}

fn build_request<'a>(source_id: &'a str, snapshot: &'a SourceWindowSnapshot) -> DetectorRequest<'a> {
    DetectorRequest {
        source_id,
        total_entries: snapshot.total_entries,
        error_count_last_tick: snapshot.error_count_last_tick,
        warn_count_in_window: snapshot.warn_count_in_window,
        error_rate_mean: snapshot.error_rate_mean,
        latency_samples_ms: &snapshot.latency_samples_ms,
        recent_messages: &snapshot.recent_messages,
    }
}

/// Runs one configured plugin against a snapshot of one source's window.
/// Never panics and never propagates a plugin failure: a misbehaving
/// detector should not take down anomaly detection for every other source.
pub async fn run_external_detector(
    config: &PluginDetectorConfig,
    source_id: &str,
    snapshot: &SourceWindowSnapshot,
) -> Vec<Anomaly> {
    match run_external_detector_inner(config, source_id, snapshot).await {
        Ok(anomalies) => anomalies,
        Err(e) => {
            tracing::warn!("custom detector '{}' failed: {:#}", config.id, e);
            vec![]
        }
    }
}

async fn run_external_detector_inner(
    config: &PluginDetectorConfig,
    source_id: &str,
    snapshot: &SourceWindowSnapshot,
) -> Result<Vec<Anomaly>> {
    if config.command.trim().is_empty() {
        bail!("empty command");
    }

    let request = build_request(source_id, snapshot);
    let mut payload = serde_json::to_string(&request)?;
    payload.push('\n');

    let mut child = Command::new(&config.command)
        .args(&config.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("spawning detector '{}' ({})", config.id, config.command))?;

    {
        let mut stdin = child.stdin.take().context("plugin process has no stdin")?;
        stdin.write_all(payload.as_bytes()).await?;
        stdin.shutdown().await.ok();
    }

    let timeout = Duration::from_millis(config.timeout_ms.max(100));
    let output = tokio::time::timeout(timeout, child.wait_with_output())
        .await
        .with_context(|| format!("detector '{}' timed out after {}ms", config.id, config.timeout_ms))??;

    if !output.status.success() {
        bail!("detector '{}' exited with {}", config.id, output.status);
    }

    let response: DetectorResponse = serde_json::from_slice(&output.stdout)
        .with_context(|| format!("detector '{}' did not produce valid JSON on stdout", config.id))?;

    Ok(response
        .anomalies
        .into_iter()
        .map(|a| {
            Anomaly::new(
                AnomalyKind::Custom(a.label),
                source_id,
                a.value,
                a.baseline,
                a.contributing_entries,
            )
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Process-spawning tests against the real fixture binary live in
    // tests/external_detector_plugin.rs: `CARGO_BIN_EXE_*` is only set for
    // integration test binaries, not for unit tests compiled into the lib.

    #[test]
    fn empty_command_is_rejected_without_reaching_process_spawn() {
        let config = PluginDetectorConfig {
            id: "bad".to_string(),
            command: "  ".to_string(),
            args: vec![],
            timeout_ms: 1000,
        };
        let snapshot = SourceWindowSnapshot::capture(&SourceWindow::new(300));
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run_external_detector_inner(&config, "app-1", &snapshot));
        assert!(result.is_err());
    }
}
