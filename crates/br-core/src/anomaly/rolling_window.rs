// Re-export from models: the generic RollingWindow lives in models/anomaly.rs
// This module provides source-level rolling windows per source_id
use std::collections::VecDeque;

use chrono::Duration;
use dashmap::DashMap;

use crate::models::anomaly::RollingWindow;
use crate::models::log_entry::LogLevel;

/// Custom detectors need to see real log text, not just level counts, but
/// keeping every message forever would defeat the point of a rolling window.
const RECENT_MESSAGES_CAP: usize = 50;

#[derive(Debug)]
pub struct SourceWindow {
    /// Error/Fatal count per second window
    pub error_window: RollingWindow<f64>,
    /// Warn count per second window
    pub warn_window: RollingWindow<f64>,
    /// Memory RSS samples (MB)
    pub memory_window: RollingWindow<f64>,
    /// Latency samples (ms), populated from structured fields
    pub latency_window: RollingWindow<f64>,
    /// Total entry count
    pub total_entries: u64,
    /// Error count last tick
    pub error_count_last_tick: u64,
    /// Most recent log messages, oldest first, capped at `RECENT_MESSAGES_CAP`.
    pub recent_messages: VecDeque<String>,
}

impl SourceWindow {
    pub fn new(window_seconds: u64) -> Self {
        let dur = Duration::seconds(window_seconds as i64);
        Self {
            error_window: RollingWindow::new(dur),
            warn_window: RollingWindow::new(dur),
            memory_window: RollingWindow::new(dur),
            latency_window: RollingWindow::new(dur),
            total_entries: 0,
            error_count_last_tick: 0,
            recent_messages: VecDeque::new(),
        }
    }

    pub fn record_entry_level(&mut self, level: &LogLevel) {
        self.total_entries += 1;
        match level {
            LogLevel::Error | LogLevel::Fatal => {
                self.error_count_last_tick += 1;
            }
            LogLevel::Warn => {
                self.warn_window.push(1.0);
            }
            _ => {}
        }
    }

    pub fn record_message(&mut self, message: &str) {
        self.recent_messages.push_back(message.to_string());
        if self.recent_messages.len() > RECENT_MESSAGES_CAP {
            self.recent_messages.pop_front();
        }
    }

    pub fn flush_tick(&mut self) {
        let count = self.error_count_last_tick as f64;
        if count > 0.0 {
            self.error_window.push(count);
        } else {
            self.error_window.push(0.0);
        }
        self.error_count_last_tick = 0;
        self.error_window.evict();
        self.warn_window.evict();
        self.memory_window.evict();
        self.latency_window.evict();
    }
}

pub type SourceWindows = DashMap<String, SourceWindow>;