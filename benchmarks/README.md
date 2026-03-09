# benchmarks

This directory is reserved for repeatable benchmark harnesses and fixture manifests.

Current benchmark harnesses:

- `cargo test -p photo_workroom_ingest benchmark_large_folder_scan_behavior -- --ignored --nocapture`
  - creates a deterministic synthetic large-folder fixture
  - reports elapsed scan time and validates expected asset and sidecar counts

Latest local measurement on March 9, 2026:

- command: `cargo test -p photo_workroom_ingest benchmark_large_folder_scan_behavior -- --ignored --nocapture`
- output sample: `benchmark_large_folder_scan_behavior assets=3240 sidecars=720 elapsed_ms=58`
