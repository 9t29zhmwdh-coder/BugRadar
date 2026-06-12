use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Unknown,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "trace" => Self::Trace,
            "debug" | "dbg" => Self::Debug,
            "info" | "information" => Self::Info,
            "warn" | "warning" => Self::Warn,
            "error" | "err" => Self::Error,
            "fatal" | "critical" | "crit" => Self::Fatal,
            _ => Self::Unknown,
        }
    }

    pub fn severity_score(&self) -> u8 {
        match self {
            Self::Trace => 0,
            Self::Debug => 1,
            Self::Info => 2,
            Self::Warn => 3,
            Self::Error => 4,
            Self::Fatal => 5,
            Self::Unknown => 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchSourceKind {
    FilePath { path: String },
    DockerContainer { container_id: String, container_name: String },
    DockerAllContainers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchSource {
    pub id: String,
    pub label: String,
    pub kind: WatchSourceKind,
    pub parser_id: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

impl WatchSource {
    pub fn new(label: impl Into<String>, kind: WatchSourceKind, parser_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            label: label.into(),
            kind,
            parser_id: parser_id.into(),
            enabled: true,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub source_id: String,
    pub source_path: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub stacktrace: Option<Vec<String>>,
    pub fields: serde_json::Value,
    pub raw_lines: Vec<String>,
    pub parser_id: String,
    pub ingested_at: DateTime<Utc>,
}

impl LogEntry {
    pub fn new(
        source_id: impl Into<String>,
        source_path: impl Into<String>,
        timestamp: DateTime<Utc>,
        level: LogLevel,
        message: impl Into<String>,
        parser_id: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_id: source_id.into(),
            source_path: source_path.into(),
            timestamp,
            level,
            message: message.into(),
            stacktrace: None,
            fields: serde_json::Value::Object(Default::default()),
            raw_lines: Vec::new(),
            parser_id: parser_id.into(),
            ingested_at: Utc::now(),
        }
    }
}
