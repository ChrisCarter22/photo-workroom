# ROADMAP.md

This document gives the high-level release roadmap. Detailed execution tracking lives in `PLANNING.md`.

Current repository status on March 8, 2026:

- documentation groundwork exists
- implementation has not started
- roadmap items below are forward-looking, not completed product milestones

## Status legend

- `[x]` completed
- `[-]` in progress
- `[ ]` not started

## Product benchmark

The near-term benchmark is standard Photo Mechanic parity for the core desktop workflow.

Included in that benchmark:

- ingest, including ingest from selection and watched or live ingest
- multi-source ingest, slideshow, and live slideshow review workflows
- tabbed contact sheets with task visibility and saved folder context
- fast culling with tags, ratings, and color classes
- preview, comparison, and crop workflows
- IPTC-style single and batch metadata workflows plus reverse geocoding and compatibility controls
- variables, code replacements, and keyword tooling
- Save As, watermarking, delivery, video clip handling, and external-editor handoff

Explicitly deferred from the baseline:

- Photo Mechanic Plus-style catalog parity until a stable first-party design exists
- this is a deliberate scope decision for the open source project, not a statement that the commercial Plus edition is unavailable

## Near-term roadmap

- `[-]` Foundation and repository baseline
  - `[x]` core documentation bundle exists
  - `[x]` planning, architecture, and process docs were expanded
  - `[ ]` monorepo scaffold exists
  - `[ ]` workspace manifests exist
  - `[ ]` CI exists
- `[ ]` Desktop shell baseline
  - `[ ]` Tauri app scaffold exists
  - `[ ]` React renderer exists
  - `[ ]` typed IPC smoke path exists
- `[ ]` Local catalog foundation
  - `[ ]` SQLite schema exists
  - `[ ]` migrations exist
  - `[ ]` scan pipeline exists

## Product milestones

- `[ ]` M3: Ingest, browse, preview, and selection MVP
- `[ ]` M4: Metadata, rename, search, delivery, and export MVP
- `[ ]` M5: Performance and reliability hardening
- `[ ]` M6: CI maturity, cross-platform validation, and beta
- `[ ]` M7: Stable release readiness
- `[ ]` M8: Post-release maintenance cadence

## Release lanes

### v0.x

Focus:

- scaffold the application
- build the local catalog
- prove core import and browsing workflows

### v1.0 beta

Focus:

- complete ingest, culling, metadata, search, rename, delivery, and output workflows
- validate real-world libraries and contributor workflows

### v1.0 stable

Focus:

- clear release-blocking debt
- complete cross-platform packaging
- finalize release engineering and onboarding

### Post-1.0

Focus:

- maintain quality
- pay down debt
- expand capabilities based on validated user needs
