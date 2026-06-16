use std::path::Path;
use anyhow::Result;

use super::{ConfigInspectionResult, ConfigIssue};

pub fn inspect(path: &Path) -> Result<ConfigInspectionResult> {
    let content = std::fs::read_to_string(path)?;
    let mut issues = Vec::new();

    match serde_yaml::from_str::<serde_yaml::Value>(&content) {
        Ok(value) => {
            let key_count = count_keys(&value);
            check_common_issues(&value, &mut issues, "");
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "yaml".to_string(),
                issues,
                key_count,
                parsed_ok: true,
            })
        }
        Err(e) => {
            issues.push(ConfigIssue {
                severity: "error".to_string(),
                key: "<root>".to_string(),
                message: format!("YAML parse error: {}", e),
                suggestion: Some("Check for indentation issues or invalid YAML syntax".to_string()),
            });
            Ok(ConfigInspectionResult {
                file_path: path.display().to_string(),
                format: "yaml".to_string(),
                issues,
                key_count: 0,
                parsed_ok: false,
            })
        }
    }
}

fn count_keys(v: &serde_yaml::Value) -> usize {
    match v {
        serde_yaml::Value::Mapping(m) => {
            m.len() + m.values().map(count_keys).sum::<usize>()
        }
        serde_yaml::Value::Sequence(s) => s.iter().map(count_keys).sum(),
        _ => 0,
    }
}

fn check_common_issues(v: &serde_yaml::Value, issues: &mut Vec<ConfigIssue>, prefix: &str) {
    if let serde_yaml::Value::Mapping(map) = v {
        for (k, val) in map {
            let key_str = k.as_str().unwrap_or("<key>");
            let full_key = if prefix.is_empty() { key_str.to_string() } else { format!("{}.{}", prefix, key_str) };

            if matches!(key_str, "host" | "password" | "secret" | "token" | "key" | "url" | "database")
                && (val.is_null() || val.as_str().map(|s| s.is_empty()).unwrap_or(false))
            {
                issues.push(ConfigIssue {
                    severity: "warning".to_string(),
                    key: full_key.clone(),
                    message: format!("Key '{}' appears to be empty or null", full_key),
                    suggestion: Some("Ensure this value is set correctly".to_string()),
                });
            }

            if key_str == "port" {
                if let Some(port) = val.as_u64() {
                    if port < 1024 {
                        issues.push(ConfigIssue {
                            severity: "info".to_string(),
                            key: full_key.clone(),
                            message: format!("Port {} is a privileged port (< 1024)", port),
                            suggestion: Some("May require root/admin permissions".to_string()),
                        });
                    }
                }
            }

            check_common_issues(val, issues, &full_key);
        }
    }
}
