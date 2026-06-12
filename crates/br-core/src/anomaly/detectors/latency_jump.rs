use crate::models::anomaly::{Anomaly, AnomalyConfig, AnomalyKind};
use super::{AnomalyDetector, SourceWindow};

pub struct LatencyJumpDetector;

impl AnomalyDetector for LatencyJumpDetector {
    fn detect(&self, source_id: &str, window: &SourceWindow, config: &AnomalyConfig) -> Vec<Anomaly> {
        if window.latency_window.len() < config.min_samples_for_baseline {
            return vec![];
        }

        let items: Vec<f64> = window.latency_window.iter().map(|(_, v)| *v).collect();
        let n = items.len();
        if n < 2 {
            return vec![];
        }

        let current = items[n - 1];
        let baseline: f64 = items[..n - 1].iter().sum::<f64>() / (n - 1) as f64;

        if baseline <= 0.0 {
            return vec![];
        }

        let factor = current / baseline;
        if factor >= config.latency_jump_threshold {
            vec![Anomaly::new(
                AnomalyKind::LatencyJump,
                source_id,
                current,
                baseline,
                vec![],
            )]
        } else {
            vec![]
        }
    }
}
