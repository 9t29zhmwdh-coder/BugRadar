pub mod error_spike;
pub mod latency_jump;
pub mod memory_leak;

pub use error_spike::ErrorSpikeDetector;
pub use latency_jump::LatencyJumpDetector;
pub use memory_leak::MemoryLeakDetector;

use crate::models::anomaly::{Anomaly, AnomalyConfig};
use super::rolling_window::SourceWindow;

pub trait AnomalyDetector: Send + Sync {
    fn detect(&self, source_id: &str, window: &SourceWindow, config: &AnomalyConfig) -> Vec<Anomaly>;
}
