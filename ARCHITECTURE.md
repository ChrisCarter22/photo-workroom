# ARCHITECTURE.md

This document describes the target architecture for Photo Workroom.

Current repository status on March 8, 2026:

- the repository now contains a real workspace scaffold with `apps/main-app/`, Rust crates under `crates/`, root manifests, and GitHub templates
- the desktop shell baseline exists with a typed Tauri health-check command and a Photo Mechanic-style workspace layout stub
- most bounded contexts below remain target-state guidance beyond the current shell bootstrap

## System goals

The system is a local-first desktop application for high-volume photo workflow operations.

Primary goals:

- ultra-fast ingest from cards, folders, and watched locations
- fast culling and contact-sheet browsing at large library scale
- correct metadata read, edit, merge, and write-back behavior
- responsive preview generation for JPEG, RAW, video, and paired assets
- local search, filter, and sort with durable indexing
- maintainable boundaries for long-horizon growth
- safe operation by coding agents and human contributors
- auditable, testable, cross-platform architecture

## Reference product target

The current product benchmark is standard Photo Mechanic feature parity for the core photographer workflow.

That means the product direction should explicitly cover:

- ingest from cards, folders, selections, and optionally watched paths
- ingesting multiple disks or folders in a single session when the workflow calls for it
- fast tabbed contact-sheet browsing with strong keyboard support
- culling with tags, star ratings, and named color classes
- fast preview and side-by-side comparison
- slide show and live slide show modes for review and tethered-style workflows
- IPTC-style metadata authoring for single and batch workflows
- variable-driven automation for rename, metadata, output, and labels
- controlled keyword workflows, including structured keywords
- Save As, export, watermarking, upload, print, and delivery workflows
- GPS, reverse geocoding, capture-time correction, and external-editor handoff workflows
- metadata compatibility controls for working with other applications
- video browsing and clip-export workflows where media support is in scope
- deep shortcut and preference customization for deadline-oriented users

Explicit non-target for now:

- although Camera Bits currently offers Photo Mechanic Plus, Plus-style cross-drive catalog parity is not the baseline target until this project has a stable, first-party implementation strategy for that catalog scope

## Non-goals

- cloud-first product behavior
- collaborative multi-user server architecture
- full non-destructive RAW editor behavior similar to darktable or Capture One
- AI-heavy analysis features in the initial milestones
- browser-only or mobile-first delivery
- arbitrary plugin loading or remote-code marketplaces

## Architectural principles

- local-first by default
- metadata correctness over convenience shortcuts
- performance as a first-class design constraint
- explicit module boundaries
- narrow dependency direction
- verification-first development
- small, composable pipelines
- no hidden network dependence
- least-privilege desktop integration
- deterministic writes and recoverable failure handling

## Bounded contexts and modules

### 1. Ingest pipeline

Purpose:

- accept assets from cards, cameras, folders, or watched paths
- plan copy or move operations into the library
- detect duplicates, collisions, and sidecars before mutating state

Responsibilities:

- source discovery
- ingest session planning
- ingest from selection support
- multi-source ingest across disks or folders in one session
- incremental ingest tracking
- auto ingest and live ingest modes
- primary and secondary destination planning
- folder merge and source-structure preservation rules
- destination path computation
- checksum or duplicate detection strategy
- transfer execution and progress reporting
- early open of destination contact sheets while ingest continues
- handoff to metadata, preview, and indexing pipelines

### 2. Metadata engine

Purpose:

- normalize metadata from file, sidecar, and database sources
- preserve round-trip fidelity wherever the file format supports it

Responsibilities:

- EXIF, IPTC, and XMP reads
- precedence rules and merge strategy
- write-back to JPEG or other writable formats
- XMP sidecar creation and maintenance for RAW formats
- IPTC-style template application for batch edits
- single-item metadata authoring flow
- variable expansion for metadata templates
- code replacement support for repetitive captioning
- structured keyword and controlled vocabulary support
- reverse geocoding support for populating location fields
- compatibility controls and troubleshooting behavior for working with other applications
- rights and licensing metadata fields
- audit logging for metadata changes

### 3. Preview pipeline

Purpose:

- generate fast thumbnails and larger previews without blocking the UI

Responsibilities:

- queued preview requests
- RAW decode strategy
- JPEG and PNG fast path
- optional video frame extraction and preview handling
- one-up and two-up comparison support
- slide show and live slide show support
- crop overlay and transform-preview support
- zoom, pan, and highlight or shadow inspection modes
- cache invalidation when source files change

### 4. Index and search pipeline

Purpose:

- maintain the local queryable catalog

Responsibilities:

- SQLite schema and migrations
- full-text search tables
- filter indexes for dates, ratings, labels, tags, and camera fields
- active contact-sheet Find and Find-and-Replace workflows
- refresh and rescan behavior for folder-backed contact sheets
- arrangement or manual sort persistence where supported
- synchronization after ingest, metadata edits, rename, and delete flows

### 5. UI frontend

Purpose:

- render workflow-driven views for import, browse, cull, edit, search, and export

Responsibilities:

- application shell with a Photo Mechanic-style workspace layout
- contact sheet and preview panels
- Navigator, Favorites, and Tasks side panels
- tabbed folder and workset browsing
- customizable thumbnail labels and overlays
- keyboard-driven workflow interactions
- configurable shortcuts and preference snapshots
- local UI state and unsaved edit state
- typed IPC calls for privileged backend work

Default shell layout direction:

- left sidebar for search, Favorites, Navigator, and Tasks
- top tab strip for open contact sheets or workspaces
- center content region for the active contact sheet, preview, or blank workspace
- bottom status bar for selection counts, task summaries, and lightweight state feedback

Shell behavior rules:

- the initial blank `Untitled` workspace is a placeholder for the first opened folder
- opening a folder into a blank workspace should replace the placeholder with a named contact sheet tab
- opening additional folders should create additional contact sheet tabs by default
- separate windows should be available intentionally for users who want parallel workspaces, but the default browsing model remains tab-first

The goal is the same workflow structure as the reference app, not a pixel-for-pixel visual clone.

### 6. Export pipeline

Purpose:

- generate derived files safely and predictably

Responsibilities:

- export job planning
- filename templates
- resize and transcode operations
- watermark application
- Save As-style output variants
- print, email, and uploader workflows
- text-export and reporting-style output where workflow automation benefits from it
- video clip export where media support is in scope
- web-gallery or proof-output workflows if adopted
- external-editor handoff integration
- progress tracking and retry-safe error handling

### 7. Task coordinator and helpers

Purpose:

- keep long-running work off the UI thread while preserving fast browsing and preview interaction

Responsibilities:

- maintain explicit task queues for ingest, preview, metadata, rename, and delivery work
- enforce priority rules so visible preview and browsing work beats bulk background jobs
- support cancellation, retry, pause, and resume where the workflow requires it
- publish progress and error state into the Tasks surface and task history model
- isolate helper failures so one bad job does not stall unrelated work

Expected helper types:

- ingest helper for copy, move, watched-folder, and ingest-from-selection workflows
- preview helper for thumbnails, visible previews, compare mode, and crop-preview updates
- metadata helper for sidecar reads, write-back, batch metadata application, and code-replacement-aware saves
- rename helper for template expansion, collision checks, filesystem rename, and DB path updates
- delivery helper for Save As, watermarking, FTP or upload, email, gallery, and print-output jobs
- filesystem watch helper for live-ingest and rescan signals when that workflow is enabled

### 8. Extension surface

Purpose:

- enable future growth without exposing the core to arbitrary code execution

Responsibilities:

- sidecar or process-based integrations only
- stable JSON or IPC contracts
- signed or otherwise trusted binaries for privileged integrations

## Desktop runtime responsibilities

The desktop shell is expected to be Tauri v2 unless superseded by a later ADR.

Responsibilities:

- window lifecycle and menus
- native dialogs and limited filesystem access
- IPC boundary definition
- application packaging and updater integration
- WebView sandboxing and content security policy enforcement

## Rust core responsibilities

Rust owns privileged and performance-sensitive behavior:

- filesystem operations
- metadata reads and writes
- database access and migrations
- preview generation and heavy image transforms
- long-running background work, task coordination, and helper scheduling
- structured logging and audit events
- validation of every IPC request coming from the renderer

## TypeScript UI responsibilities

The frontend owns presentation and workflow orchestration:

- rendering the application shell and workflow screens
- collecting user intent and validating form input
- managing selection state, draft state, and view configuration
- calling typed backend commands for privileged work
- surfacing progress, warnings, and recoverable errors truthfully

## SQLite database and local index

SQLite is the default local index and persistence layer.

It stores:

- canonical asset records
- variant relationships such as RAW plus JPEG pairs
- tags, ratings, labels, keywords, captions, and search text
- ingest sessions and export jobs
- task history for ingest, upload, and delivery operations
- favorites, worksets, or other saved browsing context if productized
- audit log entries
- cache references where a DB-backed index is useful

Schema expectations:

- versioned migrations
- WAL mode during normal runtime
- explicit transactions for multi-step mutations
- append-only audit records for user-visible state changes

## Filesystem abstraction

The system relies on native OS filesystem access through Rust.

Rules:

- all privileged filesystem access flows through backend commands
- paths are normalized before persistence
- rename, move, and write operations prefer write-to-temp-then-rename when possible
- path handling must remain correct on macOS, Windows, and Linux
- network paths are not assumed to be reliable or supported by default

## Pipelines summary

Ingest pipeline:

- trigger from manual import, camera detection, or watched folder event
- discover candidate files
- compute destination plan
- transfer files
- extract metadata
- create DB entries
- enqueue thumbnails and previews
- emit audit log events

Metadata pipeline:

- read metadata from files and sidecars
- normalize fields into domain models
- apply user edits transactionally
- write back to file or sidecar
- update search indexes and audit log

Preview pipeline:

- accept preview requests from visible UI state
- check on-disk cache
- decode and resize if cache miss
- persist cache entry
- stream result back to the renderer

Export pipeline:

- build export plan from selection and template rules
- generate outputs to temporary paths
- finalize outputs atomically
- log completion or recoverable per-file failures

## Concurrency model

Concurrency rules:

- heavy background tasks run off the UI thread
- preview and visible-thumbnail requests have higher priority than bulk background work
- rename, metadata, and delivery helpers must not freeze contact-sheet scrolling or preview navigation
- helper queues should be separated by workload type rather than merged into one opaque worker pool
- worker counts are capped to avoid disk thrashing
- SQLite runs with one writer and many readers where practical
- file operations and DB changes are grouped so partially completed work can be resumed or rolled back safely
- UI commands remain asynchronous and never block on large scans or preview generation

## Caching strategy

Preview cache:

- previews and thumbnails are stored on disk, not embedded in SQLite blobs by default
- cache keys should include path identity plus change detection data such as size, timestamp, or checksum
- stale cache entries are eligible for background pruning

In-memory cache:

- only currently visible or recently viewed previews remain in memory
- avoid whole-library in-memory caching

Metadata cache:

- once metadata has been normalized into the DB, avoid re-reading the source file unless change detection indicates drift

## Error model

Recoverable errors:

- unreadable metadata
- unsupported formats
- duplicate files
- per-file export failures

Critical errors:

- database corruption
- failed migration
- cache or working directory permission failures that block startup

Rules:

- log all errors with actionable context
- continue processing other assets when safe
- surface critical failures clearly in the UI
- never silently drop mutations or pretend writes succeeded

## Observability

Observability is local-first and privacy-preserving.

Required behavior:

- write structured logs locally
- record user-visible operations in an audit table
- preserve timestamps, operation type, and failure details
- avoid remote telemetry by default

## Security boundaries

Security boundary model:

- the Rust core is trusted and privileged
- the WebView renderer is treated as less trusted
- only explicit IPC commands may cross the boundary
- no general filesystem or shell escape hatches are exposed to the UI
- no remote script loading or unrestricted plugin execution is permitted

## Upgrade and migration strategy

Data and application upgrades must be explicit:

- use incremental SQLite migrations
- record schema version in the database
- run migrations on startup before normal operations continue
- document all architectural changes in `DECISIONS.md`
- treat incompatible data changes as release-blocking work until migration and rollback plans exist

## Target repository shape

The intended repository layout is documented in `REPO_STRUCTURE.md`. The current repository does not yet implement that tree.

## Dependency direction rules

Required direction:

- UI code may call backend commands, but backend code must not depend on renderer code
- lower-level Rust crates must not import higher-level application shells
- domain logic should remain reusable across command handlers
- documentation may reference the target structure, but implementation must follow actual module boundaries once created

## Anti-patterns to avoid

- monolithic business logic in the UI
- direct file mutation from the renderer
- duplicated metadata rules across frontend and backend
- hidden network requirements for core workflows
- speculative dependencies with unclear ownership
- skipping verification in the name of speed
