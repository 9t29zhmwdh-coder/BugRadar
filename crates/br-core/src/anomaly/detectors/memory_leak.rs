use crate::models::anomaly::{Anomaly, AnomalyConfig, AnomalyKind};
use super::{AnomalyDetector, SourceWindow};

pub struct MemoryLeakDetector;

impl AnomalyDetector for MemoryLeakDetector {
    fn detect(&self, source_id: &str, window: &SourceWindow, config: &AnomalyConfig) -> Vec<Anomaly> {
        let items: Vec<(chrono::DateTime<chrono::Utc>, f64)> = window.memory_window.iter().cloned().collect();
        let n = items.len();

        if n < config.min_samples_for_baseline {
            return vec![];
        }

        // Calculate growth rate: compare first half mean vs second half mean
        let mid = n / 2;
        let first_half_mean: f64 = items[..mid].iter().map(|(_, v)| v).sum::<f64>() / mid as f64;
        let second_half_mean: f64 = items[mid..].iter().map(|(_, v)| v).sum::<f64>() / (n - mid) as f64;

        if first_half_mean <= 0.0 {
            return vec![];
        }

        // Time span in minutes
        let time_span_minutes = {
            let start = items.first().map(|(t, _)| *t).unwrap();
            let end = items.last().map(|(t, _)| *t).unwrap();
            (end - start).num_seconds() as f64 / 60.0
        };

        if time_span_minutes < 1.0 {
            return vec![];
        }

        let growth_mb_per_min = (second_half_mean - first_half_mean) / time_span_minutes;

        if growth_mb_per_min >= config.memory_growth_threshold_mb_per_min {
            vec![Anomaly::new(
                AnomalyKind::MemoryLeak,
                source_id,
                growth_mb_per_min,
                config.memory_growth_threshold_mb_per_min,
                vec![],
            )]
        } else {
            vec![]
        }
    }
}
