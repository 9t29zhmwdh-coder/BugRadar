use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

use crate::models::anomaly::{Anomaly, AnomalyConfig, Severity};
use crate::models::incident::Incident;

pub struct IncidentGrouper {
    /// Open incidents indexed by id
    open_incidents: HashMap<String, Incident>,
    config: AnomalyConfig,
}

impl IncidentGrouper {
    pub fn new(config: AnomalyConfig) -> Self {
        Self {
            open_incidents: HashMap::new(),
            config,
        }
    }

    /// Returns (updated_or_new_incident, is_new)
    pub fn process_anomaly(&mut self, anomaly: &Anomaly) -> (Incident, bool) {
        let correlation_window = Duration::seconds(self.config.incident_correlation_window_seconds as i64);
        let now = Utc::now();

        // Find best matching open incident:
        // same source_id OR same anomaly kind, AND last_seen within correlation window
        let matching_id = self.open_incidents
            .iter()
            .filter(|(_, i)| {
                i.last_seen + correlation_window >= now
                    && (i.source_ids.contains(&anomaly.source_id)
                        || i.anomaly_ids.is_empty())  // fresh incident always matches
            }))
            .max_by_key(|(_, i)| i.last_seen)
            .map(|(id, _)| id.clone());

        if let Some(incident_id) = matching_id {
            let incident = self.open_incidents.get_mut(&incident_id).unwrap();
            incident.add_anomaly(&anomaly.id);
            if !incident.source_ids.contains(&anomaly.source_id) {
                incident.source_ids.push(anomaly.source_id.clone());
            }
            // Escalate severity if anomaly is worse
            if anomaly.severity > incident.severity {
                incident.severity = anomaly.severity;
            }
            (incident.clone(), false)
        } else {
            // Create new incident
            let title = Self::generate_title(anomaly);
            let mut incident = Incident::new(title, anomaly.severity, vec![anomaly.source_id.clone()]);
            incident.add_anomaly(&anomaly.id);
            self.open_incidents.insert(incident.id.clone(), incident.clone());
            (incident, true)
        }
    }

    fn generate_title(anomaly: &Anomaly) -> String {
        format!(
            "{} detected ({}x baseline) on {}",
            anomaly.kind.label(),
            format!("{:.1}", anomaly.deviation_factor),
            &anomaly.source_id[..anomaly.source_id.len().min(20)]
        )
    }

    pub fn resolve_stale_incidents(&mut self, max_age_seconds: u64) -> Vec<Incident> {
        let cutoff = Utc::now() - Duration::seconds(max_age_seconds as i64);
        let stale_ids: Vec<String> = self.open_incidents
            .iter()
            .filter(|(_, i)| i.last_seen < cutoff)
            .map(|(id, _)| id.clone())
            .collect();

        stale_ids.into_iter()
            .filter_map(|id| self.open_incidents.remove(&id))
            .collect()
    }

    pub fn get_open_incident_count(&self) -> usize {
        self.open_incidents.len()
    }
}
