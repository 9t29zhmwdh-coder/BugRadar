//! Watching a file must pick a registered parser and never panic. Before,
//! json and plaintext were not registered, and the plaintext fallback
//! unwrapped None, so every file source killed its command.

use std::time::Duration;

use br_core::collector::LogCollector;
use br_core::plugin::registry::PluginRegistry;
use br_core::{WatchSource, WatchSourceKind};

#[test]
fn every_parser_the_collector_names_is_registered() {
    let registry = PluginRegistry::new();
    for id in ["plaintext", "json", "nginx", "docker"] {
        assert!(registry.create(id, "s1", "/tmp/x.log").is_some(), "{id} missing");
    }
}

#[test]
fn detection_is_deterministic_and_falls_back_to_plaintext() {
    let registry = PluginRegistry::new();
    assert_eq!(registry.detect("/var/log/nginx/access.log"), "nginx");
    assert_eq!(registry.detect("/srv/app/events.jsonl"), "json");
    assert_eq!(registry.detect("/var/lib/docker/containers/abc-json.log"), "docker");
    assert_eq!(registry.detect("/Users/me/app.log"), "plaintext");
    assert_eq!(registry.detect("/Users/me/no-extension"), "plaintext");
}

#[tokio::test]
async fn watching_a_file_starts_a_tail_and_delivers_entries() {
    let dir = std::env::temp_dir().join(format!("br-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("app.log");
    std::fs::write(&path, "").unwrap();

    let mut collector = LogCollector::new();
    let mut rx = collector.take_receiver().unwrap();
    let source = WatchSource::new(
        "demo",
        WatchSourceKind::FilePath { path: path.display().to_string() },
        "auto",
    );
    collector.start_watching(&source);
    assert_eq!(collector.active_source_ids(), vec![source.id.clone()]);

    tokio::time::sleep(Duration::from_millis(300)).await;
    std::fs::write(&path, "2026-09-26T17:00:00Z ERROR database connection refused\n").unwrap();

    let entry = tokio::time::timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("no entry within 5 s")
        .expect("channel closed");
    assert!(entry.message.contains("database connection refused"));

    collector.stop_watching(&source.id);
    let _ = std::fs::remove_dir_all(&dir);
}
