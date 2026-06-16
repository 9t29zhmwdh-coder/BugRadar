use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{AiAnalyzer, prompts::{build_analysis_prompt, IncidentContext}};
use crate::models::report::{DiagnosticReport, FixSuggestion, ConfigConflict, CodeSnippet};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-haiku-4-5-20251001";

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct ApiRequest {
    model: &'static str,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize)]
struct RawReport {
    summary: Option<String>,
    root_cause: Option<String>,
    contributing_factors: Option<Vec<String>>,
    fix_suggestions: Option<Vec<serde_json::Value>>,
    config_conflicts: Option<Vec<serde_json::Value>>,
    confidence: Option<f64>,
}

pub struct ClaudeAnalyzer {
    api_key: String,
    client: Client,
}

impl ClaudeAnalyzer {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn parse_fix_suggestions(raw: &[serde_json::Value]) -> Vec<FixSuggestion> {
        raw.iter().map(|v| FixSuggestion {
            priority: v["priority"].as_u64().unwrap_or(5) as u8,
            title: v["title"].as_str().unwrap_or("Fix").to_string(),
            description: v["description"].as_str().unwrap_or("").to_string(),
            command: v["command"].as_str().map(|s| s.to_string()),
            code_snippet: v.get("code_snippet").map(|cs| CodeSnippet {
                language: cs["language"].as_str().unwrap_or("text").to_string(),
                filename: cs["filename"].as_str().map(|s| s.to_string()),
                content: cs["content"].as_str().unwrap_or("").to_string(),
                diff: cs["diff"].as_str().map(|s| s.to_string()),
            }),
        }).collect()
    }

    fn parse_config_conflicts(raw: &[serde_json::Value]) -> Vec<ConfigConflict> {
        raw.iter().map(|v| ConfigConflict {
            file_path: v["file_path"].as_str().unwrap_or("").to_string(),
            key: v["key"].as_str().unwrap_or("").to_string(),
            current_value: v["current_value"].as_str().unwrap_or("").to_string(),
            suggested_value: v["suggested_value"].as_str().unwrap_or("").to_string(),
            reason: v["reason"].as_str().unwrap_or("").to_string(),
        }).collect()
    }
}

#[async_trait]
impl AiAnalyzer for ClaudeAnalyzer {
    async fn analyze(&self, ctx: &IncidentContext<'_>) -> Result<DiagnosticReport> {
        let prompt = build_analysis_prompt(ctx);

        let body = ApiRequest {
            model: MODEL,
            max_tokens: 2048,
            messages: vec![Message { role: "user", content: prompt }],
        };

        let response = self.client
            .post(API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Claude API request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Claude API error {}: {}", status, &body[..body.len().min(200)]);
        }

        let api_resp: ApiResponse = response.json().await.context("Failed to parse Claude response")?;
        let total_tokens = api_resp.usage.map(|u| u.input_tokens + u.output_tokens);

        let text = api_resp.content.into_iter()
            .find(|b| b.kind == "text")
            .and_then(|b| b.text)
            .unwrap_or_default();

        let raw: RawReport = serde_json::from_str(&text)
            .context("Failed to parse JSON from Claude response")?;

        let mut report = DiagnosticReport::new(&ctx.incident.id, "claude", MODEL);
        report.summary = raw.summary.unwrap_or_default();
        report.root_cause = raw.root_cause.unwrap_or_default();
        report.contributing_factors = raw.contributing_factors.unwrap_or_default();
        report.fix_suggestions = raw.fix_suggestions.as_deref()
            .map(Self::parse_fix_suggestions)
            .unwrap_or_default();
        report.config_conflicts = raw.config_conflicts.as_deref()
            .map(Self::parse_config_conflicts)
            .unwrap_or_default();
        report.confidence = raw.confidence.unwrap_or(0.7) as f32;
        report.tokens_used = total_tokens;

        Ok(report)
    }

    fn provider_name(&self) -> &str { "claude" }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }
}
