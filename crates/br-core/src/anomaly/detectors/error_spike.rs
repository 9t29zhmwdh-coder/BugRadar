use crate::models::anomaly::{Anomaly, AnomalyConfig, AnomalyKind};
use super::{AnomalyDetector, SourceWindow};

pub struct ErrorSpikeDetector;

impl AnomalyDetector for ErrorSpikeDetector {
    fn detect(&self, source_id: &str, window: &SourceWindow, config: &AnomalyConfig) -> Vec<Anomaly> {
        if window.error_window.len() < config.min_samples_for_baseline {
            return vec![];
        }

        // Use all but the last sample as baseline
        let items: Vec<f64> = window.error_window.iter().map(|(_, v)| *v).collect();
        let n = items.len();
        if n < 2 {
            return vec![];
        }

        let current = items[n - 1];
        let baseline_sum: f64 = items[..n - 1].iter().sum();
        let baseline = baseline_sum / (n - 1) as f64;

        if baseline < 1.0 {
            // Avoid false positives when baseline is near zero
            if current < 5.0 {
                return vec![];
            }
            // Treat as spike against floor of 1
            let factor = current / 1.0;
            if factor < config.error_spike_threshold {
                return vec![];
            }
            return vec![Anomaly::new(
                AnomalyKind::ErrorSpike,
                source_id,
                current,
                1.0,
                vec![],
            )];
        }

        let factor = current / baseline;
        if factor >= config.error_spike_threshold {
            vec![Anomaly::new(
                AnomalyKind::ErrorSpike,
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
