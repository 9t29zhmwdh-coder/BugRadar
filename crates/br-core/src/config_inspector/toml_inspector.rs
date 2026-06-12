use std::path::Path;
use anyhow::Result;

use super::{ConfigInspectionResult, ConfigIssue};

pub fn inspect(path: &Path) -> Result<ConfigInspectionResult> {
    let content = std::fs::read_to_string(path)?;
    let mut issues = Vec::new();

    match toml::from_str::<toml::Value>(&content) {
        Ok(value) => {
            let key_count = count_keys(&value);
            check_issues(&value, &mut issues, "");
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "toml".to_string(),
                issues,
                key_count,
                parsed_ok: true,
            })
        }
        Err(e) => {
            issues.push(ConfigIssue {
                severity: "error".to_string(),
                key: "<root>".to_string(),
                message: format!("TOML parse error: {}", e),
                suggestion: Some("Check TOML syntax".to_string()),
            });
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "toml".to_string(),
                issues,
                key_count: 0,
                parsed_ok: false,
            })
        }
    }
}

fn count_keys(v: &toml::Value) -> usize {
    match v {
        toml::Value::Table(t) => t.len() + t.values().map(count_keys).sum::<usize>(),
        toml::Value::Array(a) => a.iter().map(count_keys).sum(),
        _ => 0,
    }
}

fn check_issues(v: &toml::Value, issues: &mut Vec<ConfigIssue>, prefix: &str) {
    if let toml::Value::Table(map) = v {
        for (key, val) in map {
            let full_key = if prefix.is_empty() { key.clone() } else { format!("{}.{}", prefix, key) };
            if matches!(key.as_str(), "password" | "secret" | "token") {
                if val.as_str().map(|s| s.is_empty()).unwrap_or(false) {
                    issues.push(ConfigIssue {
                        severity: "warning".to_string(),
                        key: full_key.clone(),
                        message: format!("Sensitive key '{}' is empty", full_key),
                        suggestion: None,
                    });
                }
            }
            check_issues(val, issues, &full_key);
        }
    }
}
