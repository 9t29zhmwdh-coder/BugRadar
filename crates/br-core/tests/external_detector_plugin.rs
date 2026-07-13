//! End-to-end tests for the custom detector plugin API: spawns the real
//! `fixture_detector` binary as a subprocess, exactly like BugRadar would
//! spawn a user-configured plugin, and checks the JSON-over-stdio contract
//! holds for the happy path and for every failure mode a real plugin author
//! could hit (bad JSON, nonzero exit, hanging process).

use std::time::Duration;

use br_core::anomaly::external_detector::{run_external_detector, PluginDetectorConfig, SourceWindowSnapshot};
use br_core::anomaly::rolling_window::SourceWindow;
use br_core::models::anomaly::AnomalyKind;

fn fixture(mode: &str, timeout_ms: u64) -> PluginDetectorConfig {
    PluginDetectorConfig {
        id: format!("fixture-{mode}"),
        command: env!("CARGO_BIN_EXE_fixture_detector").to_string(),
        args: vec![mode.to_string()],
        timeout_ms,
    }
}

fn snapshot() -> SourceWindowSnapshot {
    let mut w = SourceWindow::new(300);
    w.record_message("disk full: /var");
    SourceWindowSnapshot::capture(&w)
}

#[tokio::test]
async fn parses_anomalies_from_a_well_behaved_plugin() {
    let config = fixture("ok", 2000);
    let anomalies = run_external_detector(&config, "app-1", &snapshot()).await;
    assert_eq!(anomalies.len(), 1);
    assert_eq!(anomalies[0].kind, AnomalyKind::Custom("disk full".to_string()));
    assert_eq!(anomalies[0].source_id, "app-1");
    assert_eq!(anomalies[0].contributing_entries, vec!["disk full: /var".to_string()]);
}

#[tokio::test]
async fn empty_anomalies_array_is_not_an_error() {
    let config = fixture("empty", 2000);
    let anomalies = run_external_detector(&config, "app-1", &snapshot()).await;
    assert!(anomalies.is_empty());
}

#[tokio::test]
async fn invalid_json_output_yields_no_anomalies_not_a_panic() {
    let config = fixture("badjson", 2000);
    let anomalies = run_external_detector(&config, "app-1", &snapshot()).await;
    assert!(anomalies.is_empty());
}

#[tokio::test]
async fn nonzero_exit_yields_no_anomalies_not_a_panic() {
    let config = fixture("fail", 2000);
    let anomalies = run_external_detector(&config, "app-1", &snapshot()).await;
    assert!(anomalies.is_empty());
}

#[tokio::test]
async fn a_hanging_plugin_is_killed_at_the_configured_timeout() {
    let config = fixture("sleep", 200);
    let started = std::time::Instant::now();
    let anomalies = run_external_detector(&config, "app-1", &snapshot()).await;
    assert!(anomalies.is_empty());
    assert!(started.elapsed() < Duration::from_secs(2), "should not wait for the full 5s sleep");
}
