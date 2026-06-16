use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::{
    log_entry::{LogEntry, LogLevel, WatchSource},
    anomaly::Anomaly,
    incident::{Incident, IncidentFilter, IncidentStatus},
    report::DiagnosticReport,
};

// --- WatchSources ---

pub async fn upsert_watch_source(pool: &SqlitePool, s: &WatchSource) -> Result<()> {
    let kind_json = serde_json::to_string(&s.kind)?;
    sqlx::query(
        "INSERT OR REPLACE INTO watch_sources (id, label, kind, parser_id, enabled, created_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&s.id)
    .bind(&s.label)
    .bind(&kind_json)
    .bind(&s.parser_id)
    .bind(s.enabled as i32)
    .bind(s.created_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_watch_sources(pool: &SqlitePool) -> Result<Vec<WatchSource>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, i32, String)>(
        "SELECT id, label, kind, parser_id, enabled, created_at FROM watch_sources"
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|(id, label, kind_json, parser_id, enabled, created_at)| {
        Ok(WatchSource {
            id,
            label,
            kind: serde_json::from_str(&kind_json)?,
            parser_id,
            enabled: enabled != 0,
            created_at: created_at.parse()?,
        })
    }).collect()
}

pub async fn delete_watch_source(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM watch_sources WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// --- LogEntries ---

pub async fn insert_log_entry(pool: &SqlitePool, entry: &LogEntry) -> Result<()> {
    let level_str = serde_json::to_string(&entry.level)?;
    let level_str = level_str.trim_matches('"').to_string();
    let stacktrace_json = entry.stacktrace.as_ref().map(serde_json::to_string).transpose()?;
    let fields_json = serde_json::to_string(&entry.fields)?;
    let raw_lines_json = serde_json::to_string(&entry.raw_lines)?;

    sqlx::query(
        "INSERT INTO log_entries
         (id, source_id, source_path, timestamp, level, message, stacktrace, fields, raw_lines, parser_id, ingested_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&entry.id)
    .bind(&entry.source_id)
    .bind(&entry.source_path)
    .bind(entry.timestamp.to_rfc3339())
    .bind(&level_str)
    .bind(&entry.message)
    .bind(&stacktrace_json)
    .bind(&fields_json)
    .bind(&raw_lines_json)
    .bind(&entry.parser_id)
    .bind(entry.ingested_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_recent_logs(pool: &SqlitePool, source_id: &str, limit: i64) -> Result<Vec<LogEntry>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, String, Option<String>, String, String, String, String)>(
        "SELECT id, source_id, source_path, timestamp, level, message, stacktrace, fields, raw_lines, parser_id, ingested_at
         FROM log_entries WHERE source_id = ?
         ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(source_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|(id, source_id, source_path, timestamp, level, message, stacktrace, fields, raw_lines, parser_id, ingested_at)| {
        Ok(LogEntry {
            id,
            source_id,
            source_path,
            timestamp: timestamp.parse()?,
            level: LogLevel::parse_level(&level),
            message,
            stacktrace: stacktrace.map(|s| serde_json::from_str(&s)).transpose()?,
            fields: serde_json::from_str(&fields)?,
            raw_lines: serde_json::from_str(&raw_lines)?,
            parser_id,
            ingested_at: ingested_at.parse()?,
        })
    }).collect()
}

pub async fn delete_old_logs(pool: &SqlitePool, older_than_days: i64) -> Result<u64> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(older_than_days);
    let result = sqlx::query("DELETE FROM log_entries WHERE ingested_at < ?")
        .bind(cutoff.to_rfc3339())
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

// --- Anomalies ---

pub async fn insert_anomaly(pool: &SqlitePool, a: &Anomaly) -> Result<()> {
    let kind_json = serde_json::to_string(&a.kind)?;
    let severity_str = serde_json::to_string(&a.severity)?.trim_matches('"').to_string();
    let entries_json = serde_json::to_string(&a.contributing_entries)?;

    sqlx::query(
        "INSERT INTO anomalies
         (id, detected_at, kind, source_id, severity, value, baseline, deviation_factor, contributing_entries, incident_id)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&a.id)
    .bind(a.detected_at.to_rfc3339())
    .bind(&kind_json)
    .bind(&a.source_id)
    .bind(&severity_str)
    .bind(a.value)
    .bind(a.baseline)
    .bind(a.deviation_factor)
    .bind(&entries_json)
    .bind(&a.incident_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn assign_anomaly_to_incident(pool: &SqlitePool, anomaly_id: &str, incident_id: &str) -> Result<()> {
    sqlx::query("UPDATE anomalies SET incident_id = ? WHERE id = ?")
        .bind(incident_id)
        .bind(anomaly_id)
        .execute(pool)
        .await?;
    Ok(())
}

// --- Incidents ---

pub async fn upsert_incident(pool: &SqlitePool, i: &Incident) -> Result<()> {
    let status_str = serde_json::to_string(&i.status)?.trim_matches('"').to_string();
    let severity_str = serde_json::to_string(&i.severity)?.trim_matches('"').to_string();
    let anomaly_ids_json = serde_json::to_string(&i.anomaly_ids)?;
    let source_ids_json = serde_json::to_string(&i.source_ids)?;
    let notes_json = serde_json::to_string(&i.notes)?;

    sqlx::query(
        "INSERT OR REPLACE INTO incidents
         (id, title, status, severity, anomaly_ids, source_ids, first_seen, last_seen, event_count, ai_analysis_id, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&i.id)
    .bind(&i.title)
    .bind(&status_str)
    .bind(&severity_str)
    .bind(&anomaly_ids_json)
    .bind(&source_ids_json)
    .bind(i.first_seen.to_rfc3339())
    .bind(i.last_seen.to_rfc3339())
    .bind(i.event_count as i64)
    .bind(&i.ai_analysis_id)
    .bind(&notes_json)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_incident(pool: &SqlitePool, id: &str) -> Result<Option<Incident>> {
    let row = sqlx::query_as::<_, (String, String, String, String, String, String, String, String, i64, Option<String>, String)>(
        "SELECT id, title, status, severity, anomaly_ids, source_ids, first_seen, last_seen, event_count, ai_analysis_id, notes
         FROM incidents WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.map(|(id, title, status, severity, anomaly_ids, source_ids, first_seen, last_seen, event_count, ai_analysis_id, notes)| {
        Ok(Incident {
            id,
            title,
            status: serde_json::from_str(&format!("\"{}\"", status))?,
            severity: serde_json::from_str(&format!("\"{}\"", severity))?,
            anomaly_ids: serde_json::from_str(&anomaly_ids)?,
            source_ids: serde_json::from_str(&source_ids)?,
            first_seen: first_seen.parse()?,
            last_seen: last_seen.parse()?,
            event_count: event_count as u64,
            ai_analysis_id,
            notes: serde_json::from_str(&notes)?,
        })
    }).transpose()
}

pub async fn list_incidents(pool: &SqlitePool, filter: &IncidentFilter) -> Result<Vec<Incident>> {
    let limit = filter.limit.unwrap_or(50) as i64;
    let offset = filter.offset.unwrap_or(0) as i64;

    let rows = sqlx::query_as::<_, (String, String, String, String, String, String, String, String, i64, Option<String>, String)>(
        "SELECT id, title, status, severity, anomaly_ids, source_ids, first_seen, last_seen, event_count, ai_analysis_id, notes
         FROM incidents
         ORDER BY last_seen DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|(id, title, status, severity, anomaly_ids, source_ids, first_seen, last_seen, event_count, ai_analysis_id, notes)| {
        Ok(Incident {
            id,
            title,
            status: serde_json::from_str(&format!("\"{}\"", status))?,
            severity: serde_json::from_str(&format!("\"{}\"", severity))?,
            anomaly_ids: serde_json::from_str(&anomaly_ids)?,
            source_ids: serde_json::from_str(&source_ids)?,
            first_seen: first_seen.parse()?,
            last_seen: last_seen.parse()?,
            event_count: event_count as u64,
            ai_analysis_id,
            notes: serde_json::from_str(&notes)?,
        })
    }).collect()
}

pub async fn update_incident_status(pool: &SqlitePool, id: &str, status: &IncidentStatus) -> Result<()> {
    let status_str = serde_json::to_string(status)?.trim_matches('"').to_string();
    sqlx::query("UPDATE incidents SET status = ? WHERE id = ?")
        .bind(&status_str)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_incident_ai_analysis(pool: &SqlitePool, incident_id: &str, report_id: &str) -> Result<()> {
    sqlx::query("UPDATE incidents SET ai_analysis_id = ? WHERE id = ?")
        .bind(report_id)
        .bind(incident_id)
        .execute(pool)
        .await?;
    Ok(())
}

// --- DiagnosticReports ---

pub async fn insert_report(pool: &SqlitePool, r: &DiagnosticReport) -> Result<()> {
    let factors_json = serde_json::to_string(&r.contributing_factors)?;
    let suggestions_json = serde_json::to_string(&r.fix_suggestions)?;
    let conflicts_json = serde_json::to_string(&r.config_conflicts)?;

    sqlx::query(
        "INSERT INTO diagnostic_reports
         (id, incident_id, created_at, summary, root_cause, contributing_factors, fix_suggestions, config_conflicts, confidence, ai_provider, model, tokens_used)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&r.id)
    .bind(&r.incident_id)
    .bind(r.created_at.to_rfc3339())
    .bind(&r.summary)
    .bind(&r.root_cause)
    .bind(&factors_json)
    .bind(&suggestions_json)
    .bind(&conflicts_json)
    .bind(r.confidence as f64)
    .bind(&r.ai_provider)
    .bind(&r.model)
    .bind(r.tokens_used.map(|t| t as i64))
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_report_by_id(pool: &SqlitePool, id: &str) -> Result<Option<DiagnosticReport>> {
    let row = sqlx::query_as::<_, (String, String, String, String, String, String, String, String, f64, String, String, Option<i64>)>(
        "SELECT id, incident_id, created_at, summary, root_cause, contributing_factors, fix_suggestions, config_conflicts, confidence, ai_provider, model, tokens_used
         FROM diagnostic_reports WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.map(|(id, incident_id, created_at, summary, root_cause, contributing_factors, fix_suggestions, config_conflicts, confidence, ai_provider, model, tokens_used)| {
        Ok(DiagnosticReport {
            id,
            incident_id,
            created_at: created_at.parse()?,
            summary,
            root_cause,
            contributing_factors: serde_json::from_str(&contributing_factors)?,
            fix_suggestions: serde_json::from_str(&fix_suggestions)?,
            config_conflicts: serde_json::from_str(&config_conflicts)?,
            confidence: confidence as f32,
            ai_provider,
            model,
            tokens_used: tokens_used.map(|t| t as u32),
        })
    }).transpose()
}
