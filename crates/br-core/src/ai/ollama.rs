use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{AiAnalyzer, prompts::{build_analysis_prompt, IncidentContext}};
use crate::models::report::{DiagnosticReport, FixSuggestion, ConfigConflict, CodeSnippet};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: &'static str,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaAnalyzer {
    host: String,
    model: String,
    client: Client,
}

impl OllamaAnalyzer {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            model: model.into(),
            client: Client::new(),
        }
    }

    pub fn default_local() -> Self {
        Self::new("http://localhost:11434", "llama3.2")
    }
}

#[async_trait]
impl AiAnalyzer for OllamaAnalyzer {
    async fn analyze(&self, ctx: &IncidentContext<'_>) -> Result<DiagnosticReport> {
        let prompt = build_analysis_prompt(ctx);

        let body = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            format: "json",
        };

        let url = format!("{}/api/generate", self.host);
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Ollama request failed")?;

        let ollama_resp: OllamaResponse = response.json().await.context("Failed to parse Ollama response")?;

        let raw: serde_json::Value = serde_json::from_str(&ollama_resp.response)
            .context("Failed to parse JSON from Ollama response")?;

        let mut report = DiagnosticReport::new(&ctx.incident.id, "ollama", &self.model);
        report.summary = raw["summary"].as_str().unwrap_or("").to_string();
        report.root_cause = raw["root_cause"].as_str().unwrap_or("").to_string();
        report.contributing_factors = raw["contributing_factors"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        report.confidence = raw["confidence"].as_f64().unwrap_or(0.5) as f32;

        Ok(report)
    }

    fn provider_name(&self) -> &str { "ollama" }

    async fn is_available(&self) -> bool {
        let url = format!("{}/api/tags", self.host);
        self.client.get(&url).send().await.map(|r| r.status().is_success()).unwrap_or(false)
    }
}
