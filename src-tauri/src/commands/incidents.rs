use tauri::State;
use br_core::models::incident::{Incident, IncidentFilter, IncidentStatus};
use br_core::db::queries;

use crate::{error::BrResult, state::AppState};

#[tauri::command]
pub async fn list_incidents(filter: Option<IncidentFilter>, state: State<'_, AppState>) -> BrResult<Vec<Incident>> {
    let filter = filter.unwrap_or_default();
    let incidents = queries::list_incidents(&state.db.pool, &filter).await?;
    Ok(incidents)
}

#[tauri::command]
pub async fn get_incident(id: String, state: State<'_, AppState>) -> BrResult<Option<Incident>> {
    let incident = queries::get_incident(&state.db.pool, &id).await?;
    Ok(incident)
}

#[tauri::command]
pub async fn update_incident_status(id: String, status: IncidentStatus, state: State<'_, AppState>) -> BrResult<()> {
    queries::update_incident_status(&state.db.pool, &id, &status).await?;
    Ok(())
}

#[tauri::command]
pub async fn add_incident_note(id: String, note: String, state: State<'_, AppState>) -> BrResult<()> {
    if let Some(mut incident) = queries::get_incident(&state.db.pool, &id).await? {
        incident.add_note(note);
        queries::upsert_incident(&state.db.pool, &incident).await?;
    }
    Ok(())
}
