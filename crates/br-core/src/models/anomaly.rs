use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyKind {
    ErrorSpike,
    LatencyJump,
    MemoryLeak,
    CrashLoop,
    UnhandledException,
    DatabaseTimeout,
    Custom(String),
}

impl AnomalyKind {
    pub fn label(&self) -> &str {
        match self {
            Self::ErrorSpike => "Error Spike",
            Self::LatencyJump => "Latency Jump",
            Self::MemoryLeak => "Memory Leak",
            Self::CrashLoop => "Crash Loop",
            Self::UnhandledException => "Unhandled Exception",
            Self::DatabaseTimeout => "Database Timeout",
            Self::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn from_deviation(factor: f64) -> Self {
        if factor >= 10.0 { Self::Critical }
        else if factor >= 5.0 { Self::High }
        else if factor >= 2.5 { Self::Medium }
        else { Self::Low }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: String,
    pub detected_at: DateTime<Utc>,
    pub kind: AnomalyKind,
    pub source_id: String,
    pub severity: Severity,
    pub value: f64,
    pub baseline: f64,
    pub deviation_factor: f64,
    pub contributing_entries: Vec<String>,
    pub incident_id: Option<String>,
}

impl Anomaly {
    pub fn new(
        kind: AnomalyKind,
        source_id: impl Into<String>,
        value: f64,
        baseline: f64,
        contributing_entries: Vec<String>,
    ) -> Self {
        let deviation_factor = if baseline > 0.0 { value / baseline } else { value };
        let severity = Severity::from_deviation(deviation_factor);
        Self {
            id: Uuid::new_v4().to_string(),
            detected_at: Utc::now(),
            kind,
            source_id: source_id.into(),
            severity,
            value,
            baseline,
            deviation_factor,
            contributing_entries,
            incident_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollingWindow<T: Clone> {
    pub window_size: Duration,
    pub items: VecDeque<(DateTime<Utc>, T)>,
}

impl<T: Clone> RollingWindow<T> {
    pub fn new(window_size: Duration) -> Self {
        Self {
            window_size,
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.items.push_back((Utc::now(), value));
        self.evict();
    }

    pub fn evict(&mut self) {
        let cutoff = Utc::now() - self.window_size;
        while self.items.front().map(|(t, _)| *t < cutoff).unwrap_or(false) {
            self.items.pop_front();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(DateTime<Utc>, T)> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl RollingWindow<f64> {
    pub fn ema(&self, alpha: f64) -> f64 {
        let mut ema = 0.0;
        let mut initialized = false;
        for (_, v) in &self.items {
            if !initialized {
                ema = *v;
                initialized = true;
            } else {
                ema = alpha * v + (1.0 - alpha) * ema;
            }
        }
        ema
    }

    pub fn mean(&self) -> f64 {
        if self.items.is_empty() {
            return 0.0;
        }
        self.items.iter().map(|(_, v)| v).sum::<f64>() / self.items.len() as f64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    pub window_seconds: u64,
    pub error_spike_threshold: f64,
    pub latency_jump_threshold: f64,
    pub memory_growth_threshold_mb_per_min: f64,
    pub min_samples_for_baseline: usize,
    pub incident_correlation_window_seconds: u64,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            window_seconds: 300,
            error_spike_threshold: 3.0,
            latency_jump_threshold: 2.5,
            memory_growth_threshold_mb_per_min: 5.0,
            min_samples_for_baseline: 10,
            incident_correlation_window_seconds: 120,
        }
    }
}
