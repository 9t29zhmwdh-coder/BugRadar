pub mod claude;
pub mod ollama;
pub mod prompts;

use anyhow::Result;
use async_trait::async_trait;

use crate::models::report::DiagnosticReport;
use prompts::IncidentContext;

#[async_trait]
pub trait AiAnalyzer: Send + Sync {
    async fn analyze(&self, ctx: &IncidentContext<'_>) -> Result<DiagnosticReport>;
    fn provider_name(&self) -> &str;
    async fn is_available(&self) -> bool;
}
