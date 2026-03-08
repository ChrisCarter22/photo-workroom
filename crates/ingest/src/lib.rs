use photo_workroom_core::SubsystemSnapshot;

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "ingest",
        "Phase 5",
        "Folder, card, and watched-source ingest planning now has a dedicated Rust crate entrypoint.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_ingest_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "ingest");
        assert!(snapshot.summary.contains("ingest"));
    }
}
