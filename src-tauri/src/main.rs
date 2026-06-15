#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod state;
mod commands;

use std::path::PathBuf;

use tauri::{Manager, AppHandle, Emitter};
use tracing::info;

use br_core::db::Database;
use state::AppState;

fn app_data_dir(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let db_path = app_data_dir(&handle).join("bugradar.sqlite");

            tauri::async_runtime::block_on(async move {
                let db = Database::open(&db_path).await.expect("Failed to open database");
                let mut state = AppState::new(db);

                // Wire up log collector → anomaly engine
                let log_rx = {
                    let mut collector = state.collector.lock().await;
                    collector.take_receiver()
                };

                if let Some(rx) = log_rx {
                    let anomaly_rx_opt = {
                        let mut eng = state.anomaly_engine.lock().await;
                        eng.spawn(rx);
                        eng.take_receiver()
                    };

                    // Forward anomaly events to frontend
                    if let Some(mut anomaly_rx) = anomaly_rx_opt {
                        let handle_clone = handle.clone();
                        let db_pool = state.db.pool.clone();

                        tokio::spawn(async move {
                            while let Some(event) = anomaly_rx.recv().await {
                                let source_id = event.anomaly.source_id.clone();
                                let incident_id = event.incident.id.clone();
                                let is_new = event.incident_is_new;

                                let _ = br_core::db::queries::insert_anomaly(&db_pool, &event.anomaly).await;
                                let _ = br_core::db::queries::upsert_incident(&db_pool, &event.incident).await;

                                let _ = handle_clone.emit(&format!("anomaly://detected/{}", source_id), &event.anomaly);
                                if is_new {
                                    let _ = handle_clone.emit("incident://created", &event.incident);
                                } else {
                                    let _ = handle_clone.emit(&format!("incident://updated/{}", incident_id), &event.incident);
                                }
                            }
                        });
                    }
                }

                handle.manage(state);
                info!("BugRadar initialized");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::save_settings,
            commands::check_ai_backend,
            commands::save_api_key,
            commands::has_api_key,
            commands::collector::watch_source,
            commands::collector::stop_watching,
            commands::collector::list_watch_sources,
            commands::collector::get_recent_logs,
            commands::incidents::list_incidents,
            commands::incidents::get_incident,
            commands::incidents::update_incident_status,
            commands::incidents::add_incident_note,
            commands::anomaly::get_anomaly_config,
            commands::anomaly::save_anomaly_config,
            commands::sysmon::get_system_metrics,
            commands::sysmon::get_container_statuses,
            commands::sysmon::start_metrics_polling,
            commands::config_inspect::inspect_config_file,
            commands::ai::trigger_ai_analysis,
            commands::ai::get_diagnostic_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
