use bollard::Docker;
use bollard::container::LogsOptions;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::models::log_entry::LogEntry;
use crate::plugin::LogParserPlugin;

pub struct DockerCollector {
    docker: Docker,
}

impl DockerCollector {
    pub fn new() -> anyhow::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    pub fn spawn_container_tail(
        &self,
        container_id: String,
        source_id: String,
        source_path: String,
        mut parser: Box<dyn LogParserPlugin>,
        tx: mpsc::Sender<LogEntry>,
    ) -> tokio::task::JoinHandle<()> {
        let docker = self.docker.clone();

        tokio::spawn(async move {
            info!("Starting Docker log tail for container {}", container_id);

            let options = LogsOptions::<String> {
                follow: true,
                stdout: true,
                stderr: true,
                tail: "100".to_string(),
                ..Default::default()
            };

            let mut stream = docker.logs(&container_id, Some(options));

            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(log_output) => {
                        let line = log_output.to_string();
                        let line = line.trim_end_matches(n);

                        if let Some(mut entry) = parser.push_line(line) {
                            entry.source_id = source_id.clone();
                            entry.source_path = source_path.clone();
                            let _ = tx.send(entry).await;
                        }
                    }
                    Err(e) => {
                        warn!("Docker log stream error for {}: {}", container_id, e);
                        break;
                    }
                }
            }

            if let Some(mut entry) = parser.flush() {
                entry.source_id = source_id.clone();
                let _ = tx.send(entry).await;
            }

            info!("Docker log tail ended for {}", container_id);
        })
    }

    pub async fn list_containers(&self) -> anyhow::Result<Vec<(String, String)>> {
        use bollard::container::ListContainersOptions;

        let options = ListContainersOptions::<String> {
            all: false,
            ..Default::default()
        };

        let containers = self.docker.list_containers(Some(options)).await?;
        let result = containers
            .into_iter()
            .filter_map(|c| {
                let id = c.id?;
                let name = c.names?.into_iter().next()?.trim_start_matches(/).to_string();
                Some((id, name))
            })
            .collect();

        Ok(result)
    }
}
