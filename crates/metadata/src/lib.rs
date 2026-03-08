use photo_workroom_core::SubsystemSnapshot;

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "metadata",
        "Phase 9",
        "IPTC-aware normalization, precedence, and write-back flow are reserved behind the metadata crate.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_metadata_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "metadata");
        assert!(snapshot.summary.contains("IPTC"));
    }
}
