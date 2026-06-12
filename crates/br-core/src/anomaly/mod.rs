pub mod rolling_window;
pub mod incident_grouper;
pub mod detectors;

use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, info};

use crate::models::anomaly::{Anomaly, AnomalyConfig};
use crate::models::incident::Incident;
use crate::models::log_entry::LogEntry;
use rolling_window::{SourceWindow, SourceWindows};
use incident_grouper::IncidentGrouper;
use detectors::{AnomalyDetector, ErrorSpikeDetector, LatencyJumpDetector, MemoryLeakDetector};

pub struct AnomalyEvent {
    pub anomaly: Anomaly,
    pub incident: Incident,
    pub incident_is_new: bool,
}

pub struct AnomalyEngine {
    pub anomaly_tx: mpsc::Sender<AnomalyEvent>,
    pub anomaly_rx: Option<mpsc::Receiver<AnomalyEvent>>,
    config: Arc<AnomalyConfig>,
    source_windows: Arc<SourceWindows>,
}

impl AnomalyEngine {
    pub fn new(config: AnomalyConfig) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            anomaly_tx: tx,
            anomaly_rx: Some(rx),
            config: Arc::new(config),
            source_windows: Arc::new(DashMap::new()),
        }
    }

    pub fn take_receiver(&mut self) -> Option<mpsc::Receiver<AnomalyEvent>> {
        self.anomaly_rx.take()
    }

    /// Spawn the background task that processes incoming log entries and detects anomalies.
    pub fn spawn(
        &self,
        mut log_rx: mpsc::Receiver<LogEntry>,
    ) -> tokio::task::JoinHandle<()> {
        let tx = self.anomaly_tx.clone();
        let config = self.config.clone();
        let windows = self.source_windows.clone();

        tokio::spawn(async move {
            let detectors: Vec<Box<dyn AnomalyDetector>> = vec![
                Box::new(ErrorSpikeDetector),
                Box::new(LatencyJumpDetector),
                Box::new(MemoryLeakDetector),
            ];

            let mut grouper = IncidentGrouper::new((*config).clone());
            let mut tick = time::interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    Some(entry) = log_rx.recv() => {
                        let mut window = windows
                            .entry(entry.source_id.clone())
                            .or_insert_with(|| SourceWindow::new(config.window_seconds));
                        window.record_entry_level(&entry.level);

                        // Extract latency from structured fields if present
                        if let Some(lat) = entry.fields.get("latency_ms").or_else(|| entry.fields.get("duration_ms")) {
                            if let Some(v) = lat.as_f64() {
                                window.latency_window.push(v);
                            }
                        }
                    }
                    _ = tick.tick() => {
                        // Flush all windows and run detectors
                        let source_ids: Vec<String> = windows.iter().map(|e| e.key().clone()).collect();

                        for source_id in source_ids {
                            let anomalies = {
                                let mut window = windows.get_mut(&source_id).unwrap();
                                window.flush_tick();
                                detectors.iter()
                                    .flat_map(|d| d.detect(&source_id, &window, &config))
                                    .collect::<Vec<_>>()
                            };

                            for anomaly in anomalies {
                                debug!("Anomaly detected: {:?} on {}", anomaly.kind, source_id);
                                let (incident, is_new) = grouper.process_anomaly(&anomaly);
                                let _ = tx.send(AnomalyEvent {
                                    anomaly,
                                    incident,
                                    incident_is_new: is_new,
                                }).await;
                            }
                        }

                        // Auto-resolve stale incidents (60 min)
                        let stale = grouper.resolve_stale_incidents(3600);
                        if !stale.is_empty() {
                            info!("Auto-resolved {} stale incidents", stale.len());
                        }
                    }
                }
            }
        })
    }
}
