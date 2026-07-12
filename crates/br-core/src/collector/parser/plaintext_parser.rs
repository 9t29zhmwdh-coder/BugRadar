use chrono::{DateTime, Utc};
use regex::Regex;

use crate::models::log_entry::{LogEntry, LogLevel};
use crate::plugin::{LogParserPlugin, PluginFactory, PluginMetadata};
use super::stacktrace_merger::StacktraceMerger;

/// Parses plain text log lines with common timestamp+level prefixes.
/// Supports multi-line stacktrace merging.
pub struct PlaintextParser {
    source_id: String,
    source_path: String,
    timestamp_re: Regex,
    level_re: Regex,
    merger: StacktraceMerger,
    pending: Option<PendingEntry>,
}

struct PendingEntry {
    base: LogEntry,
    stacktrace_lines: Vec<String>,
}

impl PlaintextParser {
    pub fn new(source_id: &str, source_path: &str) -> Self {
        Self {
            source_id: source_id.to_string(),
            source_path: source_path.to_string(),
            // Matches: 2024-01-15T12:34:56 or 2024-01-15 12:34:56 or [2024-01-15 12:34:56]
            timestamp_re: Regex::new(
                r"^[\[]*(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?)"
            ).unwrap(),
            // Matches: [INFO], INFO:, INFO , WARN, ERROR etc.
            level_re: Regex::new(
                r"(?i)\b(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|FATAL|CRITICAL)\b"
            ).unwrap(),
            merger: StacktraceMerger::new(),
            pending: None,
        }
    }

    fn parse_timestamp(&self, line: &str) -> Option<DateTime<Utc>> {
        let caps = self.timestamp_re.captures(line)?;
        let ts_str = caps.get(1)?.as_str();
        ts_str.parse::<DateTime<Utc>>().ok()
            .or_else(|| {
                // Try with local datetime
                chrono::NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%dT%H:%M:%S%.f")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S"))
                    .ok()
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
            })
    }

    fn parse_level(&self, line: &str) -> LogLevel {
        self.level_re.find(line)
            .map(|m| LogLevel::parse_level(m.as_str()))
            .unwrap_or(LogLevel::Info)
    }

    fn finalize_pending(&mut self) -> Option<LogEntry> {
        let pending = self.pending.take()?;
        let mut entry = pending.base;
        if !pending.stacktrace_lines.is_empty() {
            entry.stacktrace = Some(pending.stacktrace_lines);
        }
        Some(entry)
    }
}

impl LogParserPlugin for PlaintextParser {
    fn id(&self) -> &str { "plaintext" }

    fn can_handle(&self, source_path: &str) -> bool {
        source_path.ends_with(".log") || source_path.ends_with(".txt")
    }

    fn push_line(&mut self, line: &str) -> Option<LogEntry> {
        if line.trim().is_empty() {
            return self.finalize_pending();
        }

        if self.merger.is_continuation(line) {
            if let Some(ref mut p) = self.pending {
                p.stacktrace_lines.push(line.to_string());
                p.base.raw_lines.push(line.to_string());
            }
            return None;
        }

        // New entry: finalize any pending one first
        let finished = self.finalize_pending();

        let timestamp = self.parse_timestamp(line).unwrap_or_else(Utc::now);
        let level = self.parse_level(line);

        // Strip timestamp prefix from message
        let message = self.timestamp_re.replace(line, "").trim().to_string();
        let message = self.level_re.replace(&message, "").trim_start_matches([':', ' ', ']', '[']).trim().to_string();

        let mut entry = LogEntry::new(
            &self.source_id,
            &self.source_path,
            timestamp,
            level,
            if message.is_empty() { line.to_string() } else { message },
            "plaintext",
        );
        entry.raw_lines = vec![line.to_string()];

        self.pending = Some(PendingEntry {
            base: entry,
            stacktrace_lines: Vec::new(),
        });

        finished
    }

    fn flush(&mut self) -> Option<LogEntry> {
        self.finalize_pending()
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "plaintext".to_string(),
            name: "Plaintext".to_string(),
            description: "Parses plain text log files with timestamp+level prefix, supports stacktrace merging".to_string(),
            file_patterns: vec!["*.log".to_string(), "*.txt".to_string()],
        }
    }
}

pub struct PlaintextPluginFactory;

impl PluginFactory for PlaintextPluginFactory {
    fn id(&self) -> &str { "plaintext" }

    fn create(&self, source_id: &str, source_path: &str) -> Box<dyn LogParserPlugin> {
        Box::new(PlaintextParser::new(source_id, source_path))
    }
}