use photo_workroom_core::{QueueSnapshot, SubsystemSnapshot};

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "task_runtime",
        "Phase 3",
        "Separated queue definitions keep preview work ahead of slower background pipelines.",
    )
}

pub fn bootstrap_queues() -> Vec<QueueSnapshot> {
    vec![
        QueueSnapshot::new(
            "preview",
            "high",
            "Visible previews and thumbnails stay interactive.",
        ),
        QueueSnapshot::new(
            "ingest",
            "medium",
            "Transfer and metadata extraction remain bounded.",
        ),
        QueueSnapshot::new(
            "delivery",
            "low",
            "Export and delivery helpers yield to visible browsing.",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_priority_order_for_bootstrap_queues() {
        let queues = bootstrap_queues();

        assert_eq!(queues.len(), 3);
        assert_eq!(queues[0].name, "preview");
    }
}
