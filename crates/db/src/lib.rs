use photo_workroom_core::SubsystemSnapshot;

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "db",
        "Phase 3",
        "SQLite schema, migrations, and WAL tuning are scaffolded as a dedicated crate boundary.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_database_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "db");
        assert!(snapshot.summary.contains("SQLite"));
    }
}
