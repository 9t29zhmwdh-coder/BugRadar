use tauri::State;
use br_core::models::anomaly::{Anomaly, AnomalyConfig};
use br_core::db::queries;

use crate::{error::BrResult, state::AppState};
use super::{get_settings, save_settings};

#[tauri::command]
pub async fn get_anomalies(source_id: Option<String>, limit: Option<i64>, state: State<'_, AppState>) -> BrResult<Vec<Anomaly>> {
    let limit = limit.unwrap_or(100);
    let sid = source_id.as_deref().unwrap_or("");
    let anomalies = queries::get_recent_logs(&state.db.pool, sid, limit)
        .await
        .map(|_| vec![])?; // anomalies are in a separate table
    // TODO: add get_anomalies query
    Ok(vec![])
}

#[tauri::command]
pub async fn get_anomaly_config(state: State<'_, AppState>) -> BrResult<AnomalyConfig> {
    let settings = get_settings(state).await?;
    Ok(settings.anomaly_config)
}

#[tauri::command]
pub async fn save_anomaly_config(config: AnomalyConfig, state: State<'_, AppState>) -> BrResult<()> {
    let mut settings = get_settings(state.clone()).await?;
    settings.anomaly_config = config;
    save_settings(settings, state).await
}
