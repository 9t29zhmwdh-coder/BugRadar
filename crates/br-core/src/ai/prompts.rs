use crate::models::anomaly::Anomaly;
use crate::models::incident::Incident;
use crate::models::log_entry::LogEntry;

pub struct IncidentContext<'a> {
    pub incident: &'a Incident,
    pub anomalies: &'a [Anomaly],
    pub recent_errors: &'a [LogEntry],
    pub system_context: Option<String>,
}

pub fn build_analysis_prompt(ctx: &IncidentContext) -> String {
    let anomaly_summary = ctx.anomalies.iter()
        .map(|a| format!(
            "- {} (severity: {:?}, value: {:.2}, baseline: {:.2}, factor: {:.1}x)",
            a.kind.label(), a.severity, a.value, a.baseline, a.deviation_factor
        ))
        .collect::<Vec<_>>()
        .join("\n");

    let error_log_sample = ctx.recent_errors.iter().take(10)
        .map(|e| {
            let stacktrace = e.stacktrace.as_ref()
                .map(|s| format!("\n  Stacktrace:\n    {}", s.join("\n    ")))
                .unwrap_or_default();
            format!("[{:?}] {}{}", e.level, e.message, stacktrace)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let sys_ctx = ctx.system_context.as_deref().unwrap_or("N/A");

    format!(r#"You are an expert software engineer and DevOps specialist performing root-cause analysis.

## Incident: {}
- Status: {:?}
- Severity: {:?}
- First seen: {}
- Event count: {}

## Detected Anomalies
{}

## Recent Error Logs (sample)
{}

## System Context
{}

## Task
Analyze this incident and provide a structured JSON response with:
1. `summary` - One paragraph explaining what is happening
2. `root_cause` - Most likely root cause (specific, actionable)
3. `contributing_factors` - Array of strings, secondary contributing factors
4. `fix_suggestions` - Array of fix objects, each with:
   - `priority` (1=highest)
   - `title`
   - `description`
   - `command` (optional shell command to run)
   - `code_snippet` (optional: `{{language, filename, content}}`)
5. `config_conflicts` - Array of config objects if any (each: `{{file_path, key, current_value, suggested_value, reason}}`)
6. `confidence` - Float 0.0-1.0

Respond ONLY with valid JSON, no markdown, no explanation outside the JSON."#,
        ctx.incident.title,
        ctx.incident.status,
        ctx.incident.severity,
        ctx.incident.first_seen.format("%Y-%m-%d %H:%M:%S UTC"),
        ctx.incident.event_count,
        anomaly_summary,
        error_log_sample,
        sys_ctx,
    )
}
