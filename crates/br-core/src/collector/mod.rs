pub mod file_watcher;
pub mod docker_collector;
pub mod parser;

use std::collections::HashMap;
use std::path::PathBuf;

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::info;

use crate::models::log_entry::{LogEntry, WatchSource, WatchSourceKind};
use crate::plugin::registry::PluginRegistry;
use file_watcher::spawn_file_tail;

pub struct LogCollector {
    pub tx: mpsc::Sender<LogEntry>,
    pub rx: Option<mpsc::Receiver<LogEntry>>,
    active_tasks: DashMap<String, JoinHandle<()>>,
    registry: PluginRegistry,
}

impl LogCollector {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(4096);
        Self {
            tx,
            rx: Some(rx),
            active_tasks: DashMap::new(),
            registry: PluginRegistry::new(),
        }
    }

    pub fn take_receiver(&mut self) -> Option<mpsc::Receiver<LogEntry>> {
        self.rx.take()
    }

    pub fn start_watching(&self, source: &WatchSource) {
        let source_id = source.id.clone();

        match &source.kind {
            WatchSourceKind::FilePath { path } => {
                let parser_id = if source.parser_id == "auto" || source.parser_id.is_empty() {
                    "plaintext".to_string()
                } else {
                    source.parser_id.clone()
                };

                let parser = self.registry.create(&parser_id, &source.id, path)
                    .unwrap_or_else(|| {
                        self.registry.create("plaintext", &source.id, path).unwrap()
                    });

                info!("Watching file {} with parser {}", path, parser_id);
                let handle = spawn_file_tail(
                    PathBuf::from(path),
                    source.id.clone(),
                    parser,
                    self.tx.clone(),
                );
                self.active_tasks.insert(source_id, handle);
            }
            WatchSourceKind::DockerContainer { container_id, container_name } => {
                if let Ok(docker_collector) = docker_collector::DockerCollector::new() {
                    let parser = self.registry.create("docker", &source.id, container_name)
                        .unwrap_or_else(|| self.registry.create("plaintext", &source.id, container_name).unwrap());

                    let handle = docker_collector.spawn_container_tail(
                        container_id.clone(),
                        source.id.clone(),
                        container_name.clone(),
                        parser,
                        self.tx.clone(),
                    );
                    self.active_tasks.insert(source_id, handle);
                }
            }
            WatchSourceKind::DockerAllContainers => {
                tracing::warn!("DockerAllContainers watch not implemented in this version");
            }
        }
    }

    pub fn stop_watching(&self, source_id: &str) {
        if let Some((_, handle)) = self.active_tasks.remove(source_id) {
            handle.abort();
            info!("Stopped watching source {}", source_id);
        }
    }

    pub fn active_source_ids(&self) -> Vec<String> {
        self.active_tasks.iter().map(|e| e.key().clone()).collect()
    }
}
