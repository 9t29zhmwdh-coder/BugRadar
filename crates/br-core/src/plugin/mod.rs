pub mod registry;
pub mod builtin;

use crate::models::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub file_patterns: Vec<String>,
}

pub trait LogParserPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn can_handle(&self, source_path: &str) -> bool;
    /// Returns a completed LogEntry if a full entry is ready, None if line is a continuation (stacktrace etc.)
    fn push_line(&mut self, line: &str) -> Option<LogEntry>;
    /// Flush any buffered partial entry (on source close or EOF)
    fn flush(&mut self) -> Option<LogEntry>;
    fn metadata(&self) -> PluginMetadata;
}

pub trait PluginFactory: Send + Sync {
    fn id(&self) -> &str;
    /// Creates a fresh parser instance per WatchSource
    fn create(&self, source_id: &str, source_path: &str) -> Box<dyn LogParserPlugin>;
}
