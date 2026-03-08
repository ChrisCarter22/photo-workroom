# Photo Workroom

Photo Workroom is a local-first desktop photo workflow application under active bootstrap.

Current milestone:

- npm and Cargo workspaces are scaffolded
- `apps/main-app` boots a React + TypeScript + Vite renderer with a Tauri v2 backend
- the renderer exercises a typed Rust health-check command through Tauri IPC
- initial Rust crate boundaries exist for `core`, `db`, `image`, `ingest`, `metadata`, and `task_runtime`
- CI workflows, database persistence, filesystem scanning, ingest execution, and metadata write-back are still planned

## Quick start

Prerequisites:

- Node `20.20.0` or newer
- npm `10.9.4` or newer
- Rust `1.93.1` with `rustfmt` and `clippy`
- platform Tauri prerequisites described in `CONTRIBUTING.md`

Install and verify:

```bash
npm ci
npm run verify:local
```

Manual equivalent (same checks in explicit order):

```bash
npm run lint
npm run typecheck
npm run test
npm run test:integration
npm run test:e2e
npm run build

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
```

Run the desktop shell locally:

```bash
npm run tauri:dev
```

Run the deterministic launch smoke check:

```bash
npm run test:tauri-launch
```

Troubleshooting notes:

- `npm run dev` starts only the renderer and cannot exercise Tauri-only behavior such as backend IPC and separate-window actions
- use `npm run tauri:dev` when verifying shell behavior that depends on desktop runtime features
- no repository-level `.env` file is required for the current Phase 0 through Phase 2 baseline

## Repository map

- `apps/main-app/`: desktop shell renderer and Tauri runtime
- `crates/`: initial Rust subsystem boundaries
- `tests/fixtures/metadata/`: promoted metadata reference fixtures
- `tests/validation/BOOTSTRAP_VALIDATION.md`: platform bootstrap and launch verification evidence
- `examples/`: manually reviewed reference files kept alongside the formal fixture copies
- `PLANNING.md`: phase tracker and current implementation status
- `CONTRIBUTING.md`: setup requirements, verification rules, and contributor workflow
