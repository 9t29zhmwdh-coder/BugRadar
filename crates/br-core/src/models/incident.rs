use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::anomaly::Severity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Open,
    Investigating,
    Resolved,
    Suppressed,
}

impl Default for IncidentStatus {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentNote {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub status: IncidentStatus,
    pub severity: Severity,
    pub anomaly_ids: Vec<String>,
    pub source_ids: Vec<String>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub event_count: u64,
    pub ai_analysis_id: Option<String>,
    pub notes: Vec<IncidentNote>,
}

impl Incident {
    pub fn new(title: impl Into<String>, severity: Severity, source_ids: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            status: IncidentStatus::Open,
            severity,
            anomaly_ids: Vec::new(),
            source_ids,
            first_seen: now,
            last_seen: now,
            event_count: 1,
            ai_analysis_id: None,
            notes: Vec::new(),
        }
    }

    pub fn add_anomaly(&mut self, anomaly_id: &str) {
        if !self.anomaly_ids.contains(&anomaly_id.to_string()) {
            self.anomaly_ids.push(anomaly_id.to_string());
        }
        self.last_seen = Utc::now();
        self.event_count += 1;
    }

    pub fn add_note(&mut self, text: impl Into<String>) {
        self.notes.push(IncidentNote {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            text: text.into(),
        });
    }

    pub fn should_trigger_ai(&self) -> bool {
        self.severity >= Severity::High && self.anomaly_ids.len() >= 3 && self.ai_analysis_id.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentFilter {
    pub status: Option<Vec<IncidentStatus>>,
    pub severity: Option<Vec<Severity>>,
    pub source_id: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for IncidentFilter {
    fn default() -> Self {
        Self {
            status: None,
            severity: None,
            source_id: None,
            limit: Some(50),
            offset: Some(0),
        }
    }
}
