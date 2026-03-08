use photo_workroom_core::SubsystemSnapshot;

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "image",
        "Phase 7",
        "Thumbnail, preview, and transform helpers are isolated behind an image crate boundary.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_image_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "image");
        assert!(snapshot.summary.contains("Thumbnail"));
    }
}
