pub mod collector;
pub mod incidents;
pub mod anomaly;
pub mod sysmon;
pub mod config_inspect;
pub mod ai;

use tauri::State;
use crate::{error::BrResult, state::AppState};
use br_core::anomaly::PluginDetectorConfig;
use br_core::models::anomaly::AnomalyConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub ai_provider: String,
    pub ollama_host: String,
    pub ollama_model: String,
    pub log_retention_days: i64,
    pub anomaly_config: AnomalyConfig,
    #[serde(default)]
    pub custom_detectors: Vec<PluginDetectorConfig>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_provider: "claude".to_string(),
            ollama_host: "http://localhost:11434".to_string(),
            ollama_model: "llama3.2".to_string(),
            log_retention_days: 7,
            anomaly_config: AnomalyConfig::default(),
            custom_detectors: Vec::new(),
        }
    }
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> BrResult<AppSettings> {
    let raw = state.db.get_setting("app_settings").await?;
    let settings = raw
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    Ok(settings)
}

#[tauri::command]
pub async fn save_settings(settings: AppSettings, state: State<'_, AppState>) -> BrResult<()> {
    let json = serde_json::to_string(&settings)?;
    state.db.set_setting("app_settings", &json).await?;
    Ok(())
}

#[tauri::command]
pub async fn check_ai_backend(provider: String, state: State<'_, AppState>) -> BrResult<bool> {
    let settings = get_settings(state.clone()).await?;
    match provider.as_str() {
        "claude" => {
            let key = get_api_key_internal().await;
            Ok(key.map(|k| !k.is_empty()).unwrap_or(false))
        }
        "ollama" => {
            let client = reqwest::Client::new();
            let url = format!("{}/api/tags", settings.ollama_host);
            Ok(client.get(&url).send().await.map(|r| r.status().is_success()).unwrap_or(false))
        }
        _ => Ok(false),
    }
}

#[tauri::command]
pub async fn save_api_key(key: String) -> BrResult<()> {
    use keyring::Entry;
    let entry = Entry::new("BugRadar", "claude_api_key").map_err(|e| anyhow::anyhow!("{}", e))?;
    entry.set_password(&key).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn has_api_key() -> BrResult<bool> {
    Ok(get_api_key_internal().await.map(|k| !k.is_empty()).unwrap_or(false))
}

pub async fn get_api_key_internal() -> Option<String> {
    use keyring::Entry;
    let entry = Entry::new("BugRadar", "claude_api_key").ok()?;
    entry.get_password().ok()
}
