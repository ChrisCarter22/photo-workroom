# REPO_STRUCTURE.md

This document describes the target repository layout and the meaning of each top-level area.

Current repository status on March 9, 2026:

- the repository now contains a real workspace scaffold
- the top-level directories described below reflect the current tree, not just the intended target
- several directories are still placeholders until later phases fill them in

## Current state

Present today:

- root markdown documentation
- root workspace manifests including `package.json`, `package-lock.json`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `tsconfig.base.json`, and `eslint.config.js`
- `apps/main-app/` with renderer and Tauri shell code
- `crates/` with initial Rust subsystem boundaries
- `crates/db/migrations/` with the first SQLite migration files for Phase 3 bootstrap
- `tests/fixtures/metadata/` with promoted reference fixtures
- placeholder `packages/` and `benchmarks/` directories with checked-in README files
- `scripts/tauri-launch-smoke.mjs` for deterministic desktop launch verification
- `.github/` issue and pull-request templates plus the bootstrap validation workflow

Present supporting sample data today:

- `examples/`
  - currently contains `XMP Side Car.XMP` as a seed metadata sidecar example
  - currently contains `IPTC_Fields.XMP` as a broad IPTC XMP sidecar reference
  - currently contains `IPTC-PhotometadataRef-Std2025.1.jpg` as a broad IPTC JPEG reference
  - also contains `.DS_Store`, which is not product data and should be treated as incidental local filesystem noise
- `tests/fixtures/metadata/`
  - contains promoted copies of the three reference files above for future automated use

## Current structure

```text
/
├─ AGENTS.md
├─ README.md
├─ README_INDEX.md
├─ ARCHITECTURE.md
├─ PLANNING.md
├─ TECH_STACK.md
├─ REPO_STRUCTURE.md
├─ TESTING.md
├─ DEPENDENCY_POLICY.md
├─ CI_CD.md
├─ SECURITY.md
├─ DATA_MODEL.md
├─ INGEST_PIPELINE.md
├─ METADATA_SYSTEM.md
├─ IPTC_FIELD_MAP.md
├─ PERFORMANCE.md
├─ UX_WORKFLOWS.md
├─ CONTRIBUTING.md
├─ DECISIONS.md
├─ DEBT_REGISTER.md
├─ ROADMAP.md
├─ Cargo.lock
├─ package.json
├─ package-lock.json
├─ Cargo.toml
├─ rust-toolchain.toml
├─ tsconfig.base.json
├─ eslint.config.js
├─ apps/
│  └─ main-app/
│     ├─ package.json
│     ├─ src/
│     └─ src-tauri/
├─ examples/
├─ crates/
│  ├─ core/
│  ├─ db/
│  ├─ image/
│  ├─ ingest/
│  ├─ metadata/
│  └─ task_runtime/
├─ packages/
│  └─ README.md
├─ tests/
│  ├─ README.md
│  ├─ fixtures/
│  │  └─ metadata/
│  └─ validation/
│     └─ BOOTSTRAP_VALIDATION.md
├─ benchmarks/
│  └─ README.md
├─ scripts/
│  ├─ README.md
│  └─ tauri-launch-smoke.mjs
└─ .github/
   ├─ ISSUE_TEMPLATE/
   ├─ workflows/
   │  └─ bootstrap-validation.yml
   └─ PULL_REQUEST_TEMPLATE.md
```

## Workspace notes

- `apps/main-app/src-tauri/icons/icon.png` is currently a placeholder application icon for the bootstrap milestone
- `apps/main-app/src-tauri/icons/icon.ico` now exists to satisfy Windows Tauri resource builds
- `crates/db/src/lib.rs` now contains the local DB bootstrap boundary with startup migration and typed asset repository helpers
- `crates/task_runtime/src/lib.rs` now contains queue publication and task-state transition baseline rules for helper orchestration
- `crates/ingest/src/lib.rs` now contains a Phase 4 filesystem scan baseline with recursive traversal, classification, pairing, sidecar linking, and cancellation-aware progress updates
- app-local frontend tests live under `apps/main-app/src/` while root `tests/` currently holds shared fixtures and validation evidence logs
- generated runtime artifacts such as `node_modules/`, `target/`, `dist/`, and Tauri schema output are ignored and do not define the repository structure

## Root documentation files

Purpose:

- define architecture, planning, process, and policy before implementation grows
- give human contributors and coding agents a common baseline

Rules:

- keep these files at the repository root unless there is a documented reason to move them
- update them when architecture, workflow, or verification rules change
- avoid duplicating conflicting instructions in multiple places

## `apps/`

Purpose:

- contains deployable application entrypoints

Current implementation:

- `apps/main-app/` is the desktop application using React, TypeScript, and Tauri

Current contents:

- renderer code under `src/`
- Tauri backend under `src-tauri/`
- app-specific tests and configuration

## `crates/`

Purpose:

- contains reusable Rust libraries with clear ownership boundaries

Current crates:

- `core/` for domain logic and shared models
- `db/` for migrations and DB access
- `image/` for thumbnail, preview, and transform helpers
- `ingest/` for ingest planning and execution logic
- `metadata/` for metadata normalization and write-back
- `task_runtime/` for background helper orchestration, queueing, cancellation, and task-state publication

Rules:

- do not create cyclical dependencies
- keep Tauri-specific command handlers out of low-level crates
- prefer reusable domain APIs over direct UI-driven logic

## `packages/`

Purpose:

- contains shared JS and TS utilities only if sharing becomes necessary

Rules:

- do not create this area just to mirror common monorepo patterns
- add packages only when there is a real need for shared frontend code

## `tests/`

Purpose:

- contains integration, E2E, and cross-workspace fixtures

Current structure:

- `tests/fixtures/`
- `tests/integration/`
- `tests/e2e/`
- `tests/validation/`

Rules:

- keep fixture sizes reasonable
- separate unit tests that belong near code from shared higher-level tests
- promote stable sample files from `examples/` into `tests/fixtures/` once the scaffold exists and the files are part of automated verification

## `examples/`

Purpose:

- hold manually reviewed reference files before the formal test-fixture tree exists

Current known contents:

- `XMP Side Car.XMP`, which should be treated as a seed sidecar reference for metadata and sidecar parsing work
- `IPTC_Fields.XMP`, which should be treated as the broad XMP-sidecar reference for IPTC property coverage
- `IPTC-PhotometadataRef-Std2025.1.jpg`, which should be treated as the broad IPTC JPEG reference for metadata coverage

Rules:

- keep only intentional reference files here
- move stable examples into `tests/fixtures/metadata/` once automated tests exist
- do not treat editor or OS junk files as canonical examples

## `benchmarks/`

Purpose:

- hold performance fixture definitions and repeatable benchmark runners

Rules:

- use benchmarks only for workflows with stable measurement goals
- document the corpus and hardware assumptions when reporting benchmark results

## `scripts/`

Purpose:

- contain utility scripts for development, release prep, migrations, or data generation

Rules:

- prefer checked-in scripts over long undocumented shell snippets in PR descriptions
- keep script behavior explicit and safe

## `.github/`

Purpose:

- CI workflows
- issue templates
- PR templates
- release automation configuration

Rules:

- CI definitions belong here, not hidden in ad hoc contributor notes
- any required manual release steps should still be documented in markdown

## Naming conventions

Rules:

- Rust crates use `snake_case`
- JS and TS packages use `kebab-case`
- configuration files should follow tool defaults unless there is a strong reason not to
- directory names should reflect actual ownership and responsibility, not generic buckets

## Ownership and review boundaries

Expectations:

- root docs are cross-cutting and should be reviewed for consistency
- backend crates should be reviewable independently
- UI code should stay separated from privileged backend concerns
- if `OWNERS.md` is added later, update this file to reference it

## Monorepo guidance

Rules:

- prefer one clear workspace layout over several ad hoc roots
- use workspace-aware commands such as `npm --workspace <workspace> run <script>` once packages exist
- keep generated artifacts and caches out of version control
- do not create placeholder directories without an accompanying plan item or implementation step
