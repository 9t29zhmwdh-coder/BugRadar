use bollard::Docker;
use bollard::query_parameters::ListContainersOptions;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContainerState {
    Running,
    Exited,
    Restarting,
    Paused,
    Dead,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStatus {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub restart_count: u32,
    pub is_crash_looping: bool,
}

pub struct DockerMonitor {
    docker: Docker,
}

impl DockerMonitor {
    pub fn new() -> Result<Self> {
        Ok(Self { docker: Docker::connect_with_local_defaults()? })
    }

    pub async fn get_container_statuses(&self) -> Result<Vec<ContainerStatus>> {
        let options = ListContainersOptions { all: true, ..Default::default() };
        let containers = self.docker.list_containers(Some(options)).await?;

        let statuses = containers.into_iter().filter_map(|c| {
            let id = c.id.clone().unwrap_or_default();
            let name = c.names?.into_iter().next()?.trim_start_matches('/').to_string();
            let image = c.image.clone().unwrap_or_default();
            // bollard 0.21 liefert `state` als typisiertes Enum statt als String.
            // Dessen Display gibt genau die Kleinbuchstaben-Bezeichnungen aus,
            // die vorher direkt von Docker kamen ("running", "exited", ...),
            // die Zuordnung darunter bleibt damit unveraendert.
            let state_str = c
                .state
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string())
                .to_lowercase();
            let status = c.status.unwrap_or_default();

            let state = match state_str.as_str() {
                "running" => ContainerState::Running,
                "exited" => ContainerState::Exited,
                "restarting" => ContainerState::Restarting,
                "paused" => ContainerState::Paused,
                "dead" => ContainerState::Dead,
                other => ContainerState::Unknown(other.to_string()),
            };

            // Detect crash loop from status string: "Restarting (1) 2 seconds ago"
            let restart_count = Self::extract_restart_count(&status);
            let is_crash_looping = state == ContainerState::Restarting || restart_count >= 3;

            Some(ContainerStatus { id, name, image, state, status, restart_count, is_crash_looping })
        }).collect();

        Ok(statuses)
    }

    fn extract_restart_count(status: &str) -> u32 {
        // "Restarting (5) 3 seconds ago" → 5
        let re = regex::Regex::new(r"\((\d+)\)").unwrap();
        re.captures(status)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0)
    }
}
