use chrono::Utc;

use crate::models::log_entry::{LogEntry, LogLevel};
use crate::plugin::{LogParserPlugin, PluginFactory, PluginMetadata};

/// Parses JSON-structured log lines (one JSON object per line)
pub struct JsonParser {
    source_id: String,
    source_path: String,
}

impl JsonParser {
    pub fn new(source_id: &str, source_path: &str) -> Self {
        Self {
            source_id: source_id.to_string(),
            source_path: source_path.to_string(),
        }
    }

    fn extract_level(v: &serde_json::Value) -> LogLevel {
        let level_str = v.get("level")
            .or_else(|| v.get("severity"))
            .or_else(|| v.get("lvl"))
            .and_then(|l| l.as_str())
            .unwrap_or("info");
        LogLevel::from_str(level_str)
    }

    fn extract_message(v: &serde_json::Value) -> String {
        v.get("message")
            .or_else(|| v.get("msg"))
            .or_else(|| v.get("text"))
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn extract_timestamp(v: &serde_json::Value) -> chrono::DateTime<Utc> {
        let ts = v.get("timestamp")
            .or_else(|| v.get("time")
            .or_else(|| v.get("@timestamp")))
            .and_then(|t| t.as_str());

        ts.and_then(|s| s.parse().ok()).unwrap_or_else(Utc::now)
    }
}

impl LogParserPlugin for JsonParser {
    fn id(&self) -> &str { "json" }

    fn can_handle(&self, source_path: &str) -> bool {
        source_path.ends_with(".json") || source_path.ends_with(".jsonl") || source_path.ends_with(".ndjson")
    }

    fn push_line(&mut self, line: &str) -> Option<LogEntry> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        let v: serde_json::Value = serde_json::from_str(trimmed).ok()?;

        let level = Self::extract_level(&v);
        let message = Self::extract_message(&v);
        let timestamp = Self::extract_timestamp(&v);

        let mut entry = LogEntry::new(
            &self.source_id,
            &self.source_path,
            timestamp,
            level,
            message,
            "json",
        );

        // Store remaining fields
        if let serde_json::Value::Object(map) = &v {
            let filtered: serde_json::Map<_, _> = map
                .iter()
                .filter(|(k, _)| !matches!(k.as_str(), "message" | "msg" | "level" | "severity" | "timestamp" | "time"))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            entry.fields = serde_json::Value::Object(filtered);
        }

        entry.raw_lines = vec![line.to_string()];
        Some(entry)
    }

    fn flush(&mut self) -> Option<LogEntry> { None }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "json".to_string(),
            name: "JSON".to_string(),
            description: "Parses structured JSON log lines (one object per line)".to_string(),
            file_patterns: vec!["*.json".to_string(), "*.jsonl".to_string(), "*.ndjson".to_string()],
        }
    }
}

pub struct JsonPluginFactory;

impl PluginFactory for JsonPluginFactory {
    fn id(&self) -> &str { "json" }

    fn create(&self, source_id: &str, source_path: &str) -> Box<dyn LogParserPlugin> {
        Box::new(JsonParser::new(source_id, source_path))
    }
}
