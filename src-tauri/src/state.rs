use std::sync::Arc;
use tokio::sync::Mutex;

use br_core::collector::LogCollector;
use br_core::anomaly::AnomalyEngine;
use br_core::db::Database;
use br_core::sysmon::MetricsCollector;

pub struct AppState {
    pub db: Arc<Database>,
    pub collector: Arc<Mutex<LogCollector>>,
    pub anomaly_engine: Arc<Mutex<AnomalyEngine>>,
    pub metrics: Arc<Mutex<MetricsCollector>>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        use br_core::models::anomaly::AnomalyConfig;
        Self {
            db: Arc::new(db),
            collector: Arc::new(Mutex::new(LogCollector::new())),
            anomaly_engine: Arc::new(Mutex::new(AnomalyEngine::new(AnomalyConfig::default()))),
            metrics: Arc::new(Mutex::new(MetricsCollector::new())),
        }
    }
}
