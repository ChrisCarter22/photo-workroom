use photo_workroom_core::{QueueSnapshot, SubsystemSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    High,
    Medium,
    Low,
}

impl TaskPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelperQueueType {
    Preview,
    Metadata,
    Rename,
    Ingest,
    Delivery,
}

impl HelperQueueType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Metadata => "metadata",
            Self::Rename => "rename",
            Self::Ingest => "ingest",
            Self::Delivery => "delivery",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueDefinition {
    pub queue_type: HelperQueueType,
    pub priority: TaskPriority,
    pub summary: &'static str,
}

pub const COORDINATOR_RESPONSIBILITIES: &[&str] = &[
    "Maintain dedicated helper queues for ingest, preview, metadata, rename, and delivery.",
    "Prioritize visible preview and browsing work over bulk background helpers.",
    "Support cancellation, retry, pause, and resume transitions safely.",
    "Publish progress and failure state into the Tasks surface and task-history model.",
    "Isolate helper failures so one queue cannot stall unrelated work.",
];

pub const DEFAULT_QUEUE_DEFINITIONS: [QueueDefinition; 5] = [
    QueueDefinition {
        queue_type: HelperQueueType::Preview,
        priority: TaskPriority::High,
        summary: "Visible previews and thumbnails stay interactive.",
    },
    QueueDefinition {
        queue_type: HelperQueueType::Metadata,
        priority: TaskPriority::High,
        summary: "Active metadata edits should not lag behind browsing.",
    },
    QueueDefinition {
        queue_type: HelperQueueType::Rename,
        priority: TaskPriority::Medium,
        summary: "Rename batches remain responsive without starving preview.",
    },
    QueueDefinition {
        queue_type: HelperQueueType::Ingest,
        priority: TaskPriority::Medium,
        summary: "Transfer and extraction stay bounded behind interactive work.",
    },
    QueueDefinition {
        queue_type: HelperQueueType::Delivery,
        priority: TaskPriority::Low,
        summary: "Delivery helpers yield to ingest, metadata, and visible preview.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Running,
    Paused,
    Succeeded,
    Failed,
    Cancelled,
}

impl TaskState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

pub fn can_transition_task_state(from: TaskState, to: TaskState) -> bool {
    matches!(
        (from, to),
        (TaskState::Queued, TaskState::Running)
            | (TaskState::Queued, TaskState::Cancelled)
            | (TaskState::Running, TaskState::Paused)
            | (TaskState::Running, TaskState::Succeeded)
            | (TaskState::Running, TaskState::Failed)
            | (TaskState::Running, TaskState::Cancelled)
            | (TaskState::Paused, TaskState::Running)
            | (TaskState::Paused, TaskState::Cancelled)
            | (TaskState::Failed, TaskState::Queued)
            | (TaskState::Cancelled, TaskState::Queued)
    )
}

pub fn can_retry_task(state: TaskState) -> bool {
    matches!(state, TaskState::Failed | TaskState::Cancelled)
}

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "task_runtime",
        "Phase 3",
        "Coordinator responsibilities, queue priorities, and cancellation/retry state rules are defined for helper runtime scaffolding.",
    )
}

pub fn bootstrap_queues() -> Vec<QueueSnapshot> {
    DEFAULT_QUEUE_DEFINITIONS
        .iter()
        .map(|definition| {
            QueueSnapshot::new(
                definition.queue_type.as_str(),
                definition.priority.as_str(),
                definition.summary,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_all_helper_queue_types_with_priority_order() {
        let queues = bootstrap_queues();

        assert_eq!(queues.len(), 5);
        assert_eq!(queues[0].name, "preview");
        assert_eq!(queues[0].priority, "high");
        assert_eq!(queues[1].name, "metadata");
        assert_eq!(queues[2].name, "rename");
        assert_eq!(queues[3].name, "ingest");
        assert_eq!(queues[4].name, "delivery");
        assert_eq!(queues[4].priority, "low");
    }

    #[test]
    fn defines_coordinator_responsibilities_for_publication() {
        assert_eq!(COORDINATOR_RESPONSIBILITIES.len(), 5);
        assert!(COORDINATOR_RESPONSIBILITIES
            .iter()
            .any(|responsibility| responsibility.contains("Prioritize visible preview")));
    }

    #[test]
    fn models_cancellation_and_retry_task_state_transitions() {
        assert!(can_transition_task_state(
            TaskState::Queued,
            TaskState::Running
        ));
        assert!(can_transition_task_state(
            TaskState::Running,
            TaskState::Cancelled
        ));
        assert!(can_transition_task_state(
            TaskState::Paused,
            TaskState::Running
        ));
        assert!(can_transition_task_state(
            TaskState::Failed,
            TaskState::Queued
        ));

        assert!(!can_transition_task_state(
            TaskState::Succeeded,
            TaskState::Running
        ));
        assert!(!can_transition_task_state(
            TaskState::Queued,
            TaskState::Paused
        ));

        assert!(can_retry_task(TaskState::Failed));
        assert!(can_retry_task(TaskState::Cancelled));
        assert!(!can_retry_task(TaskState::Succeeded));
    }
}
