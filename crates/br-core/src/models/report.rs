use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub filename: Option<String>,
    pub content: String,
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixSuggestion {
    pub priority: u8,
    pub title: String,
    pub description: String,
    pub code_snippet: Option<CodeSnippet>,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigConflict {
    pub file_path: String,
    pub key: String,
    pub current_value: String,
    pub suggested_value: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub id: String,
    pub incident_id: String,
    pub created_at: DateTime<Utc>,
    pub summary: String,
    pub root_cause: String,
    pub contributing_factors: Vec<String>,
    pub fix_suggestions: Vec<FixSuggestion>,
    pub config_conflicts: Vec<ConfigConflict>,
    pub confidence: f32,
    pub ai_provider: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

impl DiagnosticReport {
    pub fn new(incident_id: impl Into<String>, ai_provider: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            incident_id: incident_id.into(),
            created_at: Utc::now(),
            summary: String::new(),
            root_cause: String::new(),
            contributing_factors: Vec::new(),
            fix_suggestions: Vec::new(),
            config_conflicts: Vec::new(),
            confidence: 0.0,
            ai_provider: ai_provider.into(),
            model: model.into(),
            tokens_used: None,
        }
    }
}
