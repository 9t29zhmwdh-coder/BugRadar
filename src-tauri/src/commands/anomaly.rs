use tauri::State;
use br_core::models::anomaly::AnomalyConfig;

use crate::{error::BrResult, state::AppState};
use super::{get_settings, save_settings};

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
