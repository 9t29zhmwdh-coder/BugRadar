use std::path::Path;
use anyhow::Result;

use super::{ConfigInspectionResult, ConfigIssue};

pub fn inspect(path: &Path) -> Result<ConfigInspectionResult> {
    let content = std::fs::read_to_string(path)?;
    let mut issues = Vec::new();

    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(value) => {
            let key_count = count_keys(&value);
            check_issues(&value, &mut issues, "");
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "json".to_string(),
                issues,
                key_count,
                parsed_ok: true,
            })
        }
        Err(e) => {
            issues.push(ConfigIssue {
                severity: "error".to_string(),
                key: "<root>".to_string(),
                message: format!("JSON parse error: {}", e),
                suggestion: Some("Check for missing commas, brackets, or invalid JSON syntax".to_string()),
            });
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "json".to_string(),
                issues,
                key_count: 0,
                parsed_ok: false,
            })
        }
    }
}

fn count_keys(v: &serde_json::Value) -> usize {
    match v {
        serde_json::Value::Object(m) => m.len() + m.values().map(count_keys).sum::<usize>(),
        serde_json::Value::Array(a) => a.iter().map(count_keys).sum(),
        _ => 0,
    }
}

fn check_issues(v: &serde_json::Value, issues: &mut Vec<ConfigIssue>, prefix: &str) {
    if let serde_json::Value::Object(map) = v {
        for (key, val) in map {
            let full_key = if prefix.is_empty() { key.clone() } else { format!("{}.{}", prefix, key) };

            if matches!(key.as_str(), "password" | "secret" | "token" | "apiKey" | "api_key")
                && (val.as_str().map(|s| s.is_empty()).unwrap_or(false) || val.is_null())
            {
                issues.push(ConfigIssue {
                    severity: "warning".to_string(),
                    key: full_key.clone(),
                    message: format!("Sensitive key '{}' is empty", full_key),
                    suggestion: None,
                });
            }

            check_issues(val, issues, &full_key);
        }
    }
}
