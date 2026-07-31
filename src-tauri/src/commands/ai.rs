use tauri::{State, AppHandle, Emitter};
use br_core::ai::{AiAnalyzer, claude::ClaudeAnalyzer, ollama::OllamaAnalyzer, prompts::IncidentContext};
use br_core::db::queries;
use br_core::models::report::DiagnosticReport;

use crate::{error::BrResult, state::AppState};
use super::{get_api_key_internal, get_settings};

#[tauri::command]
pub async fn trigger_ai_analysis(incident_id: String, app: AppHandle, state: State<'_, AppState>) -> BrResult<String> {
    let incident = queries::get_incident(&state.db.pool, &incident_id)
        .await?
        .ok_or_else(|| crate::error::BrError::NotFound(incident_id.clone()))?;

    let settings = get_settings(state.clone()).await?;

    let _ = app.emit(&format!("ai://analysis/started/{}", incident_id), ());

    // Build context
    let recent_errors = queries::get_recent_logs(&state.db.pool, &incident.source_ids.first().cloned().unwrap_or_default(), 20).await?;

    let ctx = IncidentContext {
        incident: &incident,
        anomalies: &[],  // TODO: load actual anomalies
        recent_errors: &recent_errors,
        system_context: None,
    };

    let analyzer: Box<dyn AiAnalyzer> = match settings.ai_provider.as_str() {
        "ollama" => Box::new(OllamaAnalyzer::new(&settings.ollama_host, &settings.ollama_model)),
        _ => {
            // Ohne Schluessel gar nicht erst senden. Vorher baute dieser Zweig
            // den Analyzer mit einem leeren Schluessel, die Anfrage ging samt
            // Logzeilen an die API und scheiterte erst dort an der Anmeldung.
            let key = get_api_key_internal().await.unwrap_or_default();
            if key.is_empty() {
                return Err(crate::error::BrError::Ai(
                    "No Claude API key configured. Set one in settings, or switch the AI provider to Ollama.".to_string(),
                ));
            }
            Box::new(ClaudeAnalyzer::new(key))
        }
    };

    let report = analyzer.analyze(&ctx).await.map_err(|e| crate::error::BrError::Ai(e.to_string()))?;
    let report_id = report.id.clone();

    queries::insert_report(&state.db.pool, &report).await?;
    queries::set_incident_ai_analysis(&state.db.pool, &incident_id, &report_id).await?;

    let _ = app.emit(&format!("ai://analysis/done/{}", incident_id), &report);

    Ok(report_id)
}

#[tauri::command]
pub async fn get_diagnostic_report(id: String, state: State<'_, AppState>) -> BrResult<Option<DiagnosticReport>> {
    let report = queries::get_report_by_id(&state.db.pool, &id).await?;
    Ok(report)
}
