use tauri::State;
use br_core::models::log_entry::{WatchSource, LogEntry};
use br_core::db::queries;

use crate::{error::BrResult, state::AppState};

#[tauri::command]
pub async fn watch_source(source: WatchSource, state: State<'_, AppState>) -> BrResult<String> {
    let source_id = source.id.clone();

    queries::upsert_watch_source(&state.db.pool, &source).await?;

    let collector = state.collector.lock().await;
    collector.start_watching(&source);

    Ok(source_id)
}

#[tauri::command]
pub async fn stop_watching(source_id: String, state: State<'_, AppState>) -> BrResult<()> {
    let collector = state.collector.lock().await;
    collector.stop_watching(&source_id);
    queries::delete_watch_source(&state.db.pool, &source_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_watch_sources(state: State<'_, AppState>) -> BrResult<Vec<WatchSource>> {
    let sources = queries::list_watch_sources(&state.db.pool).await?;
    Ok(sources)
}

#[tauri::command]
pub async fn get_recent_logs(source_id: String, limit: i64, state: State<'_, AppState>) -> BrResult<Vec<LogEntry>> {
    let entries = queries::get_recent_logs(&state.db.pool, &source_id, limit).await?;
    Ok(entries)
}
