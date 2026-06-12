use chrono::Utc;
use regex::Regex;

use crate::models::log_entry::{LogEntry, LogLevel};
use crate::plugin::{LogParserPlugin, PluginFactory, PluginMetadata};

// Parses nginx combined log format:
// 127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.1" 200 2326
pub struct NginxParser {
    source_id: String,
    source_path: String,
    re: Regex,
}

impl NginxParser {
    fn new(source_id: &str, source_path: &str) -> Self {
        let pattern = r#"^(\S+)\s+\S+\s+\S+\s+\[([^\]]+)\]\s+"([^"]+)"\s+(\d+)\s+(\d+|-)"#;
        Self {
            source_id: source_id.to_string(),
            source_path: source_path.to_string(),
            re: Regex::new(pattern).unwrap(),
        }
    }
}

impl LogParserPlugin for NginxParser {
    fn id(&self) -> &str { "nginx" }

    fn can_handle(&self, source_path: &str) -> bool {
        let p = source_path.to_lowercase();
        p.contains("nginx") || p.ends_with("access.log") || p.ends_with("error.log")
    }

    fn push_line(&mut self, line: &str) -> Option<LogEntry> {
        if line.trim().is_empty() {
            return None;
        }

        let (level, message) = if let Some(caps) = self.re.captures(line) {
            let status: u16 = caps.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            let level = match status {
                500..=599 => LogLevel::Error,
                400..=499 => LogLevel::Warn,
                _ => LogLevel::Info,
            };
            let request = caps.get(3).map(|m| m.as_str()).unwrap_or(line);
            (level, format!("[{}] {}", status, request))
        } else {
            (LogLevel::Info, line.to_string())
        };

        let mut entry = LogEntry::new(
            &self.source_id,
            &self.source_path,
            Utc::now(),
            level,
            message,
            "nginx",
        );
        entry.raw_lines = vec![line.to_string()];
        Some(entry)
    }

    fn flush(&mut self) -> Option<LogEntry> { None }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "nginx".to_string(),
            name: "Nginx".to_string(),
            description: "Parses nginx combined access and error log format".to_string(),
            file_patterns: vec!["*nginx*".to_string(), "*access.log".to_string()],
        }
    }
}

pub struct NginxPluginFactory;

impl PluginFactory for NginxPluginFactory {
    fn id(&self) -> &str { "nginx" }

    fn create(&self, source_id: &str, source_path: &str) -> Box<dyn LogParserPlugin> {
        Box::new(NginxParser::new(source_id, source_path))
    }
}
