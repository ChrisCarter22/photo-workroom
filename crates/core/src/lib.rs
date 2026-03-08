use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    pub request_id: String,
    pub active_workspace: String,
}

impl HealthCheckRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.request_id.trim().is_empty() {
            return Err("request_id must not be empty");
        }

        if self.active_workspace.trim().is_empty() {
            return Err("active_workspace must not be empty");
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubsystemSnapshot {
    pub name: String,
    pub phase: String,
    pub summary: String,
}

impl SubsystemSnapshot {
    pub fn new(name: &str, phase: &str, summary: &str) -> Self {
        Self {
            name: name.to_string(),
            phase: phase.to_string(),
            summary: summary.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QueueSnapshot {
    pub name: String,
    pub priority: String,
    pub summary: String,
}

impl QueueSnapshot {
    pub fn new(name: &str, priority: &str, summary: &str) -> Self {
        Self {
            name: name.to_string(),
            priority: priority.to_string(),
            summary: summary.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub request_id: String,
    pub app_version: String,
    pub runtime: String,
    pub active_workspace: String,
    pub healthy: bool,
    pub message: String,
    pub subsystems: Vec<SubsystemSnapshot>,
    pub queues: Vec<QueueSnapshot>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_health_check_requests() {
        let request = HealthCheckRequest {
            request_id: "health-1".to_string(),
            active_workspace: "Untitled".to_string(),
        };

        assert_eq!(request.validate(), Ok(()));
    }

    #[test]
    fn rejects_empty_health_check_requests() {
        let request = HealthCheckRequest {
            request_id: String::new(),
            active_workspace: String::new(),
        };

        assert_eq!(request.validate(), Err("request_id must not be empty"));
    }

    #[test]
    fn creates_snapshots_with_owned_values() {
        let subsystem = SubsystemSnapshot::new("db", "Phase 3", "SQLite boundary scaffolded.");
        let queue = QueueSnapshot::new("preview", "high", "Visible previews first.");

        assert_eq!(subsystem.name, "db");
        assert_eq!(queue.priority, "high");
    }
}
