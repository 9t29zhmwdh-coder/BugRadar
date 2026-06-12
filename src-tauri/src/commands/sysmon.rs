use tauri::{State, AppHandle, Emitter};
use tokio::time::{interval, Duration};
use br_core::sysmon::{SystemMetrics, ContainerStatus, DockerMonitor};

use crate::{error::BrResult, state::AppState};

#[tauri::command]
pub async fn get_system_metrics(state: State<'_, AppState>) -> BrResult<SystemMetrics> {
    let metrics = state.metrics.lock().await.collect();
    Ok(metrics)
}

#[tauri::command]
pub async fn get_container_statuses() -> BrResult<Vec<ContainerStatus>> {
    let monitor = DockerMonitor::new().map_err(|e| anyhow::anyhow!("{}", e))?;
    let statuses = monitor.get_container_statuses().await.map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(statuses)
}

#[tauri::command]
pub async fn start_metrics_polling(interval_ms: u64, app: AppHandle, state: State<'_, AppState>) -> BrResult<()> {
    let metrics_state = state.metrics.clone();
    let app_clone = app.clone();

    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(interval_ms));
        loop {
            ticker.tick().await;
            let snapshot = metrics_state.lock().await.collect();
            let _ = app_clone.emit("metrics://snapshot", &snapshot);
        }
    });

    Ok(())
}
