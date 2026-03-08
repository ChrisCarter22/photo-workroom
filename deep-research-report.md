# Executive Summary  

Current planning note:

- this research draft still contains historical `pnpm` references
- the accepted repository baseline is now `npm`; see `DECISIONS.md`, `TECH_STACK.md`, and `DEPENDENCY_POLICY.md` for the current source of truth

We recommend a **Tauri+Rust core with React/TypeScript frontend** and **SQLite** as the local database, following a local-first, high-performance design.  Tauri’s architecture (Rust backend, OS-native WebViews) yields small app size and low memory use【32†L168-L174】【46†L361-L369】, while Rust ensures computational efficiency for large photo libraries.  React + Vite is chosen for wide familiarity and TypeScript safety; alternatives like Svelte or Vue were considered but React’s ecosystem and agent-friendliness prevail.  Image/media handling will use **LibRaw** for RAW decoding and **rexiv2/Exiv2** for metadata read/write【37†L304-L312】.  We enforce strict coding standards: only use `pnpm` (no `npm install`), pin dependency versions, run full lint/typecheck/tests on each change, and explicitly forbid skipping or silencing failures.  The repository will use a **monorepo layout** (apps, crates, tests, docs) with clear module boundaries (e.g. `crates/` for Rust logic, `apps/` for frontend).  Every pipeline (ingest, preview, search, export) is architected as a bounded module.  We will document every decision and technical debt with ADRs and a debt register, ensure CI/CD gates lint, tests, builds across OS, and include benchmarks.  The **Agents.md** will mandate the full verification flow (lint, typecheck, tests, builds) and forbid partial work.  This suite of docs and rules forms a blueprint that coding agents (e.g. Codex) can follow to build a robust, auditable Photo Mechanic–like app without being allowed to bypass errors or inflate dependencies.

# Research and Source-Pattern Synthesis  

We surveyed **Tauri and desktop-framework docs**, **large desktop/web-photo apps (Immich, Lap, digiKam)**, and **monorepo best practices**. Key patterns emerged:

- **Tauri + Rust Backend:** Projects like [Lap](https://github.com/julyx10/lap) (an offline photo manager) use Tauri 2 + Rust + SQLite【46†L361-L369】.  Tauri’s model uses OS WebView (Edge/Safari/WebKit) for UI, avoiding bundling Chromium【32†L168-L174】.  Benchmarks show Tauri apps are tiny (~8–10 MiB) and use far less memory (e.g. 172 MB vs 409 MB for Electron【32†L168-L174】).  We adopt Rust for performance-critical tasks (indexing, preview generation), and React/TypeScript for UI. Alternatives (Electron, Qt, etc.) were rejected due to bulk (Electron) or agent compatibility (native C++).  

- **Local-First and Privacy:** digiKam’s docs emphasize **no telemetry or data sharing**【13†L139-L147】.  Similarly, we treat “no cloud” as a goal (like [Lap] which is *“Private by default”*【46†L281-L289】).  All user data (photos, metadata) stays on disk; any optional plugin exporting data must be opt-in.  Tauri’s security model also segregates trusted Rust code from the WebView, enforcing permissions on IPC calls【40†L208-L216】, which we will leverage to restrict risky actions.  

- **Metadata and Raw Handling:** digiKam uses **LibRaw** and Exiv2 for full RAW and metadata support【15†L414-L421】. We mirror this: use LibRaw (with its wide RAW format support【21†L25-L33】) and a Rust binding to Exiv2 (the [rexiv2 crate](https://github.com/felixc/rexiv2)【37†L304-L312】) for reading/writing EXIF, XMP, IPTC.  This covers the “16-bit enabled” and “exif/IPTC” needs noted in digiKam【13†L113-L118】.  For ingestion rename templates, digiKam’s advanced import shows allowing *“Customize” with metadata fields* in the renaming mask【19†L105-L110】; we adopt a similar templating approach.  

- **Architecture and Concurrency:** We structure the app into clear modules (bounded contexts) inspired by Immich’s hexagonal design【9†L174-L183】 and Tauri’s trust boundaries【40†L208-L216】.  Example modules: *FileSystem/Scanning*, *Ingest Pipeline*, *Metadata Engine*, *Preview Generator*, *Search/Indexing*, *Export Pipeline*, and *UI layer*. Dependencies flow inward (UI → core services → storage); we forbid circular or cross-cutting imports. Concurrency is managed carefully: as one Reddit discussion notes, scanning and thumbnail gen must balance parallelism vs I/O contention【27†L123-L132】. We plan to use Rust’s async/thread pools and SQLite pragmas (e.g. WAL) for efficiency.  

- **Monorepo and CI Patterns:** We adopt a **pnpm workspaces** monorepo (like the modern TS monorepo example【41†L473-L482】).  Layout: top-level manifests, with subdirs for each package/crate.  Tools: ESLint + Prettier (or Biome) for JS/TS, `cargo fmt`+Clippy for Rust.  Tests: Vitest/Jest for UI logic, Playwright for E2E, Rust’s builtin tests (or cargo-nextest).  CI: matrix builds (Linux/Windows/macOS) with lint, typecheck, unit, integration, e2e tests (similar to typical Node/Rust CI patterns【41†L472-L481】).  We will cache pnpm store and Rust target across workflows.

- **Observability & Logging:** We found no direct analog in photo apps, but will combine standard logging libraries (e.g. Rust `log`/`env_logger` or tracing, and console logs on UI). Events like ingesting/deleting images are logged to the audit model.  Digital asset managers often have an “Audit” or “History” feature, which we’ll emulate in `AuditLog` entries in the DB.

- **UX Workflows:** Common flows (card import, select/batch edit, metadata editing, search) are documented in digiKam’s and Lap’s docs. We note the patterns:
  - Renaming templates (digiKam’s advanced import【19†L105-L110】).
  - Ratings/tags filters (digikam tags search【15†L397-L405】).
  - Live import (folder watching like “Folder Ingest”).
  - External editor handoff (digiKam and others use XDG or OS open-with).
  - Watermarking and export pipeline placeholders (since digiKam has many export plugins【15†L427-L430】). 

**Adopted Patterns:**  
- **Documentation for Agents:** Inspired by the [AGENTS.md format][3], [Vitest’s agent guide][7], etc., we will write precise, prescriptive instructions focusing on commands, test flows, and strict “Definition of Done”.  
- **Monorepo Layout:** We use distinct `apps/` vs `crates/` vs `packages/` dirs, following the Lap repo’s separation of `src-tauri/` (Rust) and `src-vite/` (UI)【46†L243-L252】, and guidance from pnpm workspace examples【41†L508-L517】.  
- **Backend Design:** We adopt Immich’s principle of hexagonal architecture (separating storage, services, controllers)【9†L175-L183】 in our modules.  
- **Testing**: The Vitest example emphasizes running *all* tests and not mocking core code【7†L43-L52】【7†L79-L88】. We similarly require complete test runs.  
- **Dependency Control:** We will mirror pnpm best practices (no `npm install`, only `pnpm install` as in [AGENTS.md examples][3] and monorepo guides【41†L508-L517】).  

**Anti-Patterns to Avoid:**  
- Including heavy runtimes (no bundling Node or Chromium – Tauri avoids this【32†L129-L137】【32†L138-L146】).
- Unchecked dependency bloat (no duplicating functionality in multiple libraries).
- Silently swallowing errors or skipping tests (we explicitly forbid these in agents’ workflow).
- Leaving architecture unstated or inconsistent (we’ll document module boundaries).
- Telemetry or data leaks (digiKam’s “no telemetry”【13†L139-L147】 is our model).  

# Recommended Tech Stack  

- **Desktop Shell:** **Tauri v2.0**.  Provides a Rust backend and uses native WebViews (Edge/WebKit) instead of bundling Chromium, yielding very small binaries (~10 MB) and low RAM use【32†L168-L174】. Tauri’s security model segregates Rust core vs JS UI【40†L208-L216】 and supports sidecar processes (for future plugins)【34†L292-L302】. Electron was rejected (huge bundle, Node runtime overhead【32†L168-L174】).  
- **Language:** **Rust 2021** for all performance-sensitive code (indexing, I/O, image processing). **TypeScript (strict mode)** for UI logic. We avoid less-typed or slower languages.  
- **UI Framework:** **React + Vite**. React’s ecosystem (Redux/Context, React Query, etc.) and large community make it a safe choice. We considered Vue and Svelte (Lap uses Vue【46†L363-L366】) but React has the widest support and tooling. Vite is the build tool (fast cold starts). For styling, plain CSS or Tailwind (TBD).  
- **Database:** **SQLite** (likely via `sqlx` or `rusqlite` in Rust). SQLite is embedded, fast on local disk, and supports FTS for search. It aligns with Tauri’s official guides for persistent storage. For indexing large libraries, use WAL and tuned pragmas.  
- **Metadata:** Use **rexiv2 (Exiv2)** for reading/writing EXIF, IPTC, XMP【37†L304-L312】. It has broad format support (JPEG, RAW sidecars, etc.). LibRaw handles RAW decoding【21†L25-L33】. We may optionally shell out to ExifTool only for edge cases, but prefer a native lib.  
- **Thumbnail/Preview:** For fast previews, use **stb-image** or Rust’s `image` crate (libjpeg, libpng, etc.) and `libwebp` for conversions. Thumbnails will be generated on ingest (in Rust) and cached. We might also leverage GPU by using image downsampling libraries if needed (e.g. `libvips` or Rust’s SIMD features).  
- **Parallelism:** Rust’s async (Tokio) or thread pools for scanning/processing. We’ll benchmark I/O vs CPU tradeoffs (as noted by [Lap’s author][27]).  
- **Plugin Architecture:** Tauri supports **sidecar binaries**【34†L292-L302】. We plan a minimal plugin API (e.g. external scripts for custom uploaders), but core functionality is in the base app. Plugins must run with user’s consent and be sandboxed via Tauri’s permission model.  
- **Tests:**  
  - **Unit/Integration:** Rust: built-in `cargo test` (or cargo-nextest) for core crates. JS/TS: **Vitest** (fast, TypeScript-friendly) for frontend logic.  
  - **E2E:** **Playwright** against the Tauri app (simulate user flows).  
  - **CI Testing:** All tests must run headlessly on Linux/Win/macOS.  
- **Lint/Format:**  
  - TS: **ESLint** (with TypeScript plugin, Airbnb or custom rules) and **Prettier** (or new **Biome.js** if stable).  
  - Rust: **rustfmt** and **Clippy**.  
- **Package Management:** **pnpm** workspaces for JS/TS. For Rust, Cargo workspaces for crates. No `npm install` allowed. See *DEPENDENCY_POLICY.md*.  
- **Logging & Observability:** Rust: `tracing` or `log4rs` writing to file. UI: console logs disabled in production, but errors reported via controlled mechanism. We minimize telemetry.  
- **CI/CD:** GitHub Actions (or equivalent). Matrix over OS. Caching pnpm store and `target/`. Pre-merge must pass lint, build, test. Releases: build signed installer (use Tauri’s bundler for each OS) and publish artifacts (with code signing keys under secrets).  
- **Approved Dependencies:** We will only add libraries after a Dependency Decision Record (DDR) evaluation. Core crates: libraw, rexiv2, rusqlite/sqlx, tokio, image-processing crates. UI: React, React Router, React Query, etc. Avoid monolithic frameworks (e.g. we reject adding lodash for one function, prefer native JS). Full table in *TECH_STACK.md*.  

# Proposed Repository Structure  

```
/ (root)
  /apps
    /main-app/              # Tauri + React app 
      src-tauri/            # Rust Tauri code (API commands, sidecar setup)
      src/                  # React UI (TypeScript + Vite)
      package.json
      tauri.conf.json
  /crates
    /core/                  # Rust library core (ingest, metadata, etc.)
    /db/                    # Rust DB schema/migrations (if separate crate)
    /image/                 # Rust image processing utilities
    ...                     # Other domain-specific crates
  /packages                 # Optional: JS/TS shared libraries (if needed)
  /tests
    /fixtures/             # Test images, metadata for automated tests
    /unit/                 # Integration/unit test scripts not in code repos
  /docs                    # Architecture.md, data models, pipeline docs, etc.
  /scripts                 # Build & release scripts (signing, packaging)
  AGENTS.md
  ARCHITECTURE.md
  DATA_MODEL.md
  DEPENDENCY_POLICY.md
  TESTING.md
  CI_CD.md
  CONTRIBUTING.md
  ROADMAP.md
  DECISIONS.md
  DEBT_REGISTER.md
  package.json            # workspace config
  pnpm-workspace.yaml
  rust-toolchain          # pin Rust version
  .github/                # CI configs, issue templates, etc.
```

This layout mirrors examples like [Lap](https://github.com/julyx10/lap) which had separate `src-tauri`/`src-vite` sections【46†L243-L252】, and pnpm monorepo examples【41†L508-L517】. We keep most code under `apps/` and `crates/`; docs and scripts are top-level.  Each crate/app has its own AGENTS.md if needed.

# AGENTS.md  

> **For coding agents (e.g. Codex)**  
> This document defines how AI coding agents must work on this codebase. Agents **must** read and follow the guidelines here and the core docs (`ARCHITECTURE.md`, `TECH_STACK.md`, `TESTING.md`, `DEPENDENCY_POLICY.md`, `PLANNING.md`) before making changes.  

## Agent Operating Contract  
- Always confirm you understand the task. Inspect the current code and docs (`arch`, `stack`, etc.) before coding.  
- Identify which module/files the task affects. Analyze impact and write/update a detailed implementation plan.  
- Implement **minimally necessary** changes. Prefer updating existing code over adding new abstractions. Break tasks into small commits (atomic, reversible).  
- **No speculative rewrites:** Do not rewrite large code unless required by the plan.  
- Use `pnpm` for JavaScript/TypeScript dependency work. **Never use `npm install`.** (See DEPENDENCY_POLICY.)  
- Justify **every** new dependency in a Dependency Decision Record. Check existing packages first.  

## Workflow Steps  
1. **Understand Task:** Summarize what’s needed, list affected areas.  
2. **Inspect Code & Tests:** Run `pnpm lint`, `pnpm build`, `pnpm test` (and `cargo test`) to get a clean baseline. Identify failing tests/errors if any exist prior to changes.  
3. **Module Impact:** Identify which modules/classes/functions to change. Plan minimal edits.  
4. **Write/Update Plan:** Outline approach in comments or doc. (Agents should ‘think’ in the code or docs, not just code.)  
5. **Implement & Test:** Make changes. After each logical chunk:  
   - Run `pnpm lint`/`eslint --fix` for UI code.  
   - Run `cargo fmt --check` and `cargo clippy` for Rust.  
   - Run `pnpm typecheck` (or `tsc`) and `cargo check`.  
   - Run `pnpm test` (unit tests) and `pnpm test:e2e` (if applicable).  
   - Run `cargo test`.  
   - Run `pnpm build` and `cargo build`.  
6. **Fix Failures:** If any check fails, **stop further work** on new features. Perform root-cause analysis.  
   - If failure is new, fix the code to make all tests pass.  
   - If failure is pre-existing, isolate and document it.  
   - In all cases, do **not** suppress or skip tests to “work around” failures.  
7. **Verify & Document:** When all checks pass, update relevant docs (comments, ARCHITECTURE, DATA_MODEL, etc.) to reflect changes. Summarize what was done and any residual risks.  
8. **Submit Work:** Prepare for PR. Include brief commit messages. Ensure CI will pass (lint/build/tests).

## Definition of Done  
A task is **complete only when**:  
- All code compiles.  
- Linting yields no new warnings (unless auto-fixed).  
- Type-check passes with no errors.  
- All unit/integration/e2e tests pass.  
- Any new features have associated tests.  
- Docs have been updated for any architectural, API, or workflow changes.  
- The implementation is minimal and clearly fits the design.  

**Agents must not consider a task done if any verification step is failing or skipped.**

## Error Handling Contract  
- **Do not swallow errors:** All runtime errors should be addressed (fix code or write explicit error handling).  
- **Testing failures:** If a test fails, do not comment it out. Investigate and fix or file a documented issue.  
- **Lint/typefail:** Do not disable linting or type rules to force a build. Instead, fix or request a short-lived exception (with DDR & debt entry).  
- **Build failures:** If `pnpm build` or `cargo build` fails, troubleshoot until passing.  
- **Known issues:** If a genuine blocker arises that cannot be fixed in this task, do the following *before* pushing:  
  1. Document the issue (exact failure output) in `DEBT_REGISTER.md`.  
  2. Note whether it was pre-existing or new.  
  3. Minimize scope of changes around the problem.  
  4. Leave code in a safe, compile-able state (possibly with `if/else` stub and `todo`).  
  5. Obtain sign-off via a Decision Record if altering architecture is required.  

## “When Blocked”  
If you cannot proceed due to unclear requirements or missing info, do:  
- State the blocker explicitly (e.g. “Cannot find `MetadataWriter` behavior described”).  
- Provide evidence (error logs, test failure output).  
- Verify if the issue is pre-existing (revert your changes to see if failure still occurs).  
- Narrow the problem scope (comment only related code to isolate).  
- Add an entry in **DEBT_REGISTER.md** or **DECISIONS.md** describing the problem, with owner and criteria to revisit.  
- Ensure the codebase remains buildable (mark incomplete features with TODO and debt log).

## Additional Rules  
- **No bypass:** Under no circumstances should you:  
  - Skip or narrow test suites without justification.  
  - Comment out failing assertions or tests.  
  - Disable linting or type rules.  
  - Push code that hides errors (e.g. try/catch without handling).  
  - Declare “manual verification” without automated tests.  
- **Small, Reversible Changes:** Each change should be simple, one-purpose, and easily undone if needed.  
- **Documentation Sync:** After any code change affecting behavior or design, update relevant markdown docs (architecture, data models, UX flows, etc.).  
- **Example Commands:** Agents should use these commands to verify their work:  
  - **Install deps:** `pnpm install` (root, or with `--filter` for a subpackage)  
  - **Lint:** `pnpm lint`  
  - **Type-check:** `pnpm typecheck` (or `pnpm tsc --noEmit`)  
  - **Unit tests:** `pnpm test`  
  - **Integration tests:** e.g. `pnpm test:integration`  
  - **E2E tests:** e.g. `pnpm test:e2e` (Playwright)  
  - **Rust tests:** `cargo test` (or `cargo nextest run`) in each crate.  
  - **Build:** `pnpm build`, `cargo build --release`  
  - **Full verify:** combination of all above (scripts are provided in `package.json`/`Makefile`).  

## Summary Report  
After completing a task, include a summary with:  
- Changes made (what code, what tests).  
- Tests added/updated.  
- If any documentation was changed.  
- Any remaining TODOs or risks (they must be documented in `DEBT_REGISTER.md`).  
- Confirmation that you ran all checks (list command outputs succinctly).  

Agents must double-check this `AGENTS.md` on every task to ensure compliance. 

# ARCHITECTURE.md  

## Goals  
- **Ultra-Fast Workflow:** Ingest thousands of images (from cards or watched folders) with minimal delay; fast contact-sheet browsing and preview generation; instant metadata editing and search.  
- **Local-First & Privacy:** All data (photos, metadata, configs) resides on the user’s machine. No telemetry or automatic cloud uploads【13†L139-L147】【46†L281-L289】.  
- **High Performance:** Rust-powered core for heavy lifting (decoding, indexing). Async or multi-threaded pipelines to leverage multi-core without saturating disks.  
- **Metadata Correctness:** Preserve and correctly merge metadata from EXIF, IPTC, XMP, and user edits. Round-trip fidelity (writing back to files/sidecars).  
- **Extensibility:** Plugin-safe design for future features (export to new services, additional formats). Use clear extension points (IPC commands, sidecars).  

## Non-Goals  
- **Cloud Dependency:** No built-in server or sync to cloud (local export only). *Offline operation by default*【46†L281-L289】.  
- **Heavy Image Processing:** No integrated “develop” stack (this is a workflow tool, not a raw editor like darktable). Minimal processing (auto-rotate, minor transforms) only; external editors handle heavy edits.  
- **Large Background ML:** (Facial recognition, etc., are out of scope initially). The core system may define hooks (e.g. for “smart albums”) but not implement them immediately.  
- **Non-GUI Interfaces:** (No built-in mobile or web client; desktop-first only.)  

## Bounded Contexts / Modules  
1. **Ingest Pipeline:** Handles new images from cards, folders, or live watch.  
   - *Functions:* Copy/Move files to library location; detect duplicates; read initial metadata; create thumbnails; insert into DB.  
   - *Interfaces:* Scanning threads, camera connection (via OS/gPhoto), filesystem events.  
2. **Metadata Engine:** Central code for reading/writing metadata.  
   - *Components:* Exif reader, XMP writer, DB sync.  
   - *Rules:* Precedence: on ingest, file EXIF/XMP → initial DB state; on save, DB changes → update sidecar (if exists) and file EXIF. Configurable fallback if sidecar missing.  
3. **Preview Pipeline:** Generates scaled previews for browsing (thumbs, contact sheets).  
   - *Async:* Worker threads to decode (LibRaw or JPEG) and resize (libvips or Rust `image`).  
   - *Cache:* Previews stored in `.cache/` (e.g. in user data dir) keyed by checksum or timestamp.  
4. **Index/Search Pipeline:** Builds and updates searchable index (SQLite).  
   - Tables: assets, tags, ratings, keywords, ingest sessions, logs.  
   - Uses SQLite FTS (full-text search) for captions and tags, plus indices on dates, ratings for filtering.  
   - On changes (tagging, caption edit), update index immediately.  
5. **UI Frontend:** React/TypeScript app in Tauri WebView.  
   - Communicates via IPC commands (as defined in `src-tauri`) for any privileged action (file I/O, DB queries).  
   - Presents multiple tabs/panels: Import, Browser, Edit Metadata, Export, etc.  
6. **Export Pipeline:** Abstracts saving variants:  
   - *Formats:* JPEG, TIFF, DNG, etc. (using LibRaw for RAW re-encode, or Rust `image`).  
   - *Watermarking:* Optional step (using Rust image library or a sidecar process like ImageMagick).  
   - *Destinations:* Local folder or external (future plugins).  
7. **Plugin/Extension API:**  
   - Minimal core: define an IPC extension point or Tauri sidecar spec.  
   - Plugins run as separate processes (signed binaries) or as loaded WASM (if Tauri supports).  
   - Core provides hooks but no web-based plugin loading (avoid remote code).  

## Desktop Runtime Responsibilities (Tauri)  
- Window management, Tray icon (optional), menus.  
- Expose IPC commands for filesystem access (only as needed).  
- Application updates (Tauri’s Updater, optional; or delegate).  
- Secure webview sandboxing (CSP, no Node in renderer by default).  

## Rust Core Responsibilities  
- File I/O, metadata read/write (using rexiv2, libraw).  
- Image processing pipelines (thumbnails, previews, format conversions).  
- Database access (rusqlite/sqlx).  
- Long-running tasks (scanning, background indexing) with error resilience.  
- Logging (to file or DB).  
- Security: validate all IPC requests, enforce permissions.  

## TypeScript UI Responsibilities  
- Render interface: contact sheets, lists, filters, metadata forms.  
- Handle user input: file dialogs, drag-drop, edit fields.  
- Local state for unsaved changes, UI config.  
- Dispatch IPC calls for backend work.  
- Client-side validations (e.g. filename templates).  

## SQLite Database (Local Index)  
- Store asset metadata (paths, EXIF fields, captions, tags, ratings, color labels).  
- Link RAW+JPEG pairs: a one-to-many relation (master asset with variants).  
- Track sidecar existence/paths.  
- Ingest Sessions: for auditing.  
- Preview cache references? (or keep previews purely on disk).  
- Schema migration: versioned migrations directory (embed with `sqlx migrate`).  

## Filesystem Abstraction  
- We rely on native OS calls (Rust std fs), augmented by gPhoto or relevant libs for camera import.  
- Local-first means no network FS by default.  
- Ensure paths are handled cross-platform (Tauri’s Path API).  

## Pipelines Summary  
- **Ingest:** Trigger (card insert or folder watch) → copy files → create DB entries → extract metadata → generate previews.  
- **Metadata:** On import or manual edit → update DB + file.  
- **Preview:** Monitor DB for new assets → spawn thumb generation (threaded).  
- **Search:** Index updated automatically via triggers (after each metadata change or new asset).  
- **Export:** User action triggers pipeline (reads from DB, writes files).  

## Concurrency Model  
- Use **Rust async or thread pool** for I/O-bound tasks. Balance pool size to prevent disk thrashing (controlled by config, e.g. `num_cpus` threads).  
- UI commands must be non-blocking (await IPC). Heavy tasks post notifications.  
- SQLite access: either use a single writer with `BEGIN IMMEDIATE` or a pool of readers. WAL mode allows one writer, many readers.  
- Ensure database and file writes are atomic (use transactions for DB; write to temp then rename for files).  

## Caching Strategy  
- **Previews:** Store generated images (JPEG) in a cache dir (e.g. `~/.local/share/ourapp/cache`). Check file hash/timestamp to skip regen.  
- **In-memory:** Minimal caching (UI-level component cache for visible images). Heavier caching discouraged to save memory.  
- **Metadata:** Once read into DB, avoid re-reading file metadata unless changed (DB is source of truth).  

## Error Model  
- Isolate recoverable errors (e.g. failing to read a file’s metadata → log and proceed).  
- Critical errors (DB corruption) should surface a user-visible alert.  
- On crashes, generate a crash dump (Rust panic hook writing to log).  

## Observability  
- All important events (ingest started/completed, error, export success) logged to an *Audit* table and log file.  
- Use Tauri’s built-in logging plugin or Rust logging.  

## Security Boundaries  
- **Rust core** (trusted) vs **WebView UI** (untrusted sandbox). Only expose needed APIs.  
- No remote code execution: UI cannot `eval` arbitrary scripts. CSP ensures only app’s code runs.  
- No Node integration in renderer by default (Tauri isolates JS).  
- Must use `invoke()` (IPC) for file/DB actions, controlled by explicit allow-lists in `tauri.conf.json`.  

## Upgrade/Migration Strategy  
- **Data migrations:** Use SQLite’s pragma user_version and migration scripts. Include incremental SQL files.  
- **Versioning:** Semantic versioning of app; document upgrade path in releases.  
- App binaries auto-update via Tauri Updater (with user opt-in). Ensure migrations run on startup if needed.  

## Example Repository Tree (illustrative)  

```text
my-photo-app/
├── apps/
│   └── main-app/
│       ├── package.json
│       ├── tauri.conf.json
│       ├── src-tauri/        # Rust code
│       └── src/              # React UI code
├── crates/
│   ├── core/                 # Rust core logic
│   ├── image/                # Rust image processing helpers
│   └── db/                   # Rust DB schema/migrations
├── docs/                     # Markdown docs (architecture, data model, etc.)
├── scripts/                  # Build/release scripts
├── tests/                    # E2E and performance tests, fixtures
├── .github/                  # CI/CD workflows
├── AGENTS.md
├── ARCHITECTURE.md
└── ...
```

_Dependency Direction Rule:_ Rust crates under `crates/` should **not** depend on UI code. UI can call into core via IPC only. Similarly, no cyclical crate dependencies.  

**Anti-patterns to Avoid:** Monolithic single-file logic; scattering business logic in the UI; shipping unused libraries; ignoring failing CI.  

# PLANNING.md  

This section outlines the **phased development plan** from initial setup through to stable release and maintenance. Each phase has *goals, tasks, affected files, tests, risks, dependencies,* and *exit criteria*. We assume agile iterations.

---

## Phase 0: Repository Bootstrap  
**Goals:** Initialize repository structure, CI, linters, tools.  
**Tasks:**  
- Create repo scaffold (as per recommended structure above). Commit empty `apps/main-app`, `crates/`, etc.  
- Set up `package.json` workspace (pnpm) and `Cargo.toml` workspace.  
- Add `AGENTS.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.  
- Install/lock core tools: Node (LTS), pnpm, Rust stable. Pin versions in `engines` and `rust-toolchain`.  
- Add basic lint config: ESLint, Prettier, tsconfig.base.json, rustfmt.  
- Configure GitHub Actions CI (lint, typecheck, unit tests for a “hello world” dummy).  
- Configure PR/check policies (require passing CI).  
- Set up Dependabot or similar for dependency monitoring.  
**Affected Files:** root docs, `package.json`, workspace config, `.github/workflows/pr.yml`, linter configs.  
**Tests:** Verify linters can run on an empty app; CI pipeline setup (create a dummy test to ensure success).  
**Risks:** Misconfiguring workspace (pnpm filters, Yarn vs pnpm confusion). CI failures block subsequent steps.  
**Exit Criteria:** All CI jobs for default branch (build CI jobs for lint and test) pass. Agent instructions confirmed.  

---

## Phase 1: Dev Environment Hardening  
**Goals:** Ensure development environment consistency and tooling.  
**Tasks:**  
- Update README and CONTRIBUTING with setup steps (install Node 18+, pnpm, Rust).  
- Install linting and formatting rules (e.g. run `pnpm lint --fix`).  
- Commit pre-commit hook config (e.g. lint-staged or simple script).  
- Add a sample `.env.example` if needed.  
- Validate cross-platform tooling (CI matrix).  
- Set up issue and PR templates (code guidelines).  
**Affected Files:** `.github/*`, `README.md`, `CONTRIBUTING.md`, lint config.  
**Tests:** Running `pnpm lint`, `cargo fmt --check`, `cargo clippy` should pass on the initial code.  
**Risks:** Flaky environment differences (Node vs pnpm version). Document any OS-specific steps in wiki.  
**Exit Criteria:** Developers following the instructions (on Linux, Win, Mac) get a green CI on a trivial change.  

---

## Phase 2: Shell App Bootstrap  
**Goals:** Create the minimal app skeleton: one window with an empty React page.  
**Tasks:**  
- Use Tauri CLI (`create-tauri-app`) or manual setup: integrate a blank React+TS frontend with Tauri.  
- Verify `tauri::Builder` launches a window.  
- Add a simple UI with a splash/home screen.  
- Connect frontend to backend: define a dummy IPC command (e.g. `ping()` in Rust, called from TS).  
- Lint and build pipeline: `pnpm build`, `cargo build`.  
**Affected Files:** `apps/main-app/src-tauri/src/main.rs`, `apps/main-app/src/index.tsx`, routing config.  
**Tests:**  
   - **Unit:** Verify IPC command returns as expected.  
   - **Build:** `pnpm build` yields a static bundle (via Vite); `cargo build` creates a Tauri dev binary.  
   - **Smoke:** Launch app (headlessly) and check window title or DOM content via automation.  
**Risks:** Tauri CLI version mismatch or misconfiguration (e.g. `tauri.conf.json` settings for icons/CSP).  
**Exit:** App launches with blank UI and CI pipeline passes.  

---

## Phase 3: Local Database Foundation  
**Goals:** Set up SQLite integration and schema structure.  
**Tasks:**  
- Choose crate for DB (e.g. `sqlx` with `sqlx migrate` or `rusqlite`).  Initialize a `schema.sql`.  
- Define tables: `assets(id, path, created, camera, type, ...)`, `raw_variants(parent_id, path)`, `tags`, `ratings`, `keywords`, `ingest_sessions`, `log_entries`.  
- Create Rust module for DB access. Initialize DB file on app launch (e.g. under `~/.local/share/MyApp/`).  
- Implement a simple “DB version” check.  
- Write a small test: add a dummy asset entry, query it back.  
**Affected Files:** `crates/db/*`, migrations, Tauri command to initialize DB on startup.  
**Tests:**  
   - **Rust Unit:** `cargo test` for DB module (create, insert, query).  
   - **Integration:** Tauri app startup invokes DB init (simulate via integration test or manual check of file creation).  
**Risks:** SQLite file path issues per OS; version mismatches. Use Tauri API for platform directories.  
**Exit:** On first run, an empty DB file is created with correct tables (verify by reading file); tests confirm basic CRUD.  

---

## Phase 4: Filesystem Scanning  
**Goals:** Implement folder scanning to populate DB from existing photos.  
**Tasks:**  
- Recursively scan a target folder (user selects it via dialog).  
- For each image/RAW file found: extract basic metadata (timestamp, EXIF id, etc.), insert asset and variants into DB.  
- Handle RAW+JPEG pairing: if a RAW has matching JPEG (same name), link them.  
- Skip hidden files. Recognize duplicates (by file hash or name) and optionally log them.  
- Store absolute paths or URIs for each asset.  
- Emit progress updates (e.g. via IPC to show progress bar).  
**Affected:** `crates/ingest`, Tauri commands, UI (select directory).  
**Tests:**  
   - **Unit:** Test scan function on a fixture directory with known files (see `tests/fixtures`).  
   - **Performance:** Simple benchmark (1000 sample images) to gauge scan time.  
**Risks:** Very large folders slow down UI if blocking; must run asynchronously. External drive latency (like SD card). Possibly use incremental scanning.  
**Exit:** DB is populated correctly from a sample folder; tests for expected rows in DB and correct file associations.  

---

## Phase 5: Ingest Pipeline MVP  
**Goals:** Enable efficient import from camera or card, with duplicate handling.  
**Tasks:**  
- Integrate with OS camera APIs or gPhoto to detect camera/carded device.  
- On card detection or user command, copy images to target folder (optionally renaming with template).  
- Use metadata in filename templates (year, camera, exif fields) as in Digikam【19†L105-L110】.  
- After copying, run same processing as folder scan to index them.  
- Implement duplicate detection: skip or mark duplicates (simple name or checksum check).  
**Affected:** `crates/ingest`, UI import dialog.  
**Tests:**  
   - Import from a mock “card” directory of photos; verify files copied to target structure and indexed.  
   - Duplicate file scenario: same image twice should be flagged.  
**Risks:** gPhoto driver quirks (digiKam notes RAW metadata may not be readable via gPhoto【19†L125-L134】). Solution: if metadata unavailable, rely on DB or manual rename after import.  
**Exit:** Importing from a camera yields the new images in the library and in DB; duplicates are not double-added. All related tests (copy, metadata extraction, DB insert) pass.  

---

## Phase 6: Fast Browser / Contact Sheet MVP  
**Goals:** Show large sets of images quickly.  
**Tasks:**  
- UI: a grid view (contact sheet) displaying thumbnails (fast-loading).  
- Backend: generate or fetch cached thumbnails (e.g. 256px).  On missing thumbnail, queue generation.  
- Virtual scrolling: only render visible rows.  
- Pagination/filter buttons.  
- Sorting options (by date, name, rating).  
- Quick navigation (e.g. calendar view selector).  
**Affected:** UI components (Grid/VirtualList), `crates/preview` code.  
**Tests:**  
   - **UI E2E:** Mock DB with 1000 assets; ensure scrolling remains responsive and thumbnails show up.  
   - **Unit:** Generate a thumbnail for a test image and verify dimensions.  
**Risks:** Generating many images could block. Ensure preview generation is asynchronous and cached. Use web worker or Rust threads.  
**Exit:** Browsing 500+ images feels responsive; automated scroll/load tests pass; memory usage is bounded (old pages dropped).  

---

## Phase 7: Preview Pipeline MVP  
**Goals:** High-quality previews for RAW and video.  
**Tasks:**  
- Implement previewer: use LibRaw to decode RAW and save a JPEG thumbnail or 8-bit image.  
- For video (if any), extract first frame (using `ffmpeg` or similar) for thumbnail.  
- In Rust, provide a function to get a preview buffer given an asset ID.  
- Integrate with UI: show a larger preview on hover/selection.  
**Affected:** `crates/image`, Tauri command (getPreview), UI (Image component).  
**Tests:**  
   - Compare a generated JPEG preview to an expected image (fixtures).  
   - Video preview test (if sample video provided).  
**Risks:** Raw decode failures for some formats; build LibRaw with necessary codecs. Video uses external binary – must package it or instruct user.  
**Exit:** Previews match sample outputs; tests for typical RAW/JPEG assets succeed.  

---

## Phase 8: Selection/Rating/Tagging MVP  
**Goals:** Basic culling UI: select images, assign stars/colors/tags, mark rejects.  
**Tasks:**  
- UI: allow clicking images (or checkboxes), buttons for star/color/tag. Multi-select behavior.  
- DB: add `rating` (1–5 stars) and `colorLabel` enums to asset table; `tags` table with many-to-many.  
- IPC commands to update these fields in DB.  
- Persist selection UI state in memory.  
**Affected:** UI (Selection panel, metadata dialogs), `crates/core` (models for rating/tag).  
**Tests:**  
   - **Unit:** Tag and rate an asset via backend function; verify DB updated.  
   - **UI E2E:** Simulate user marking 3 stars and tags, then reload app – state should persist.  
**Risks:** Concurrency issues if multiple selections change same DB row simultaneously. Use transactions.  
**Exit:** Users can assign and view ratings/tags; DB and UI stay in sync. All related tests pass.  

---

## Phase 9: Metadata Read/Write MVP  
**Goals:** Edit metadata (title, caption, keywords, GPS, etc.) and save to files/sidecars.  
**Tasks:**  
- UI forms to edit common metadata fields. Bulk-edit support (apply to multiple images).  
- On save: update DB and write changes back.  Strategy: for JPEG, embed in EXIF/XMP; for RAW, write to sidecar XMP.  
- Use **rexiv2** to perform writes, ensuring IPTC/XMP compliance.  
- Capture corner cases: preserve or merge existing sidecar XMP.  
**Affected:** `crates/metadata` (new module), UI (Metadata editor).  
**Tests:**  
   - **Unit:** Write metadata to a temp image file; read it back (via rexiv2 or ExifTool) and verify fields.  
   - **Integration:** Edit via UI and check on disk (possibly via running `exiftool` as a check).  
**Risks:** Metadata libraries might not support all formats (JPEG is fine, some RAW might need exiftool). We may need fallback.  
**Exit:** Metadata fields persist in DB and in image files/sidecars. All tests (field round-trips) succeed.  

---

## Phase 10: Batch Rename/Templates MVP  
**Goals:** Implement filename templating for bulk rename.  
**Tasks:**  
- UI: dialog to specify rename mask (with tokens for date, time, camera model, sequence, etc.)【19†L105-L110】.  
- Backend: apply template per asset, renaming file on disk and updating DB path.  Handle conflicts by appending suffix or skipping.  
- Tests for rename: simulate pattern and check results.  
**Affected:** `crates/utils` or `crates/core` (rename logic), UI (batch rename dialog).  
**Tests:**  
   - Rename by {YYYY}-{MM}-{DD}_{orig}.jpg and verify file name changes, DB updates.  
   - Case sensitivity test, overwrite conflict resolution.  
**Risks:** Renaming moves file – must handle failing halfway (use atomic rename, or rename then update DB).  
**Exit:** Users can preview and commit batch renames; no data loss. Automated rename tests pass.  

---

## Phase 11: Search/Filter/Sort MVP  
**Goals:** Fast searching by text and filtering (date, camera, tag, rating, GPS).  
**Tasks:**  
- UI: search box (full-text), and filter panels (tag checkboxes, star selector, date range, map view for geo).  
- Backend: use SQLite FTS for caption/tags search; normal queries with WHERE for filters. Spatial: if images have GPS, simple bounding-box search or integrate R-Tree index.  
- Index new fields as needed.  
**Affected:** DB schema (FTS virtual tables), UI (search bar).  
**Tests:**  
   - Insert sample data and run queries programmatically; compare results.  
   - E2E: index contains known keyword; searching yields correct images.  
**Risks:** FTS performance on large DBs – test with 50k entries. Ensure indices on filter fields (dates, rating).  
**Exit:** Searching returns expected results quickly; response <100ms for thousands of items. All search tests green.  

---

## Phase 12: Export/Save-As MVP  
**Goals:** Export selected images (with rename, format conversion, watermark).  
**Tasks:**  
- UI: export dialog to choose destination, filename template, format (JPEG/WebP), add watermark text or image.  
- Backend: take each asset, read original (for RAW use LibRaw to DNG/JPEG; for JPEG just re-save or compress), apply watermark (e.g. via Rust `image` overlay or a CLI tool), write to new location.  
- Multi-thread export jobs; show progress.  
**Affected:** `crates/export`, UI (export wizard).  
**Tests:**  
   - Export a sample image to a different format; verify output file (using image metadata or visual diff).  
   - Batch export with watermark; check watermark presence.  
**Risks:** Performance (bulk re-encoding is slow). Allow configuring lower thread usage.  
**Exit:** Exports complete without errors; testing files match expected outputs (checksum or OCR watermark).  

---

## Phase 13: Performance Pass  
**Goals:** Optimize critical slow paths after feature-complete.  
**Tasks:**  
- Profile memory and CPU (use real library). Identify bottlenecks (e.g. image loading, DB locks).  
- Implement improvements: e.g., batch DB inserts, use memory-mapped IO, tune SQLite pragmas (journal=OFF during initial scan, WAL for runtime).  
- Consider WebAssembly or SIMD for hot loops.  
- Add cache for repeated metadata reads.  
**Affected:** Code hot paths.  
**Tests:**  
   - Benchmark import (1000 RAWs), browse (scroll 5000 thumbs), search (100k entries) before/after.  
   - Ensure no regressions.  
**Risks:** Premature optimization – measure only. Complex optimizations require careful testing.  
**Exit:** Performance metrics meet targets (e.g., initial import < x sec, scroll/fluidity measured stable). Benchmarks documented.  

---

## Phase 14: Reliability and Edge Cases Pass  
**Goals:** Fix crashes and handle unusual files.  
**Tasks:**  
- Test with corrupted images, read-only folders, missing files (deleted outside app).  
- Ensure graceful errors (notify user, skip item).  
- Add retry or recovery for database failures.  
**Affected:** Error handling code, possibly add more tests.  
**Tests:**  
   - Introduce corrupted JPEG/RAW and ensure app logs error but continues.  
   - Simulate power loss (kill process mid-import) and check on restart (partial writes cleaned).  
**Risks:** Uncaught panics. Use `std::panic::set_hook` to log panics.  
**Exit:** No reproducible crashes or hangs under error scenarios; quality tests passed.  

---

## Phase 15: Documentation Pass  
**Goals:** Complete writing of all required docs (this output).  
**Tasks:**  
- Review each Markdown for clarity, add missing details.  
- Ensure architecture/stack docs match implementation.  
- Update README with sample screenshot and quick start.  
**Affected:** All `.md` files.  
**Tests:**  
   - Spell-check, link-check. 
   - Technical reviewer reads docs for correctness.  
**Exit:** Docs cover all planned features, link references added, guidelines clear.  

---

## Phase 16: CI Hardening  
**Goals:** Finalize CI/CD for release and maintenance.  
**Tasks:**  
- Add end-to-end tests (Playwright) to CI.  
- Cache pnpm and cargo effectively.  
- Configure release builds: signing setup (import code-sign certs from secrets).  
- Automate changelog generation (e.g. commit-based or manual entry).  
- Add branch protection rules.  
**Affected:** `.github/workflows/*`, CI scripts.  
**Tests:** 
   - Merge from feature branch triggers all checks.  
   - Successful build artifacts and release draft upon tag.  
**Risks:** CI flakiness. Timeout on Windows/macOS. Increase timeouts or use hosted runners.  
**Exit:** CI green on dry run of release (e.g. run all jobs on a mock release tag).  

---

## Phase 17: Cross-Platform Validation  
**Goals:** Ensure app works on Windows, macOS, Linux.  
**Tasks:**  
- Set up VM or toolchain testing on each OS (use Actions or physical machines).  
- Verify file dialog behavior, filesystem paths, path separators.  
- Test standard flows on each (import, browse, export).  
- Fix any OS-specific bugs (e.g. case-insensitive filenames on Windows, path encodings).  
- Prepare installers: NSIS (Windows), DMG (macOS), AppImage/DEB (Linux).  
**Affected:** Platform-specific code (e.g. file dialog implementation).  
**Tests:**  
   - Manual: run installer on each OS, perform core tasks.  
   - Automated: e2e scripts if possible on all OS.  
**Risks:** Differences in lib availability (e.g. WebKit versions). Document requirements (like Lap requiring `libwebkit2gtk-dev`【46†L337-L344】).  
**Exit:** App runs natively on all three OS with identical behavior.  

---

## Phase 18: Beta Readiness  
**Goals:** Prepare for beta release to early users.  
**Tasks:**  
- Freeze feature set.  
- Conduct a beta test (select users).  
- Address feedback (bugs, UI clarifications).  
- Finalize all user-facing documentation (quickstart, usage guides).  
- Set up issue triage/labeling workflow.  
**Affected:** Possibly UI/UX tweaks, minor features.  
**Tests:**  
   - Usability testing (observational studies).  
**Risks:** Unexpected UX issues; apply changes carefully to not break core.  
**Exit:** Positive user feedback, no showstopper bugs.  

---

## Phase 19: Stable Release Readiness  
**Goals:** Final polish for official 1.0 release.  
**Tasks:**  
- Lock versions (update lockfiles, Cargo.lock).  
- Security audit (review dependencies via cargo-audit, npm audit).  
- Audit critical code (GDPR/data, encryption of any stored credentials).  
- Sign builds with code signing keys.  
- Write release notes (features, known issues).  
- Publish on GitHub releases and package repositories (e.g. Homebrew, apt if applicable).  
**Affected:** Version files, release pipeline.  
**Tests:**  
   - Final regression testing.  
**Exit:** All checks green, release candidate builds signed and published.  

---

## Phase 20: Post-Release Operations  
**Goals:** Support and maintain.  
**Tasks:**  
- Monitor issue tracker; fix critical bugs.  
- Plan minor updates vs backlog (use `ROADMAP.md`).  
- Periodic performance regression tests (re-run key benchmarks before each release).  
- Review technology dependencies annually (update Rust, libraries).  
- Document technical debt and pay down items from `DEBT_REGISTER.md` as planned.  
- Community guidelines: manage contributions, documentation updates.  
**Risks:** If large debt remains, schedule a “refactor” phase.  
**Exit:** Ongoing; use PROJECT board to track next milestones (v1.1, etc.).  

# TECH_STACK.md  

## Core Technologies  

- **Tauri v2 (Rust)** – _Desktop shell_: Very lightweight (no bundled Node/Chromium)【32†L168-L174】. System WebView (Edge, WebKit) used for UI. Allows secure IPC. Chosen for small bundle and native feel.  
- **Rust 2021** – _Backend language_: Systems language for performance, safety. All CPU-intensive logic (indexing, image decoding) is in Rust. Strong type system reduces bugs.  
- **React + TypeScript (UI)** – _Frontend_: Robust ecosystem and AI-agent familiarity. TypeScript strict mode for correctness. We use Vite for fast builds. We rejected frameworks with runtime overhead (Angular) or insufficient libs for tooling. (Lap used Vue3【46†L363-L366】, but React has broader tooling.)  
- **SQLite** – _Local DB_: Embedded, ACID, widely used. Good for complex queries (FTS, indexes). We will use `sqlx` or `rusqlite` with migrations. Alternatives (LevelDB, custom formats) lack SQL querying.  
- **LibRaw** – _RAW decoding_: Well-maintained C++ library supporting virtually all RAW formats【21†L25-L33】. We’ll link via FFI (crate like `libraw-rs`).  
- **rexiv2 (Exiv2)** – _Metadata I/O_: Exiv2 is a battle-tested C++ lib for Exif/IPTC/XMP. `rexiv2` crate (GObject wrapper) provides read/write【37†L304-L312】. ExifTool is more comprehensive but slower and external; we prefer native binding for speed.  
- **Image Tools:** The Rust `image` crate (for JPEG/PNG resize), possibly `libvips` for speed if needed. FFmpeg or `ffmpeg-cli` for video previews (sidecar dependency).  
- **Build Tools:** pnpm workspaces (JS), Cargo workspaces (Rust). No use of `npm install` allowed【3†L15-L23】.  We prefer official registry packages or widely-used libs with permissive licenses.  

## UI Stack  

- **Bundle**: Vite build for React. SSR not needed (desktop).  
- **State Management**: React Query or Redux (decided later). Prefer lightweight (React Context + hooks where possible).  
- **Styling**: CSS Modules or Tailwind (optional). Avoid heavy UI frameworks to keep bundle light.  
- **Routing**: React Router if multi-page needed. Probably one-page with modals.  
- **Testing**: Vitest for unit tests, Storybook (optional) for components.  
- **E2E**: Playwright (cross-browser simulation in WebView is tricky, but should work with Electron-like environment).  

## Rust Backend Stack  

- **Async Runtime**: Tokio or async-std for tasks.  
- **DB Crate**: `sqlx` (async, compile-time checks) or `rusqlite` (sync). If heavy concurrency needed, `sqlx` is better.  
- **Image Decoding**: Bindings to LibRaw. Possibly [`rawloader`](https://github.com/ruad/Rawloader) if mature for Rust (currently in alpha).  
- **Metadata**: `rexiv2` for read/write. For pure Rust, [`exif`](https://crates.io/crates/exif) only reads JPEG Exif; lacks XMP. `rexiv2` covers all needed.  
- **CLI Tools**: Avoid if possible, but packaging FFmpeg and any license issues (FFmpeg is LGPL) should be noted. For watermarking, Rust code is fine.  
- **CI Tools**: cargo-nextest (fast parallel testing) can speed up Rust tests.  

## Testing, Lint, CI  

- **JS/TS Lint:** ESLint + Prettier (or [Biome.js](https://biomejs.dev) if stable). TypeScript rules `strict:true`.  
- **Rust Lint:** `clippy`. `rustfmt` for style.  
- **CI/CD:** GitHub Actions. Matrix: Ubuntu-latest, macOS-latest, Windows-latest. Jobs:  
  - **lint** (`pnpm lint`, `cargo fmt/clippy` check).  
  - **unit-tests** (`pnpm test`, `cargo test`).  
  - **build** (`pnpm build`, `cargo build`).  
  - **e2e-tests** (once stable UI).  
  - **release**: packaging (Tauri bundler for .msi/.dmg/AppImage) and signing.  
- **Cache:** pnpm store (via `actions/cache`), Cargo registry and target.  
- **Versioning:** Semantic Versioning; use Commitizen or Conventional Commits (with `cz-customizable`) for changelog automation.  

## Logging & Telemetry  

- **Logging:** Rust `tracing` crate (with file or stdio). JS: minimal `console.log` for dev, suppressed in prod.  
- **Telemetry:** None by default (digiKam’s model【13†L139-L147】). If analytics needed later, must be opt-in and anonymized.  
- **Error Reporting:** Collect unhandled errors to a log (no remote crash reporting out-of-box).  

## Plugin Policy  

- **Allowed:** Native (Rust) sidecar binaries only, signed by project key.  
- **Not Allowed (by default):** Unverified JS code from the internet, dynamic `eval`, or writing to database outside sanctioned APIs.  
- **API Stability:** Define explicit JSON APIs for plugins; use semantic versioning on API schema.  

## Performance Stack Decisions  

- **Database:** Use WAL mode and tune `PRAGMA synchronous = NORMAL` for speed. Index frequently queried columns.  
- **Image Processing:** Consider SIMD (e.g. `rust-simd` crates) for bulk operations.  
- **Concurrency:** Thread pools with limited worker count. For I/O heavy tasks, use async fs with Tokio.  
- **Benchmarks:** Write automated benchmarks (Rust’s `bencher` or similar) to regression-test ingest speed.  

## Approved Dependency Table  

| Dependency    | Purpose                 | Approved Because…                                 | Scope Allowed            | Alternatives Rejected (notes)           |
|---------------|-------------------------|---------------------------------------------------|--------------------------|----------------------------------------|
| Tauri         | Desktop runtime/shell   | Very lightweight, native WebView【32†L168-L174】   | Core framework (UI+IPC)  | Electron (heavy bundle)【32†L168-L174】 |
| React         | UI library              | Large ecosystem, TypeScript support               | UI components/views      | Vue3 (used by Lap【46†L363-L366】, possible) |
| TypeScript    | Language (strict)       | Static typing improves reliability                | Entire frontend          | JavaScript (less safe)                 |
| SQLite (`sqlx`/`rusqlite`) | Database | Widely used embedded DB, supports FTS             | Full local index/store   | MongoDB (server-based), LevelDB (no SQL)|
| LibRaw        | RAW decoding           | Comprehensive RAW support【21†L25-L33】           | CR2, NEF, ARW, etc.      | DCRaw (older, unmaintained), RAW loader|
| rexiv2 (Exiv2)| Metadata I/O           | Handles Exif/IPTC/XMP fully【37†L304-L312】        | JPEG, RAW, PNG, etc.     | ExifTool (external), Rust exif crate (read-only) |
| Vitest        | Unit testing (JS)      | Fast TS tests, JIT, snapshot support              | Unit/integration tests   | Jest (slower), Mocha (less TS support) |
| Playwright    | E2E testing            | Cross-browser automation, good TS support         | End-to-end flows         | Cypress (electron only), Puppeteer (Chromium only) |
| ESLint/Prettier or Biome | Lint/format    | Industry-standard, AST-based rules                | Code style enforcement   | tslint (deprecated)                   |
| Rust `clippy`/`fmt` | Rust lint/format  | Official Rust tools                              | Rust code style          | —                                      |
| Cargo `nextest` | Rust test runner (optional) | Faster parallel testing                        | All Rust tests           | default `cargo test`                  |

(See DEPENDENCY_POLICY.md for rules on adding/removing dependencies.)

# DEPENDENCY_POLICY.md  

- **Enforce `pnpm` only:** All JavaScript/TypeScript dependencies must be added via `pnpm`. *No `npm install` or `yarn` usage.* Use version pinning in `package.json`. For Rust, use `cargo add`.  
- **No broad installs:** Agents must not do `pnpm install package1 package2`. Add one at a time with justification.  
- **Justification Required:** Before adding a new dependency, the agent writes a *Dependency Proposal* (short text in a PR or *DECISIONS.md*) explaining: purpose, alternatives considered, license impact. Then a human or peer review approves.  
- **Check for duplicates:** Search existing codebase and deps. If functionality exists (e.g. formatting or util), use it. No two libraries for same job.  
- **Review checklist for new dependency:**  
  1. Does built-in library suffice? (Rust std, Node APIs)  
  2. Is there an existing approved polyfill or plugin?  
  3. License compatibility (MIT/BSD/Apache often fine; avoid GPL for libraries).  
  4. Dependency size/performance impact (prefer small libs).  
  5. Security: has it been vetted (community usage, CVEs)?  
- **Dev vs Prod:** Classify in `package.json`. Dev-deps (linters, test frameworks) must not sneak into production bundle. Agents ensure to add with `-D` if dev-only.  
- **No speculative packages:** Only add when implementing a feature that requires it. No one-off tries (“npm try it, remove if not used”).  
- **Pinning/Locking:** The repo uses lockfiles (`pnpm-lock.yaml`, `Cargo.lock`). For JS, the lockfile enforces exact versions. For Rust, keep `Cargo.lock` updated.  
- **Updating:** Routine monthly audit of dependencies. Update minor versions regularly, major only with review.  
- **Removing Unused:** If a dependency is no longer used, remove it (update `package.json`/`Cargo.toml`, and run `pnpm prune`).  
- **Approved Dependency Examples:**  
  - Acceptable: “Add `exifreader` to parse JPEG metadata: needed for [Feature X], no similar built-in. (Commit IDs) - Approved.”  
  - Unacceptable: “Add `uuid` to generate IDs, but we already use incremental PKs in SQLite.”  

Agents should follow this strictly and document every change.  

# TESTING.md  

## Testing Strategy (Pyramid)  

1. **Rust Unit Tests:** For all Rust crates (core, image, db). Each function processing data must have a unit test. Use `cargo test` (or `cargo nextest`). Aim for >80% code coverage. *No mocking of core; integration tests preferred.*  
2. **TypeScript Unit Tests:** Use Vitest for all React/TS code. Test component logic, utilities. Use mocking for network or file system. Snapshots for UI changes.  
3. **Integration Tests:** E.g., testing a sequence: ingest file and query DB. Use Jest/Vitest or Rust tests across modules.  
4. **End-to-End Tests:** Use Playwright to automate the actual app: open window, simulate user actions (import folder, rate images, export). These run in CI as sanity checks.  
5. **Performance Regression:** Benchmarks for ingest speed, search latency. Compare with baselines in CI (e.g. using `criterion` crate).  
6. **Cross-Platform Tests:** Ensure tests run on Linux/macOS/Windows. Use CI matrix.  

## Fixtures and Test Data  

- Store sample images and metadata in `/tests/fixtures/`. Include: small JPEG, RAW pairs, corrupt file, video file, diverse EXIF tags. Keep them small for fast CI.  
- **Metadata fixtures:** Example XMP files to test reading/writing.  
- **Simulated Camera:** A folder representing a camera with images.  

## Writing Tests  

- Write tests **before** or along with code (TDD encouraged). Each PR should include tests for new behavior.  
- Use tables to cover edge cases (empty inputs, large sets).  
- No untested code merges.  

## Verification Rules  

- After implementation, run `pnpm test` and `cargo test`. Do not skip failing tests.  
- Agents **must** fix broken tests introduced by changes. If a test is truly invalid, fix the test (with explanation).  
- Coverage: Ensure critical paths (ingest, metadata write) have tests.  

## CI / Commands  

- `pnpm test` runs all JS/TS tests (units).  
- `pnpm test:e2e` triggers Playwright (may require headful mode on CI).  
- `cargo test` runs Rust tests.  
- `cargo nextest run` if used.  
- On every commit, CI runs: `pnpm lint`, `pnpm typecheck`, `pnpm test`, `cargo fmt -- --check`, `cargo clippy`, `cargo test`.  

Agents: *Always* execute these commands after changes. If any fail: analyze and fix or annotate the cause. Completion is not reached until the *full suite* passes.

# REPO_STRUCTURE.md  

**Root Directory:**  
- `apps/`: Contains application entrypoints. For now, `main-app/` (React+Tauri). Future apps (like a CLI) would go here.  
- `crates/`: Rust libraries.  
- `packages/`: Shared JS/TS libraries or widgets (if needed).  
- `docs/`: Markdown docs (`ARCHITECTURE.md`, `DATA_MODEL.md`, etc.).  
- `scripts/`: Utility scripts (e.g. migration runners, packaging scripts).  
- `tests/`: End-to-end tests, fixtures.  
- `.github/`: CI workflows, issue/PR templates.  

**Directory Patterns:**  
- `src-tauri/` and `src/` inside `main-app/` as per Tauri convention.  
- Rust crates each have their own folder with `src/lib.rs` and `tests/`.  
- `docs/architecture/`, `docs/tests/` could exist for detailed topics.  
- Config files at root: `.eslintrc`, `tsconfig.base.json`, `Cargo.toml` (workspace), `pnpm-workspace.yaml`.  

**Naming Conventions:**  
- Rust crates: snake_case, published under workspace with explicit versions.  
- JS packages: kebab-case or scoped (e.g. `@myapp/ui`).  
- Avoid ambigious names; reflect purpose (e.g. `img_preview`, `asset_db`).  
- Tag team or owner in front-matter of Markdown if needed.  

**Ownership & Roles:**  
- Teams should “own” modules: e.g. core team for Rust crates, UI team for React code. But cross-review is required.  
- `OWNERS.md` can list who can approve changes in each area.  

**Monorepo Tips:**  
- Use `pnpm --filter <pkg>` to run commands on a single subproject (example: `pnpm --filter main-app build`).  
- Node and Rust separate: scripts in `package.json` call Rust builds (`cargo build`).  

**Local Overrides:**  
- If needed, per-subproject `AGENTS.md` can override root rules (e.g. special commands for a subfolder). Agents always read the closest AGENTS.md.  

**Component Structure:**  
- Keep code modular: e.g. React components in `apps/main-app/src/components/`, styles in `styles/`, etc.  
- Rust: core crate `lib.rs` with modules (`ingest.rs`, `metadata.rs`, etc.). Tests alongside code or in `tests/`.  

**Examples:**  
- The [Lap repo][46] uses `src-tauri` and `src-vite` under a single app, similar to ours.  
- The [bakeruk monorepo example][41] shows separate packages for frontend/backend and shared config (tsconfig paths).  

# CI_CD.md  

## CI Jobs & Matrix  
- **Lint & Format Check:** Runs on every PR. Executes `pnpm lint` (ESLint/Prettier), `cargo fmt -- --check`, `cargo clippy`.  
- **Build & Test:** Runs on push/PR. Commands: `pnpm build`, `pnpm test`, `pnpm test:e2e` (optional slow job), `cargo build --release`, `cargo test`.  
- **OS Matrix:** Linux (Ubuntu latest), macOS, Windows. Use `actions/setup-node` & `actions/setup-rust`.  
- **Cache:**  
  - pnpm: `actions/cache` on `~/.pnpm-store`.  
  - Rust: cache `~/.cargo/registry` and `~/.cargo/git`.  
- **Secrets:** Store code signing keys under Secrets (for Mac and Win). Not accessible to PRs from forks.  
- **PR Gates:** Merge only if all workflows pass. Require code review approvals.  

## Artifacts & Release  
- **Artifacts:** On `push` to `main`, build artifacts (Linux AppImage, Windows MSI, macOS DMG) are uploaded to GitHub Releases.  
- **Release Workflow:** Tag with semantic version (e.g. `v1.0.0`), triggers release job: builds all packages, signs them, creates GitHub Release with changelog (from commits or `CHANGELOG.md`).  
- **Signing:**  
  - Windows: Sign MSI via SignTool (requires certificate).  
  - macOS: Code-sign & notarize DMG (Apple Developer ID needed).  
  - Linux: GPG-sign package if needed.  
- **Documentation Deployment:** Optionally deploy docs (README + selected MD) to gh-pages or docs site on each release.  

## CI Failure Policy  
- **First Failure:** Immediately alert via CI logs. Engineers should not ignore.  
- **Broken Tests:** Fix first, do not disable. If unavoidable, document in DEBT_REGISTER and proceed only if urgent.  
- **Stale Artifacts:** Clear CI caches occasionally to avoid inconsistent builds.  

# SECURITY.md  

- **Local-Only by Design:** By default, the app never sends user data over the network. All processing (and any optional AI/plugins) is local【13†L139-L147】【46†L281-L289】. No telemetry, no data sharing unless user explicitly configures an uploader plugin (with clear consent).  
- **Dependency Hygiene:** All dependencies must be vetted (see DEPENDENCY_POLICY). We run `cargo audit` and `npm audit` in CI to catch known vulnerabilities.  
- **Tauri Sandboxing:** The UI runs in a WebView sandbox with no direct OS access. Only explicit IPC commands (declared in `tauri.conf.json` with capabilities) are available【40†L208-L216】. This enforces a *trust boundary* between JS and Rust. Do not expose general `eval` or open-ended file APIs.  
- **Content Security Policy:** A strict CSP is applied to the WebView. No remote scripts; only local files and approved APIs.  
- **Secrets Management:** Any secrets (like API keys for future integrations) must be stored in encrypted config or OS keychain. Do not hard-code them.  
- **Plugin Security:** If we allow plugins, run them as separate processes with limited privileges. Use Tauri Sidecar pattern to launch plugins in their own lifecycle【34†L292-L302】. No plugin code is downloaded automatically.  
- **Error Disclosure:** Stack traces or errors should not contain sensitive paths or user data. In production builds, strip debug info.  
- **Upgrade Mechanism:** Tauri’s built-in updater can handle signed update binaries. We sign releases to prevent tampering.  

# DATA_MODEL.md  

## Asset Model  
- **Asset:** Each photo/video has a unique ID. Fields: `filepath`, `filename`, `folder`, `filetype` (RAW/JPEG/PNG/etc), `filesize`, `created_at` (filesystem), `captured_at` (EXIF datetime), `camera_model`, `orientation`.  
- **Variants:** For RAW+JPEG pairs: one asset is the RAW master, the JPEG is a “derived” variant. We store a link (e.g. `master_id` foreign key). If multiple variants (TIFF, resized exports), also linked.  
- **Sidecar:** If an external XMP file exists (same name with .xmp), store its path. On ingest, parse sidecar and merge into asset metadata. Writing metadata updates sidecar.  
- **Metadata Fields:** Stored both in DB and optionally in files. Key fields: Title/Caption, Keywords (tags), Author, Copyright, GPS coords, Rating, Color Label, Custom IPTC fields.  
- **Metadata Precedence:** 
  - Import: file XMP > DB > default.  
  - Save: DB writes to XMP sidecar (preferred) or directly into image (if safe format).  
  - If no sidecar exists for RAW, create one. JPEG’s internal metadata is updated directly.  
- **Cache Model:**  
  - **Thumbnails/Previews:** Stored on disk (not in DB). Entries in DB table `thumbnails(asset_id, size, path)`. If missing, generate.  
  - **In-memory:** Minimal (only current page or selection loaded).  
- **Search Index Model:**  
  - Full-text search on `title + caption + tags` via SQLite FTS virtual table.  
  - Tag filters: normalized tag list in separate table (`asset_tags`).  
  - Date filters: index on `captured_at`.  
  - GPS: store latitude, longitude columns; use R-Tree (SQLite) for spatial queries if needed.  
- **Audit Log Model:**  
  - Table `audit(id, timestamp, user (system), action, details)`. Log actions like import, delete, metadata edit.  
  - Not user-editable; append-only.  

# INGEST_PIPELINE.md  

**Overview:** Handles new images entering the library.  

1. **Trigger:** Can be manual (“Import from Camera”) or automatic (folder watch).  
2. **Transfer:** Copy/Move files to library folder. Avoid name collisions (auto-rename or prompt).  
3. **Metadata Read:** For each file, read EXIF/metadata (via rexiv2): timestamp, camera, GPS, ratings, tags (for embedded), etc. Also read any XMP sidecar.  
4. **DB Insert:** Create `asset` and variants records with all read data. Store canonical path.  
5. **Thumbnail Queue:** Enqueue generation of thumbnail and preview.  
6. **Error Handling:** If a file fails (corrupt or unsupported), log and skip.  
7. **Duplicate Checking:** Compare checksums or key metadata; if duplicate found, mark status. (UI can prompt to skip or link to existing.)  
8. **Dependencies:** `crates/image`, `crates/db`, `crates/metadata`.  

# METADATA_SYSTEM.md  

**Design:** Central service to unify file metadata and user edits.  

- **Read Pipeline:** On access or ingest, read all available metadata sources:  
  - **EXIF:** Embedded in file.  
  - **IPTC/XMP:** Embedded or sidecar.  
  - Combine them, with a clear hierarchy (file sidecar overrides embedded EXIF for overlapping fields).  
- **Edit API:** When user edits metadata:  
  - Update values in database.  
  - Write back to file’s XMP or EXIF using rexiv2 commands.  
  - Maintain both DB and file in sync.  
- **Batch Edits:** Support multi-select apply (transactionally update multiple).  
- **Special Cases:**  
  - **Time adjustment:** If user edits capture time, update EXIF DateTimeOriginal and GPS date if any.  
  - **Canon CR3 issue:** Some formats may require exiftool fallback (documented in DEBT_REGISTER if needed).  
- **Sidecar Handling:** Use XMP sidecars for RAW (since many RAW containers are read-only). For JPEG, prefer embedding.  
- **Validation:** GPS coords validated within valid ranges. Filename editing avoids reserved characters.  

# PERFORMANCE.md  

- **Indexing:** Use concurrent file scanning but throttle (e.g. 4 threads max) to avoid I/O contention【27†L123-L132】.  
- **Database:** Insert in bulk transactions for initial import. For real-time ingest, commit per import session, not per file.  
- **Thumbnails:** Generate in background threads. Use a small fixed-size thread pool. Write results asynchronously (non-blocking the UI).  
- **Debounce UI:** In browsing, debounce heavy queries (filtering) until user pauses typing. Cache recent queries.  
- **Memory:** Release images after use. Use streaming API for large files.  
- **SQLite Settings:** `PRAGMA journal_mode=WAL`, `synchronous=OFF` on bulk ops, `cache_size=10000`.  
- **Compression:** Consider storing thumbnails as JPEG (high quality) to reduce disk usage.  

# UX_WORKFLOWS.md  

1. **Card/Folder Ingest:** User clicks “Import”. Select source (camera or directory). Shows preview of files. Options: rename template (e.g. `{YYYY}-{MM}-{DD}_{Counter}`【19†L105-L110】), create new album, skip duplicates. Press “Import” and watch progress.  
2. **Contact Sheet Browsing:** The main view shows thumbnails of all images. User scrolls with instant loads (virtual scroll). Sorting and filtering controls at top (date, rating, tag filters).  
3. **Selection/Culling:** Click or marquee-select images. Assign star ratings or color labels via toolbar. Reject/unflag through context menu. Undo/Redo for selection changes.  
4. **Metadata Editing:** With one or more images selected, open “Edit Metadata” panel. Fields for caption, keywords (tags), location (map), date/time, etc. Changes apply to all selected (with warnings if heterogeneous). Save writes to DB and file.  
5. **Batch Rename:** User invokes rename dialog, chooses naming mask and options. Preview sample before apply. On confirm, filenames change in FS and DB.  
6. **Search:** Quick search bar: keywords match captions/tags. Advanced filters: date range picker, rating dropdown, tag multiselect. Results update live.  
7. **Export:** Select images, open Export dialog. Choose format (JPEG/PNG), destination folder, filename mask. Optionally add watermark text or image. Hit Export; a progress bar shows export status.  
8. **GPS/Map:** If images have geotags, show “Map” view plotting points (using Leaflet or similar). Allow dragging points to correct location (updates metadata).  
9. **External Edit:** “Edit in…” option opens the file in user’s chosen external editor (Photoshop, etc.). Instruct to save back, then app rescans (or watches folder for changes).  
10. **Help/About:** Offline docs links and credits (digiKam-style “We Respect your Work”【13†L139-L147】 warning no telemetry).  

Each workflow is documented in code comments and corresponding UI tests.

# CONTRIBUTING.md  

We welcome contributions! To keep the project high-quality and agent-friendly:

- **Developer Setup:** See `README.md` for setup instructions (Node 18+, pnpm, Rust toolchain). Use `pnpm install` and `cargo install cargo-nextest`.  
- **Code Style:** Follow existing style: TypeScript strict mode, ESLint/Prettier rules (see `.eslintrc`). Rust: `rustfmt` and Clippy.  
- **Commits:** Use [Conventional Commits](https://www.conventionalcommits.org/) (e.g. `feat:`, `fix:`, `docs:`, `test:`).  
- **Branches:** Each feature or fix gets its own branch from `main`. Name branches with short descriptive names (`feature-rename-masks`).  
- **Pull Requests:**  
  - Title format: `[<area>] Short description` (e.g. `[UI] Implement dark mode`).  
  - Include a detailed description: what changed, why, how tested. Link issues/decisions if relevant.  
  - Make sure all CI checks pass before marking ready.  
- **Reviews:** At least one approving review is required. Discuss any design changes in issues/decisions before coding large refactors.  
- **Issues:** Report bugs or feature requests via GitHub Issues. Follow the bug template and include logs or screenshots. For code changes, link the issue to PR.  
- **Local-First Ethos:** Do not introduce cloud-dependent features without discussing privacy implications.  
- **Security:** Do not commit secrets or credentials. Use environment variables or Tauri’s configuration.  
- **Documentation:** Update docs when adding features. Feel free to improve any markdown content.  
- **Testing:** Add tests for any new functionality. Pull requests should not decrease test coverage.  
- **Community:** Be respectful. Read `CODE_OF_CONDUCT.md`.

# DECISIONS.md  

This file tracks significant architectural and design decisions (ADR style). Each entry includes context, options considered, decision, and rationale. For example:

- **ADR 001: Desktop Framework** – Chose Tauri over Electron/Vue/Electron because Tauri yields smaller, more secure binaries【32†L168-L174】 and Lap’s success【46†L361-L369】.  
- **ADR 002: UI Library** – Chose React (TS) for ecosystem support; Svelte/Vue were alternatives but had smaller community or performance tradeoffs.  
- **ADR 003: Metadata Library** – Chose rexiv2 (Exiv2) for full metadata coverage【37†L304-L312】. Considered ExifTool (overhead) and Rust exif (read-only).  
- **ADR 004: Database** – Chose SQLite (with FTS) per local-first and immutability needs; considered PostgreSQL (overkill) and custom JSON (inefficient search).  

Agents should append to DECISIONS.md for any key choice or when forking paths (with date, author).

# DEBT_REGISTER.md  

A running list of known issues or technical debt:

| Date       | Issue                                              | Severity | Owner      | Notes / Proposed Fix |
|------------|----------------------------------------------------|----------|------------|----------------------|
| 2026-03-07 | Video thumbnail support missing for MP4             | Medium   | @devteam   | Add FFmpeg extraction when present.             |
| 2026-03-07 | Some NEF files fail LibRaw decode (see issue #45)   | High     | @devcore   | Use embedded IJG or fallback to dcraw.           |
| ...        | ...                                                | ...      | ...        | ...                  |

Before closing a PR that introduces known-but-unfixable issues, list them here with a tracking ID.  

# ROADMAP.md  

We plan feature releases as follows:

- **v0.x (Alpha):** Core workflows (import, browse, metadata edit) complete. Focus on functionality and performance.  
- **v1.0 (Beta→Stable):** All target features (ingest, culling, metadata, search, export). Polish UX.  
- **v1.x:** Plugins, advanced search (face search, etc.), major enhancements.  
- Post-1.0: Maintenance, community contributions, performance tweaks.

Progress milestones are tracked via GitHub Projects and Issues. Checklists for each milestone (as in PLANNING.md) will be managed in the issue tracker.
