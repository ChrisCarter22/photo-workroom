# PLANNING.md

This document is the execution tracker for Photo Workroom. It converts the research plan into a phase-based program with explicit completion markers.

Current repository status on March 9, 2026:

- the repository now contains a real workspace scaffold with `apps/`, `crates/`, `tests/`, `benchmarks/`, `scripts/`, `packages/`, and `.github/`
- the desktop shell baseline exists under `apps/main-app/` with typed Tauri health-check and separate-window folder-open flows
- a Phase 3 local DB foundation now exists in `crates/db` with `rusqlite`, SQL migrations, WAL defaults, and typed asset repository helpers
- a Phase 3 task-runtime baseline now exists in `crates/task_runtime` with explicit helper-queue priorities plus cancellation and retry state transitions
- a Phase 4 scan baseline now exists in `crates/ingest` for recursive folder scanning, media classification, RAW/JPEG pairing, sidecar linking, metadata-queue planning, and cancellation-aware progress publication
- a deterministic Tauri launch smoke command now exists at `npm run test:tauri-launch`
- cross-platform bootstrap validation automation now exists at `.github/workflows/bootstrap-validation.yml`, with a fully green Ubuntu, macOS, and Windows run recorded on March 8, 2026 (`run 22816838497`)
- bootstrap verification evidence is tracked in `tests/validation/BOOTSTRAP_VALIDATION.md`
- metadata reference fixtures have been promoted into `tests/fixtures/metadata/`
- the repository does contain `examples/XMP Side Car.XMP`, which should be treated as the first metadata sidecar reference input
- the repository also contains `examples/IPTC_Fields.XMP`, which should be treated as the broad XMP-sidecar reference for IPTC property coverage
- status markers below reflect that reality and should only be updated when work is present in the repository and verified

## Status legend

- `[x]` completed and verified in the repository
- `[-]` started or partially complete
- `[ ]` not started

## Update rules

When updating this file:

- do not mark a step complete unless the corresponding work exists in the repository
- do not mark a step complete if verification for that step is still failing
- prefer splitting vague work into checkable items rather than leaving ambiguous bullets
- if scope changes, update the phase notes and `DECISIONS.md`

Out-of-order execution note on March 8, 2026:

- a narrow subset of Phase 16 CI work was pulled forward to unblock unresolved Phase 1 and Phase 2 verification risk
- reason: Phase 1 cross-platform bootstrap validation and Phase 2 launch-smoke verification need repeatable matrix automation
- scope was kept to one bootstrap workflow and one launch-smoke helper without changing product architecture

## Product benchmark

The implementation target is standard Photo Mechanic-style workflow parity for the core desktop photographer workflow.

That means the roadmap must explicitly account for:

- ingest automation, including ingest from selection and live or watched ingest
- fast tabbed contact-sheet browsing with Navigator, Favorites, and Tasks concepts
- rapid culling using tags, star ratings, and named color classes
- fast preview, comparison, crop, and transform-preview tooling
- IPTC-style single and batch metadata entry
- variables, code replacements, and keyword vocabulary tooling
- Find, filter, sort, refresh, rescan, and manual arrangement workflows
- Save As, delivery outputs, upload, print, email, and watermarking workflows
- GPS and capture-time utilities
- deep keyboard and preference customization

Current exclusion:

- Photo Mechanic Plus-style cross-drive catalog parity is not part of the baseline target until the project has a stable first-party design for it
- as of March 8, 2026, official Camera Bits sources still document and sell Photo Mechanic Plus; the exclusion here is a deliberate scope choice for this project, not a claim that the commercial product no longer exists

## Current phase snapshot

| Phase | Status | Summary |
|---|---|---|
| Phase 0: Repository bootstrap | `[x]` | Workspace scaffold, manifests, fixture promotion, scripts, and verification entrypoints now exist and verify cleanly |
| Phase 1: Dev environment hardening | `[x]` | Toolchain docs, setup notes, templates, and bootstrap automation now verify cleanly on Ubuntu, macOS, and Windows |
| Phase 2: Shell app bootstrap | `[x]` | The shell app now supports typed IPC health-check, explicit separate-window behavior, and deterministic local launch-smoke verification |
| Phase 3: Local database and indexing foundation | `[x]` | SQLite access, migration bootstrap, startup initialization, typed asset CRUD, and task-runtime queue plus state baseline now exist and verify |
| Phase 4: Filesystem scanning foundation | `[-]` | Recursive scanning, classification, pairing, sidecar detection, and scan progress plus cancellation baseline now exist; DB persistence and benchmark coverage remain pending |
| Phase 5: Ingest MVP | `[ ]` | No ingest implementation exists yet |
| Phase 6: Fast browser and contact sheet MVP | `[ ]` | No UI implementation exists yet |
| Phase 7: Preview pipeline MVP | `[ ]` | No preview pipeline exists yet |
| Phase 8: Selection, rating, color, and tagging MVP | `[ ]` | No culling workflow exists yet |
| Phase 9: Metadata read and write MVP | `[ ]` | No metadata engine exists yet |
| Phase 10: Batch rename and template MVP | `[ ]` | No rename subsystem exists yet |
| Phase 11: Search, filter, and sort MVP | `[ ]` | No search implementation exists yet |
| Phase 12: Export, delivery, and external handoff MVP | `[ ]` | No output, delivery, or handoff implementation exists yet |
| Phase 13: Performance pass | `[ ]` | No benchmark harness exists yet |
| Phase 14: Reliability pass | `[ ]` | No edge-case verification exists yet |
| Phase 15: Documentation pass | `[-]` | Core docs exist and were expanded, but implementation-linked docs will need revision once code lands |
| Phase 16: CI hardening | `[-]` | A narrow bootstrap validation matrix workflow now exists; broader CI hardening and release automation remain pending |
| Phase 17: Cross-platform validation | `[ ]` | Bootstrap-level cross-platform validation runs exist; full phase scope remains pending |
| Phase 18: Beta readiness | `[ ]` | Product validation not started |
| Phase 19: Stable release readiness | `[ ]` | Release engineering not started |
| Phase 20: Post-release operations | `[ ]` | Post-release cadence not applicable yet |

## Milestone map

- M0: repository and tooling baseline
- M1: desktop shell boots and verifies cleanly
- M2: local index and filesystem scan foundation
- M3: ingest, browse, preview, and selection MVP
- M4: metadata, rename, search, delivery, and export MVP
- M5: performance and reliability hardening
- M6: CI, cross-platform, and beta readiness
- M7: stable release readiness
- M8: post-release operations

## Phase 0: Repository bootstrap `[x]`

Goal:

- create the repository skeleton and operating rules needed for implementation work

Primary deliverables:

- monorepo directory scaffold
- workspace manifests
- baseline contributor and agent guidance
- initial verification entrypoints

Current status notes:

- `[x]` root planning and architecture docs exist
- `[x]` support docs such as `REPO_STRUCTURE.md`, `ROADMAP.md`, `DEBT_REGISTER.md`, and `DECISIONS.md` exist
- `[x]` implementation directories now exist
- `[x]` workspace manifest files now exist

Checklist:

- `[x]` Documentation baseline
  - `[x]` `AGENTS.md` exists
  - `[x]` `ARCHITECTURE.md` exists
  - `[x]` `PLANNING.md` exists
  - `[x]` `TECH_STACK.md` exists
  - `[x]` `TESTING.md` exists
  - `[x]` `DEPENDENCY_POLICY.md` exists
  - `[x]` subsystem docs exist
  - `[x]` `IPTC_FIELD_MAP.md` exists
  - `[x]` root `README.md` quick start exists
- `[x]` Repository scaffold
  - `[x]` create `apps/`
  - `[x]` create `crates/`
  - `[x]` create `packages/`
  - `[x]` create `tests/`
  - `[x]` create `benchmarks/`
  - `[x]` create `scripts/`
  - `[x]` create `.github/`
- `[x]` Example and fixture management
  - `[x]` seed sidecar example exists at `examples/XMP Side Car.XMP`
  - `[x]` broad XMP field reference exists at `examples/IPTC_Fields.XMP`
  - `[x]` broad IPTC JPEG reference exists at `examples/IPTC-PhotometadataRef-Std2025.1.jpg`
  - `[x]` move stable metadata examples into `tests/fixtures/metadata/` once the test tree exists
  - `[ ]` add additional reference metadata examples as needed for parity-sensitive fields
- `[x]` Workspace configuration
  - `[x]` add root `package.json`
  - `[x]` add `package-lock.json`
  - `[x]` add root `Cargo.toml`
  - `[x]` add `rust-toolchain.toml`
  - `[x]` add root `tsconfig.base.json`
  - `[x]` add root lint and formatting config
- `[x]` Policy enforcement
  - `[x]` wire repository scripts for lint, typecheck, test, and build
  - `[x]` document supported toolchain versions
  - `[x]` document branch and PR expectations

Verification for phase completion:

- `npm ci`
- `npm run lint`
- `npm run typecheck`
- `npm run test`
- `npm run build`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo build --workspace`

Exit criteria:

- the monorepo scaffold exists
- contributors can install the workspace cleanly
- baseline scripts run successfully

## Phase 1: Dev environment hardening `[x]`

Goal:

- make local setup deterministic across supported operating systems

Checklist:

- `[x]` Toolchain documentation
  - `[x]` document supported Node version
  - `[x]` document supported npm version
  - `[x]` document supported Rust toolchain version
  - `[x]` document OS-specific system packages for Tauri
  - `[x]` document WebView requirements by platform
- `[x]` Contributor ergonomics
  - `[x]` add setup instructions for Linux
  - `[x]` add setup instructions for macOS
  - `[x]` add setup instructions for Windows
  - `[x]` add editor recommendations and extensions
  - `[x]` add troubleshooting notes for common setup failures
- `[x]` Local validation
  - `[x]` verify `npm ci` installs on all supported platforms
  - `[x]` verify Rust format and lint flows
  - `[x]` verify frontend lint and typecheck flows
  - `[x]` verify app launch smoke from a local machine with `npm run test:tauri-launch`
  - `[x]` verify clean bootstrap from an empty machine
  - `[x]` add cross-platform bootstrap workflow and evidence log
- `[x]` Templates and guardrails
  - `[x]` add issue templates
  - `[x]` add PR template
  - `[x]` document environment-example requirements for the current baseline

Exit criteria:

- a new contributor can follow the docs and get a green baseline setup on each target OS

## Phase 2: Shell app bootstrap `[x]`

Goal:

- create the minimal desktop application shell with a typed IPC example

Checklist:

- `[x]` App shell scaffold
  - `[x]` create `apps/main-app/`
  - `[x]` initialize React + TypeScript + Vite frontend
  - `[x]` initialize Tauri v2 runtime
  - `[x]` commit base app config files
- `[x]` Minimal UI
  - `[x]` render a Photo Mechanic-style workspace shell instead of a generic landing page
  - `[x]` add left sidebar with search plus Favorites, Navigator, and Tasks sections
  - `[x]` add top tab strip for workspaces or contact sheets
  - `[x]` add central blank workspace or empty contact-sheet state
  - `[x]` add bottom status bar for selection and task summaries
  - `[x]` make the initial `Untitled` workspace become the first opened folder tab
  - `[x]` open additional folders as new tabs by default
  - `[x]` support explicitly opening folders in separate windows
  - `[x]` show application version or environment information
  - `[x]` provide a visible health-check interaction
- `[x]` IPC baseline
  - `[x]` create a simple backend command such as `ping`
  - `[x]` call the command from the UI
  - `[x]` validate typed payloads and responses
- `[x]` Verification
  - `[x]` frontend build succeeds
  - `[x]` Rust build succeeds
  - `[x]` app launches locally
  - `[x]` smoke test validates the shell

Exit criteria:

- the app boots, renders a real window, and passes the baseline checks

## Phase 3: Local database and indexing foundation `[x]`

Goal:

- establish the persistent local catalog and schema evolution strategy

Checklist:

- `[x]` Database strategy
  - `[x]` choose the primary SQLite access layer
  - `[x]` record the DB access decision in `DECISIONS.md`
  - `[x]` define migration workflow
  - `[x]` define database location strategy per OS
- `[x]` Initial schema
  - `[x]` create `assets` table
  - `[x]` create variant or pairing tables
  - `[x]` create tags and join tables
  - `[x]` create keyword vocabulary tables
  - `[x]` create ingest session tables
  - `[x]` create task history tables
  - `[x]` create favorite folder or workset tables if the feature is persisted
  - `[x]` create audit log tables
  - `[x]` create preview cache reference tables if needed
- `[x]` Initialization flow
  - `[x]` initialize DB on app startup
  - `[x]` run migrations automatically
  - `[x]` enable WAL mode
  - `[x]` expose a typed repository layer
- `[x]` Task runtime baseline
  - `[x]` define task coordinator responsibilities
  - `[x]` define helper queue types for ingest, preview, metadata, rename, and delivery
  - `[x]` define task priority rules for interactive browsing versus background jobs
  - `[x]` define cancellation and retry state model
- `[x]` Verification
  - `[x]` unit test basic CRUD
  - `[x]` test migration application from a fresh DB
  - `[x]` test migration application from an older schema version
  - `[x]` test task-state persistence or publication rules

Exit criteria:

- the application creates and migrates a local DB successfully and tests validate the repository layer

## Phase 4: Filesystem scanning foundation `[-]`

Goal:

- scan existing folders into the local index without blocking the UI

Checklist:

- `[x]` Scanner behavior
  - `[x]` accept a user-selected root path
  - `[x]` recurse directories safely
  - `[x]` skip hidden or excluded files according to policy
  - `[x]` detect supported media types
  - `[x]` normalize and persist canonical paths
- `[x]` Asset classification
  - `[x]` detect RAW plus JPEG pairs
  - `[x]` detect XMP sidecars
  - `[x]` collect file stats needed for change detection
  - `[x]` queue metadata extraction
- `[-]` Progress and control
  - `[x]` emit progress updates
  - `[x]` support cancellation
  - `[ ]` record scan results in audit history
- `[-]` Verification
  - `[x]` unit test scanning with fixture folders
  - `[x]` test duplicate or collision handling
  - `[ ]` benchmark large-folder scan behavior

Exit criteria:

- a sample folder can be scanned into the DB accurately and repeatably

## Phase 5: Ingest MVP `[ ]`

Goal:

- ingest new files from cards, cameras, or import folders into the managed library

Checklist:

- `[ ]` Source discovery
  - `[ ]` support folder-based import
  - `[ ]` define camera or card detection strategy
  - `[ ]` define watched-folder and live-ingest policy
  - `[ ]` support ingest from current selection
  - `[ ]` support selecting multiple disks or folders in a single ingest session
- `[ ]` Planning
  - `[ ]` implement ingest session planner
  - `[ ]` support primary destination selection
  - `[ ]` support optional secondary destination selection
  - `[ ]` implement destination naming logic
  - `[ ]` implement source-structure preserve or merge rules
  - `[ ]` implement incremental ingest tracking
  - `[ ]` define post-ingest source handling such as unmount or cleanup behavior where supported
  - `[ ]` implement duplicate detection policy
  - `[ ]` implement collision policy
- `[ ]` Execution
  - `[ ]` implement ingest helper runtime
  - `[ ]` copy or move files safely
  - `[ ]` emit transfer progress
  - `[ ]` expose progress through a persistent task surface
  - `[ ]` open destination contact sheet while ingest is still running
  - `[ ]` extract metadata during or immediately after transfer
  - `[ ]` enqueue thumbnails and previews
- `[ ]` Recovery
  - `[ ]` handle partial failures without corrupting the library
  - `[ ]` support restart-safe ingest sessions
  - `[ ]` support auto ingest trigger behavior without duplicate runs
  - `[ ]` log per-file errors truthfully
- `[ ]` Verification
  - `[ ]` test import from a mock card directory
  - `[ ]` test ingest from selection
  - `[ ]` test live-ingest or watched-folder behavior
  - `[ ]` test multi-source ingest in one session
  - `[ ]` test dual-destination ingest
  - `[ ]` test duplicates
  - `[ ]` test interrupted ingest recovery

Exit criteria:

- new media can be ingested into the library with correct indexing and recoverable failure handling

## Phase 6: Fast browser and contact sheet MVP `[ ]`

Goal:

- provide a responsive large-library browsing experience

Checklist:

- `[ ]` UI foundations
  - `[ ]` implement contact sheet view
  - `[ ]` implement virtualized rendering
  - `[ ]` support multiple contact sheet tabs
  - `[ ]` preserve folder names as tab titles for open contact sheets
  - `[ ]` implement selection baseline
  - `[ ]` implement empty, loading, and error states
- `[ ]` Data flow
  - `[ ]` add paged or windowed asset queries
  - `[ ]` request thumbnails lazily
  - `[ ]` handle missing thumbnails gracefully
  - `[ ]` add sort controls
  - `[ ]` add refresh and rescan controls
- `[ ]` Interaction
  - `[ ]` add thumbnail resize controls
  - `[ ]` add customizable thumbnail labels
  - `[ ]` add hover or context quick actions
  - `[ ]` keyboard navigation
  - `[ ]` quick open preview
  - `[ ]` multi-select support
- `[ ]` Workspace panels
  - `[ ]` implement Navigator panel
  - `[ ]` implement Favorites panel
  - `[ ]` implement Tasks panel
  - `[ ]` support saved favorite folders or worksets
- `[ ]` File operations from the contact sheet
  - `[ ]` support drag and drop move or copy workflows
  - `[ ]` support RAW-plus-JPEG combined viewing preferences
  - `[ ]` support manual arrangement sort mode
  - `[ ]` support opening folders in a separate window when explicitly requested
- `[ ]` Verification
  - `[ ]` UI tests for visible-window rendering
  - `[ ]` UI tests for tabbed contact sheets and side panels
  - `[ ]` E2E smoke test with a large fixture set
  - `[ ]` performance baseline for scrolling
  - `[ ]` verify scrolling remains responsive while ingest or metadata helpers are active

Exit criteria:

- browsing hundreds or thousands of assets remains responsive and predictable

## Phase 7: Preview pipeline MVP `[ ]`

Goal:

- provide larger previews quickly for the current selection

Checklist:

- `[ ]` Preview generation
  - `[ ]` implement preview helper runtime
  - `[ ]` implement JPEG fast path
  - `[ ]` implement RAW decode path
  - `[ ]` define video preview strategy
  - `[ ]` scale to multiple preview sizes as needed
- `[ ]` Cache behavior
  - `[ ]` persist preview cache entries
  - `[ ]` invalidate cache on source change
  - `[ ]` prune stale cache entries
- `[ ]` UI integration
  - `[ ]` show selected asset preview
  - `[ ]` support one-up preview mode
  - `[ ]` support two-up comparison mode
  - `[ ]` support linked comparison panes
  - `[ ]` support fullscreen preview
  - `[ ]` support slide show mode
  - `[ ]` support live slide show mode for monitored folders
  - `[ ]` support zoom and pan
  - `[ ]` support highlight and shadow inspection
  - `[ ]` support direct tagging, rating, and color assignment from preview
  - `[ ]` show loading and failure states
  - `[ ]` prefetch nearby selections when justified
- `[ ]` Transform preview
  - `[ ]` support rotation and mirror preview state
  - `[ ]` support non-destructive crop overlay
  - `[ ]` support crop copy and paste workflows
- `[ ]` Verification
  - `[ ]` compare generated previews against fixtures
  - `[ ]` test cache hits and misses
  - `[ ]` test side-by-side preview comparison behavior
  - `[ ]` test slide show and live slide show behavior
  - `[ ]` test crop persistence and preview correctness
  - `[ ]` verify preview helper preempts lower-priority background jobs when browsing
  - `[ ]` measure preview latency targets

Exit criteria:

- the app can open previews for common assets quickly and repeatably

## Phase 8: Selection, rating, color, and tagging MVP `[ ]`

Goal:

- support the core culling workflow for large batches of photos

Checklist:

- `[ ]` Data model
  - `[ ]` add binary tag state
  - `[ ]` add ratings field
  - `[ ]` constrain star ratings to 1 through 5
  - `[ ]` add color class field with explicit naming support
  - `[ ]` support eight named color classes plus none
  - `[ ]` add keyword or tag tables and relations
  - `[ ]` define reject, keep, and tag interaction policy
- `[ ]` UI workflow
  - `[ ]` add toolbar controls
  - `[ ]` add single-key keyboard shortcuts
  - `[ ]` allow number-key behavior to be preference-driven where desired
  - `[ ]` define customizable shortcut infrastructure for major culling actions
  - `[ ]` support multi-select operations
  - `[ ]` show current metadata state clearly
- `[ ]` Persistence
  - `[ ]` save tag changes
  - `[ ]` save rating changes
  - `[ ]` save color class changes
  - `[ ]` save reject or keep changes if adopted
  - `[ ]` update search indexes after writes
- `[ ]` Selection tools
  - `[ ]` support select all tagged
  - `[ ]` support select by rating range
  - `[ ]` support select by color class
  - `[ ]` support invert selection
  - `[ ]` support filtering by tagged, untagged, rated, and colored states
- `[ ]` Verification
  - `[ ]` backend tests for write operations
  - `[ ]` UI tests for selection and shortcut behavior
  - `[ ]` test select-by-state behavior
  - `[ ]` persistence test after reload

Exit criteria:

- users can cull, rate, label, and tag assets efficiently and those changes persist correctly

## Phase 9: Metadata read and write MVP `[ ]`

Goal:

- support safe metadata editing with correct file and sidecar behavior

Checklist:

- `[ ]` Read path
  - `[ ]` parse EXIF
  - `[ ]` parse IPTC
  - `[ ]` parse embedded XMP
  - `[ ]` parse sidecar XMP
  - `[ ]` normalize fields into the DB model
- `[ ]` Field-family coverage
  - `[ ]` implement Tier 1 direct-edit field support
  - `[ ]` implement Tier 2 structured metadata models
  - `[ ]` implement Tier 3 preserve-through-roundtrip behavior for unsupported fields
- `[ ]` Write path
  - `[ ]` implement metadata helper runtime
  - `[ ]` define per-format write strategy
  - `[ ]` write JPEG metadata safely
  - `[ ]` create or update RAW sidecars
  - `[ ]` preserve non-targeted fields when writing
- `[ ]` Edit workflow
  - `[ ]` implement single-asset IPTC-style info dialog
  - `[ ]` implement bulk IPTC-style template dialog
  - `[ ]` support next and previous navigation during single-image editing
  - `[ ]` support template apply from within single-image editing
  - `[ ]` validation for date, GPS, controlled fields, and rights metadata
  - `[ ]` audit logging for metadata changes
- `[ ]` Template automation
  - `[ ]` define metadata variable grammar
  - `[ ]` support variable expansion in metadata templates
  - `[ ]` support loading and saving reusable metadata templates
  - `[ ]` support clipboard import and export of field data where desired
- `[ ]` Code replacements
  - `[ ]` define code replacement file format or internal representation
  - `[ ]` support loading and reloading code replacement sets
  - `[ ]` apply code replacements in single-image captioning flows
- `[ ]` Keyword and rights support
  - `[ ]` implement flat keyword editing
  - `[ ]` implement controlled keyword list management
  - `[ ]` implement structured keyword hierarchy support
  - `[ ]` support rights, licensor, and data-mining-permission fields
- `[ ]` Compatibility controls
  - `[ ]` define metadata compatibility presets or snapshots for other tools
  - `[ ]` define interoperability troubleshooting tools where real workflows require them
- `[ ]` Time and location utilities
  - `[ ]` implement capture-date adjustment for multi-camera sync
  - `[ ]` implement GPS metadata validation
  - `[ ]` define GPS log import workflow if in scope for MVP
  - `[ ]` implement reverse geocoding for location fields if the workflow includes it
- `[ ]` Verification
  - `[ ]` round-trip tests on temp files
  - `[ ]` sidecar creation tests
  - `[ ]` precedence rule tests
  - `[ ]` bulk-edit integration tests
  - `[ ]` code replacement tests
  - `[ ]` structured keyword tests
  - `[ ]` capture-time adjustment tests
  - `[ ]` reverse-geocoding workflow tests if implemented
  - `[ ]` parse and assert fields from `examples/XMP Side Car.XMP`
  - `[ ]` parse and assert mapped-property coverage from `examples/IPTC_Fields.XMP` against `IPTC_FIELD_MAP.md`
  - `[ ]` parse and assert Tier 1, Tier 2, and Tier 3 coverage from `examples/IPTC-PhotometadataRef-Std2025.1.jpg`

Exit criteria:

- metadata edits persist correctly in the DB and on disk according to documented precedence rules

## Phase 10: Batch rename and template MVP `[ ]`

Goal:

- support predictable bulk rename workflows with preview and collision handling

Checklist:

- `[ ]` Template system
  - `[ ]` define template grammar
  - `[ ]` define supported tokens
  - `[ ]` define sequence behavior
  - `[ ]` support metadata-driven variables used in Photo Mechanic-style workflows
  - `[ ]` define invalid-token error messages
- `[ ]` Rename planning
  - `[ ]` preview renamed outputs before commit
  - `[ ]` detect collisions before write
  - `[ ]` define overwrite or suffix policy
  - `[ ]` define rollback behavior on failure
- `[ ]` Execution
  - `[ ]` implement rename helper runtime
  - `[ ]` rename files atomically where possible
  - `[ ]` update DB paths transactionally
  - `[ ]` update related sidecar paths
  - `[ ]` log rename events
- `[ ]` Verification
  - `[ ]` template expansion unit tests
  - `[ ]` collision tests
  - `[ ]` DB path update tests
  - `[ ]` end-to-end rename preview and apply flow

Exit criteria:

- batch rename works safely, previews truthfully, and leaves DB state consistent

## Phase 11: Search, filter, and sort MVP `[ ]`

Goal:

- enable fast discovery across the local catalog

Checklist:

- `[ ]` Search schema
  - `[ ]` add FTS tables
  - `[ ]` add indexes for dates, ratings, labels, tags, and camera fields
  - `[ ]` add indexes or derived state for rotated, cropped, and tagged filters where justified
  - `[ ]` define incremental index update rules
- `[ ]` Query layer
  - `[ ]` implement text search API
  - `[ ]` implement Find on the active contact sheet
  - `[ ]` implement Find and Replace for supported metadata fields
  - `[ ]` implement filter query builder
  - `[ ]` implement sort options
  - `[ ]` define pagination or windowing behavior
- `[ ]` UI
  - `[ ]` quick search bar
  - `[ ]` advanced filters
  - `[ ]` sort controls
  - `[ ]` select matching results from Find
  - `[ ]` optionally filter the contact sheet to current Find results
  - `[ ]` empty-state and no-result messaging
- `[ ]` Verification
  - `[ ]` query correctness tests
  - `[ ]` indexing update tests after metadata edits
  - `[ ]` Find and Replace correctness tests
  - `[ ]` latency measurement for realistic libraries

Exit criteria:

- search and filters return correct results quickly at library scale

## Phase 12: Export, delivery, and external handoff MVP `[ ]`

Goal:

- generate derived outputs and delivery workflows safely with clear progress and error reporting

Checklist:

- `[ ]` Export model
  - `[ ]` define export job schema
  - `[ ]` define destination rules
  - `[ ]` define naming template behavior
  - `[ ]` define overwrite policy
- `[ ]` Export operations
  - `[ ]` implement Save As workflow
  - `[ ]` implement resize baseline
  - `[ ]` implement transcode baseline
  - `[ ]` implement watermark baseline
  - `[ ]` support multi-file export batches
  - `[ ]` define text-export workflow for reporting or downstream automation where useful
  - `[ ]` define video clip export behavior if video support is in scope
- `[ ]` Delivery workflows
  - `[ ]` implement delivery helper runtime
  - `[ ]` define uploader architecture for FTP, SFTP, or service integrations
  - `[ ]` implement print-proof or contact-sheet output strategy
  - `[ ]` implement email-delivery workflow or document why it is deferred
  - `[ ]` implement gallery-export workflow or document why it is deferred
- `[ ]` External handoff
  - `[ ]` implement external-editor launch preferences
  - `[ ]` implement paired RAW-plus-JPEG launch behavior
  - `[ ]` implement rescan or refresh after external edits
- `[ ]` UX
  - `[ ]` export dialog
  - `[ ]` progress reporting
  - `[ ]` completion summary
  - `[ ]` partial failure summary
- `[ ]` Verification
  - `[ ]` output correctness tests
  - `[ ]` watermark tests
  - `[ ]` external-editor handoff tests where automatable
  - `[ ]` upload or delivery integration tests where feasible
  - `[ ]` text-export tests if that workflow is implemented
  - `[ ]` video clip export tests if video support is implemented
  - `[ ]` long-running export progress tests
  - `[ ]` verify delivery helpers do not block preview scrolling or browsing

Exit criteria:

- selected assets can be exported or handed off with correct outputs and honest job reporting

## Phase 13: Performance pass `[ ]`

Goal:

- profile and optimize the critical user workflows after feature completeness

Checklist:

- `[ ]` Benchmark harness
  - `[ ]` create a representative fixture corpus
  - `[ ]` create ingest benchmark
  - `[ ]` create scan benchmark
  - `[ ]` create preview benchmark
  - `[ ]` create search benchmark
  - `[ ]` create concurrent-interaction benchmark for scrolling during helper activity
- `[ ]` Profiling
  - `[ ]` profile CPU hot paths
  - `[ ]` profile memory usage
  - `[ ]` profile DB contention
  - `[ ]` profile UI render bottlenecks
- `[ ]` Optimization work
  - `[ ]` batch DB inserts where needed
  - `[ ]` tune SQLite pragmas based on evidence
  - `[ ]` reduce redundant decode work
  - `[ ]` reduce unnecessary renderer updates
- `[ ]` Verification
  - `[ ]` record before and after metrics
  - `[ ]` preserve correctness while optimizing
  - `[ ]` document accepted tradeoffs

Exit criteria:

- measured performance meets the agreed targets for ingest, browsing, preview, and search

## Phase 14: Reliability pass `[ ]`

Goal:

- harden the application against broken files, interrupted work, and edge-case environments

Checklist:

- `[ ]` Broken input coverage
  - `[ ]` test corrupt JPEG handling
  - `[ ]` test corrupt RAW handling
  - `[ ]` test missing sidecar handling
  - `[ ]` test unsupported format behavior
- `[ ]` Environment coverage
  - `[ ]` test read-only destination behavior
  - `[ ]` test missing source files after external changes
  - `[ ]` test low-disk-space behavior where practical
  - `[ ]` test external drive disconnect behavior
- `[ ]` Recovery
  - `[ ]` validate interrupted ingest recovery
  - `[ ]` validate interrupted export recovery
  - `[ ]` validate interrupted upload or delivery recovery if those workflows exist
  - `[ ]` validate safe DB restart after crashes
  - `[ ]` validate panic or crash logging
- `[ ]` Verification
  - `[ ]` edge-case integration suite
  - `[ ]` manual exploratory pass for destructive flows
  - `[ ]` verify helper cancellation or retry leaves UI and task history consistent
  - `[ ]` update `DEBT_REGISTER.md` for known residual issues

Exit criteria:

- the app degrades gracefully under common failure modes and the known residual risks are documented

## Phase 15: Documentation pass `[-]`

Goal:

- keep project guidance truthful, detailed, and aligned with implementation reality

Current status notes:

- `[x]` the core doc set exists
- `[x]` the docs were expanded from research-summary placeholders into actionable reference docs
- `[ ]` implementation-specific examples cannot be finalized until code exists
- `[ ]` README-level onboarding is still missing

Checklist:

- `[-]` Core documentation
  - `[x]` expand architecture guidance
  - `[x]` expand planning guidance
  - `[x]` expand stack, testing, dependency, CI, security, and workflow guidance
  - `[x]` add detail to subsystem docs
  - `[x]` align roadmap, decisions, and debt register docs
- `[ ]` Contributor onboarding
  - `[ ]` add root `README.md`
  - `[ ]` add quick-start commands
  - `[ ]` add screenshots or interface references once the UI exists
- `[ ]` Implementation drift prevention
  - `[ ]` link docs to real workspace paths once scaffolded
  - `[ ]` update command examples after scripts exist
  - `[ ]` add docs verification tooling if desired
- `[ ]` Review
  - `[ ]` spell-check or markdown lint the full doc set in CI
  - `[ ]` perform implementation-versus-doc drift review before beta

Exit criteria:

- the documentation accurately reflects the repository and onboarding is complete

## Phase 16: CI hardening `[-]`

Goal:

- build a reliable CI pipeline that enforces the quality bar and supports release engineering

Checklist:

- `[-]` Baseline workflows
  - `[x]` add lint workflow
  - `[x]` add test workflow
  - `[x]` add build workflow
  - `[ ]` add docs validation workflow if adopted
- `[-]` Matrix coverage
  - `[x]` add Ubuntu jobs
  - `[x]` add macOS jobs
  - `[x]` add Windows jobs
- `[ ]` Performance and efficiency
  - `[x]` cache npm cache
  - `[ ]` cache cargo registry
  - `[ ]` cache cargo target intelligently
  - `[ ]` split fast and slow jobs
- `[ ]` Release support
  - `[ ]` add packaging job
  - `[ ]` add signing-ready workflow
  - `[ ]` add release dry run
  - `[ ]` define artifact retention policy

Exit criteria:

- CI blocks unverifiable merges and supports repeatable release preparation

## Phase 17: Cross-platform validation `[ ]`

Goal:

- verify consistent behavior across Linux, macOS, and Windows

Checklist:

- `[ ]` Platform setup
  - `[ ]` define supported OS versions
  - `[ ]` document platform prerequisites
  - `[ ]` validate WebView dependencies
- `[ ]` Workflow validation
  - `[ ]` ingest flow works on each OS
  - `[ ]` browse flow works on each OS
  - `[ ]` metadata flow works on each OS
  - `[ ]` export flow works on each OS
- `[ ]` Platform edge cases
  - `[ ]` path separator differences
  - `[ ]` case sensitivity differences
  - `[ ]` permission model differences
  - `[ ]` installer and packaging differences
- `[ ]` Verification
  - `[ ]` CI matrix validation
  - `[ ]` manual spot checks on packaged builds

Exit criteria:

- core workflows behave consistently across the supported desktop platforms

## Phase 18: Beta readiness `[ ]`

Goal:

- prepare the product and process for external early users

Checklist:

- `[ ]` Scope and quality
  - `[ ]` freeze beta scope
  - `[ ]` identify beta-blocking bugs
  - `[ ]` clear high-severity debt or document explicit exceptions
- `[ ]` User support
  - `[ ]` prepare quick-start guide
  - `[ ]` prepare issue templates for user feedback
  - `[ ]` define bug triage workflow
- `[ ]` Real-world validation
  - `[ ]` test with realistic photo libraries
  - `[ ]` collect feedback from photographers or representative users
  - `[ ]` document known limitations clearly

Exit criteria:

- a controlled set of users can install the app, complete core workflows, and report issues effectively

## Phase 19: Stable release readiness `[ ]`

Goal:

- complete the release engineering and quality work required for a trustworthy first stable release

Checklist:

- `[ ]` Release controls
  - `[ ]` lock release versions
  - `[ ]` review dependency health
  - `[ ]` run security audits
  - `[ ]` clear release-blocking bugs
- `[ ]` Packaging
  - `[ ]` sign Windows builds
  - `[ ]` sign and notarize macOS builds
  - `[ ]` validate Linux packaging outputs
- `[ ]` Release materials
  - `[ ]` write release notes
  - `[ ]` document upgrade path
  - `[ ]` publish known issues
- `[ ]` Final verification
  - `[ ]` full regression pass
  - `[ ]` final cross-platform validation
  - `[ ]` artifact smoke tests

Exit criteria:

- signed, tested release candidates are ready for public distribution

## Phase 20: Post-release operations `[ ]`

Goal:

- maintain product quality after the initial release

Checklist:

- `[ ]` Operational cadence
  - `[ ]` define issue triage schedule
  - `[ ]` define release cadence for patch updates
  - `[ ]` define benchmark regression cadence
- `[ ]` Maintenance
  - `[ ]` review dependency health regularly
  - `[ ]` pay down items from `DEBT_REGISTER.md`
  - `[ ]` refresh docs when workflows change
- `[ ]` Product planning
  - `[ ]` review roadmap after user feedback
  - `[ ]` plan next milestone set
  - `[ ]` record major new architectural decisions

Exit criteria:

- the team can triage, maintain, and evolve the product without losing verification discipline
