# INGEST_PIPELINE.md

This document defines the target ingest pipeline for Photo Workroom.

Current repository status on March 9, 2026:

- `crates/ingest` now includes a Phase 4 filesystem scan baseline with recursive traversal, hidden or excluded policy handling, media classification, RAW/JPEG pairing, XMP sidecar linking, metadata-queue planning, and cancellation-aware progress callbacks
- transfer execution, ingest-session persistence, and recovery behavior remain target-state work for Phase 5

## Goals

The ingest pipeline must support:

- card ingest
- folder ingest
- duplicate destination ingest
- explicit user-selected primary destination roots
- optional user-selected secondary destination roots
- optional watched-folder or live ingest
- fast progress visibility
- metadata extraction during or immediately after ingest
- safe recovery from partial failures
- auditability

## High-level pipeline

The target ingest sequence is:

1. discover source media
2. plan the ingest session
3. validate destination and naming rules
4. copy or move files safely
5. extract metadata
6. create DB records
7. enqueue previews and thumbnails
8. log outcomes and expose a summary

## Entry points

Supported entry modes should eventually include:

- manual import from a selected folder
- import from a card or camera source if supported
- watched-folder import if the product decides to include it

Each mode must converge on the same core ingest planner rather than duplicating logic.

## Detailed stages

### 1. Source discovery

Responsibilities:

- enumerate candidate files
- ignore unsupported files or excluded paths
- detect sidecars
- group likely RAW and JPEG pairs

### 2. Ingest session planning

Responsibilities:

- assign a session ID
- determine destination root from an explicit user choice
- support an optional second destination chosen by the user in the same ingest session
- compute destination filenames and folder structure
- detect collisions before writes begin
- present a truthful plan to the user where the UI requires it

User-visible planning controls should eventually include:

- source selection from folder, card, camera, or current selection
- direct choice of the primary ingest destination folder
- optional direct choice of a secondary backup or duplicate destination folder
- rename template selection
- metadata preset selection
- duplicate and collision handling policy

### 3. Transfer execution

Responsibilities:

- copy or move files according to the selected mode
- preserve timestamps when policy requires it
- report per-file and aggregate progress
- avoid leaving half-written destination files in final locations

### 4. Metadata extraction

Responsibilities:

- read embedded metadata
- read sidecar metadata when present
- normalize fields into the ingest result model
- record unreadable or unsupported metadata as recoverable warnings

### 5. Persistence

Responsibilities:

- insert or update asset records
- link variants and sidecars
- record ingest session history
- enqueue downstream preview generation

### 6. Completion and recovery

Responsibilities:

- produce a final success or partial-failure summary
- log skipped or failed files
- support safe retry behavior
- keep the DB and filesystem in a consistent state

## Duplicate handling

Duplicate policy must be explicit. The system should eventually define:

- what counts as a duplicate
- whether duplicates are detected by checksum, metadata, path, or a combination
- whether the user can skip, import anyway, or link to existing records
- how duplicate decisions are logged

## Destination naming rules

The ingest planner should support:

- template-driven file naming
- sequence numbers where needed
- metadata tokens such as date or camera fields
- collision previews before committing writes

## Error handling

Recoverable errors:

- unreadable metadata
- unsupported file type
- duplicate detected according to policy
- per-file copy failure

Critical errors:

- destination unavailable
- DB transaction failure that invalidates the ingest session
- permission failure that blocks all writes

Rules:

- never silently skip errors
- continue other files when safe
- expose partial success honestly
- log enough detail to retry or debug

## Concurrency and performance

Rules:

- scanning and copy operations must not block the UI
- worker concurrency should be bounded to avoid disk thrashing
- metadata extraction may run during or immediately after transfer if ordering is explicit
- bulk DB writes should use transactions

## Related modules

The ingest pipeline depends on:

- filesystem access in the backend
- metadata normalization
- DB persistence
- preview queue integration
- audit logging

## Verification expectations

Target tests:

- import from a fixture folder
- duplicate handling scenarios
- collision scenarios
- interrupted ingest recovery
- audit log correctness

Target performance checks:

- large folder scan throughput
- ingest throughput under realistic media counts
