use std::path::Path;
use br_core::config_inspector::{inspect_file, ConfigInspectionResult};
use crate::error::BrResult;

#[tauri::command]
pub async fn inspect_config_file(path: String) -> BrResult<ConfigInspectionResult> {
    let result = inspect_file(Path::new(&path)).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(result)
}
