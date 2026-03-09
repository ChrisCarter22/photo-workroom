# TECH_STACK.md

This document defines the preferred technology baseline for Photo Workroom.

Current repository status on March 9, 2026:

- the workspace is now scaffolded with npm workspaces, a Rust workspace, and a Tauri v2 plus React plus TypeScript plus Vite shell app
- exact installed JS versions are pinned in `package-lock.json` and the current Rust crate graph is pinned in `Cargo.lock`
- a Phase 3 baseline now exists in `crates/db` using `rusqlite` plus SQL-file migrations with `PRAGMA user_version`
- Playwright and metadata-library integration are still planned beyond the bootstrap shell
- any deviation from these defaults must be recorded in `DECISIONS.md`

## Decision summary

Recommended default stack:

- desktop runtime: Tauri v2
- core language: Rust 2021+
- UI: React + TypeScript + Vite
- local DB and index: SQLite
- metadata interop: Exiv2-based strategy, with ExifTool fallback only for documented gaps
- RAW access: LibRaw strategy where feasible
- JS and TS package manager: npm only
- frontend unit tests: Vitest
- end-to-end tests: Playwright
- Rust tests: `cargo test`, optionally `cargo nextest run`
- logging: Rust `tracing`

## Core technologies

### Tauri v2

Role:

- desktop shell
- native windowing
- packaging
- typed IPC boundary between renderer and privileged backend

Chosen because:

- smaller distribution footprint than Electron
- better default resource profile
- clear security boundary between UI and system access
- strong fit for local-first desktop workflows

### Rust

Role:

- filesystem integration
- metadata reads and writes
- preview generation
- database access
- performance-sensitive pipelines

Chosen because:

- high performance
- strong type system and memory safety
- excellent concurrency primitives
- good fit for a local-first desktop backend

### React + TypeScript + Vite

Role:

- application UI
- workflow orchestration
- typed interaction with backend commands

Chosen because:

- broad contributor familiarity
- mature ecosystem
- strong TypeScript support
- easy for coding agents and human contributors to extend safely

### SQLite

Role:

- local metadata index
- search and filter store
- ingest and audit history

Chosen because:

- embedded and local-first by design
- strong transactional guarantees
- built-in full-text search support
- no separate service dependency

### Metadata and image libraries

Preferred defaults:

- metadata: Exiv2 through a Rust-compatible wrapper or carefully controlled interop layer
- RAW decode: LibRaw
- image transforms: Rust `image` crate for baseline operations, with `libvips` considered only if benchmarks justify it
- video frame extraction: FFmpeg sidecar only when necessary and clearly documented

## Major choice comparisons

### Tauri vs Electron vs pure native

Recommended: Tauri v2

Why:

- uses the platform WebView instead of bundling Chromium
- gives the project native packaging and secure IPC without the cost of Electron
- keeps privileged logic in Rust

Rejected defaults:

- Electron: larger bundle and higher baseline memory cost
- pure native UI: higher implementation cost and slower iteration for the current team profile

### React vs alternatives

Recommended: React + TypeScript

Why:

- large ecosystem and high contributor familiarity
- strong testing and tooling story
- easier to staff and automate safely than niche frontend stacks

Rejected defaults:

- Vue: viable, but not selected as the baseline
- Svelte or Solid: attractive performance profile, but smaller contributor and tooling base for this project

### `sqlx` vs `rusqlite`

Current implemented baseline:

- `rusqlite` is the active DB layer for the current sync-first Phase 3 foundation
- migrations are SQL files applied in order via `PRAGMA user_version`
- decision recorded in `DECISIONS.md` ADR-009

Future decision rule:

- introduce `sqlx` only if async DB orchestration becomes necessary and the change is justified in a new ADR
- do not run both layers in parallel without a clearly documented responsibility split

## UI stack

Frontend defaults:

- bundler: Vite
- language: TypeScript with strict mode enabled
- routing: React Router only if multiple major screens require explicit route state
- styling: CSS Modules, scoped CSS, or another lightweight system already approved by the team
- state management: React state and context first, external state libraries only when complexity proves the need

Constraints:

- avoid heavy UI frameworks that fight the desired desktop-first interaction model
- keep keyboard accessibility and large-library responsiveness central to component decisions
- preserve a thin renderer and keep business logic in shared Rust or explicit domain layers

## Rust backend stack

Backend defaults:

- async runtime: Tokio if async orchestration becomes necessary
- DB layer: one of `sqlx` or `rusqlite`, not both by default
- logging: `tracing`
- task orchestration: explicit helper queues with bounded worker pools and cancellation support
- benchmarking: Criterion or equivalent when performance work starts
- error handling: explicit domain errors with typed conversion at IPC boundaries

Constraints:

- prefer crates with active maintenance and permissive licenses
- isolate FFI-heavy components behind narrow internal APIs
- do not let Tauri command handlers become the home for business logic

## Testing, linting, and CI

Preferred tooling:

- JS and TS lint: ESLint, optionally combined with Prettier or Biome if adopted explicitly
- Type checking: `tsc`
- frontend tests: Vitest
- end-to-end tests: Playwright
- Rust formatting: `cargo fmt`
- Rust linting: `cargo clippy`
- Rust testing: `cargo test`
- CI system: GitHub Actions

Required CI matrix once scaffolding exists:

- Ubuntu latest
- macOS latest
- Windows latest

Bootstrap note:

- `npm run test:e2e` currently runs a renderer-shell smoke suite via Vitest so the command exists and verifies real behavior before the Playwright harness lands

## Logging and telemetry

Logging policy:

- write structured logs locally
- keep logs useful for debugging ingest, metadata, and export failures
- redact or minimize sensitive data when possible

Telemetry policy:

- no telemetry by default
- no background analytics without explicit product and privacy review
- any future analytics must be opt-in and documented in `SECURITY.md`

## Plugin policy

Allowed direction:

- signed native sidecars or other tightly controlled extension mechanisms

Not allowed by default:

- arbitrary JavaScript plugins from remote sources
- runtime `eval`
- unrestricted third-party code execution in the renderer

## Performance stack decisions

Baseline decisions:

- SQLite in WAL mode during normal runtime
- bounded worker pools for ingest and preview tasks
- on-disk preview cache instead of DB blobs by default
- benchmark-driven adoption of heavier native libraries such as `libvips`

## Package manager rules

- JavaScript and TypeScript dependencies: `npm` only
- prefer project-local installs and workspace-local dependencies
- no `pnpm`
- no Yarn
- use npm workspaces when the monorepo exists
- keep `package-lock.json` authoritative
- add dependencies one at a time with written rationale

## Approved dependency table

| Dependency | Purpose | Approved because | Scope allowed | Alternatives rejected |
|---|---|---|---|---|
| Tauri | Desktop runtime and IPC shell | Lightweight desktop shell with strong Rust integration | App shell and packaging | Electron, which has a heavier runtime footprint |
| React | UI library | Mature ecosystem and contributor familiarity | Renderer components and views | Vue or Svelte as default baseline |
| TypeScript | Frontend language | Strong typing improves reliability | Entire frontend | Plain JavaScript |
| SQLite | Local database | Embedded, transactional, supports FTS | Full local index and history store | External DB servers or ad hoc flat files |
| LibRaw | RAW decode | Broad RAW format support | RAW preview and ingest pipeline | Older or less complete RAW decoders |
| Exiv2 strategy | Metadata I/O | Mature support for EXIF, IPTC, and XMP | Metadata read and write flows | Read-only EXIF libraries or shell-heavy defaults |
| Vitest | Frontend tests | Fast TS-native test execution | Unit and light integration tests | Heavier Jest baseline |
| Playwright | E2E tests | Good cross-platform automation story | Full workflow tests | Puppeteer-only or Cypress-only strategy |
| `tracing` | Rust logging | Structured logging and ecosystem maturity | Backend logs and audit helpers | Ad hoc logging |

## Change control

If a stack change becomes necessary:

- record the decision in `DECISIONS.md`
- update this file and `ARCHITECTURE.md`
- document migration, rollback, and verification impact in the relevant plan phase
