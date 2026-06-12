pub mod yaml_inspector;
pub mod json_inspector;
pub mod toml_inspector;

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigIssue {
    pub severity: String,
    pub key: String,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInspectionResult {
    pub file_path: String,
    pub format: String,
    pub issues: Vec<ConfigIssue>,
    pub key_count: usize,
    pub parsed_ok: bool,
}

pub fn inspect_file(path: &Path) -> Result<ConfigInspectionResult> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    match ext.as_str() {
        "yaml" | "yml" => yaml_inspector::inspect(path),
        "json" => json_inspector::inspect(path),
        "toml" => toml_inspector::inspect(path),
        _ => anyhow::bail!("Unsupported config format: {}", ext),
    }
}
