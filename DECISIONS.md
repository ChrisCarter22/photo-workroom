# DECISIONS.md

This file records ADR-style architectural and process decisions for Photo Workroom.

Current repository status on March 9, 2026:

- the entries below are accepted planning baselines
- when implementation work proves a different direction is necessary, add a new ADR instead of silently rewriting history

## ADR template

Each new entry should include:

- ID
- date
- status
- context
- decision
- consequences

## ADR-001: Desktop runtime baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the product needs a desktop shell with a clear security boundary and good local-system integration

Decision:

- use Tauri v2 as the default desktop runtime baseline

Consequences:

- privileged logic lives in Rust
- UI runs in the platform WebView
- packaging and updater work should follow Tauri conventions unless superseded

## ADR-002: Core implementation language

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- ingest, metadata, preview, and DB workflows are performance-sensitive and correctness-critical

Decision:

- use Rust for performance-sensitive and correctness-critical core logic

Consequences:

- backend modules should be implemented as reusable Rust crates where practical
- FFI-heavy functionality must stay behind narrow interfaces

## ADR-003: Frontend baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the project needs a productive UI stack with strong typing and broad contributor familiarity

Decision:

- use React, TypeScript, and Vite for the renderer baseline

Consequences:

- frontend code should stay thin and workflow-oriented
- business logic should not drift into UI-only helpers

## ADR-004: Local database baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the app needs a local-first searchable catalog with strong transactional guarantees

Decision:

- use SQLite as the local metadata and audit store

Consequences:

- schema changes require migrations
- search should build on SQLite indexing and FTS features

## ADR-005: Metadata strategy baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the product must read and write EXIF, IPTC, and XMP with sidecar awareness

Decision:

- adopt an Exiv2-based strategy as the primary metadata path, with controlled fallback only when justified by format gaps

Consequences:

- metadata behavior must be tested with representative fixtures
- any fallback tooling must be documented and isolated

## ADR-006: Local-first and no-telemetry baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- user trust depends on predictable offline behavior and privacy-preserving defaults

Decision:

- design the application to work locally without telemetry or cloud dependence by default

Consequences:

- security, CI, and product documentation must preserve that expectation
- any future networked feature would require a new ADR and explicit user-facing documentation

## ADR-007: Reference product parity target

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the product needs a concrete workflow benchmark so planning does not drift into a vague "photo app" shape

Decision:

- use standard Photo Mechanic feature parity as the benchmark for the core desktop workflow
- explicitly defer Photo Mechanic Plus-style catalog parity until the project has a stable first-party design for it

Consequences:

- as of March 8, 2026, Camera Bits still documents and sells both standard Photo Mechanic and Photo Mechanic Plus, so this deferral is a project scope choice rather than a product-state assumption
- planning must account for ingest, contact sheets, culling, preview, slideshow, metadata, variables, code replacements, keyword tools, delivery outputs, GPS utilities, compatibility controls, and shortcut customization
- catalog or cross-drive organizer features are not considered baseline MVP requirements today

## ADR-008: JavaScript package-manager baseline

Date:

- 2026-03-08

Status:

- Accepted for planning

Context:

- the project needs a single JS and TS package-manager baseline
- the preferred workflow is project-local dependency installation with no unnecessary tooling overhead
- dependency growth should stay narrow and explicit

Decision:

- use `npm` as the JS and TS package-manager baseline
- use project-local installs and npm workspaces when the monorepo exists
- keep `package-lock.json` authoritative and add dependencies one at a time

Consequences:

- source-of-truth docs, verification commands, and CI examples should use `npm` and not `pnpm`
- dependency policy should prefer `npm ci` for clean installs and `npm install ... --save-exact` for justified additions
- historical research notes that mention `pnpm` should be treated as superseded by the current planning baseline

## ADR-009: SQLite access layer and migration baseline

Date:

- 2026-03-09

Status:

- Accepted and implemented

Context:

- Phase 3 requires a concrete local DB foundation with deterministic startup behavior across Linux, macOS, and Windows
- the project needed one primary SQLite layer and a migration workflow that avoids hidden runtime drift

Decision:

- use `rusqlite` as the single SQLite access layer baseline for the current sync-first architecture
- store incremental SQL migrations in `crates/db/migrations/` and apply them in order using `PRAGMA user_version`
- initialize the catalog DB on desktop startup from the Tauri app-data directory with a stable path strategy:
  - `<app_data_dir>/catalog/photo-workroom.sqlite3`
- set SQLite defaults during initialization:
  - `PRAGMA foreign_keys = ON`
  - `PRAGMA journal_mode = WAL`
  - `PRAGMA synchronous = NORMAL`

Consequences:

- `crates/db` now owns migration and repository bootstrap logic instead of scattering DB setup in app code
- schema changes must ship as a new numbered SQL migration
- migration tests must cover fresh bootstrap and upgrade-from-older-schema behavior
- if async DB access becomes necessary later, a new ADR is required before introducing another access layer
