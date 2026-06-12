use chrono::Utc;

use crate::models::log_entry::{LogEntry, LogLevel};
use crate::plugin::{LogParserPlugin, PluginFactory, PluginMetadata};

// Parses Docker JSON log envelope: {"log":"...","stream":"stdout","time":"..."}
pub struct DockerParser {
    source_id: String,
    source_path: String,
}

impl DockerParser {
    fn new(source_id: &str, source_path: &str) -> Self {
        Self {
            source_id: source_id.to_string(),
            source_path: source_path.to_string(),
        }
    }

    fn parse_inner_level(message: &str) -> LogLevel {
        let lower = message.to_lowercase();
        if lower.contains("fatal") || lower.contains("critical") {
            LogLevel::Fatal
        } else if lower.contains("error") || lower.contains("err]") || lower.contains("err:") {
            LogLevel::Error
        } else if lower.contains("warn") {
            LogLevel::Warn
        } else if lower.contains("debug") {
            LogLevel::Debug
        } else {
            LogLevel::Info
        }
    }
}

impl LogParserPlugin for DockerParser {
    fn id(&self) -> &str { "docker" }

    fn can_handle(&self, source_path: &str) -> bool {
        source_path.contains("docker") || source_path.ends_with("-json.log")
    }

    fn push_line(&mut self, line: &str) -> Option<LogEntry> {
        if line.trim().is_empty() {
            return None;
        }

        let (message, level) = if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            let msg = v["log"].as_str().unwrap_or(line).trim_end_matches('\n').to_string();
            let level = Self::parse_inner_level(&msg);
            (msg, level)
        } else {
            let level = Self::parse_inner_level(line);
            (line.to_string(), level)
        };

        let mut entry = LogEntry::new(
            &self.source_id,
            &self.source_path,
            Utc::now(),
            level,
            message,
            "docker",
        );
        entry.raw_lines = vec![line.to_string()];
        Some(entry)
    }

    fn flush(&mut self) -> Option<LogEntry> { None }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "docker".to_string(),
            name: "Docker".to_string(),
            description: "Parses Docker JSON log envelope format".to_string(),
            file_patterns: vec!["*-json.log".to_string(), "*docker*".to_string()],
        }
    }
}

pub struct DockerPluginFactory;

impl PluginFactory for DockerPluginFactory {
    fn id(&self) -> &str { "docker" }

    fn create(&self, source_id: &str, source_path: &str) -> Box<dyn LogParserPlugin> {
        Box::new(DockerParser::new(source_id, source_path))
    }
}
