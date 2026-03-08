# CONTRIBUTING.md

This document defines how contributors should work in the Photo Workroom repository.

Current repository status on March 8, 2026:

- the repository now contains a real workspace scaffold and a desktop shell baseline
- `.github/workflows/bootstrap-validation.yml` now runs baseline verification in an OS matrix

## Before contributing

Read these files first:

- `AGENTS.md`
- `ARCHITECTURE.md`
- `TECH_STACK.md`
- `TESTING.md`
- `DEPENDENCY_POLICY.md`
- `PLANNING.md`

If your change touches a subsystem, also read the relevant subsystem doc such as:

- `DATA_MODEL.md`
- `INGEST_PIPELINE.md`
- `METADATA_SYSTEM.md`
- `PERFORMANCE.md`
- `UX_WORKFLOWS.md`
- `CI_CD.md`
- `SECURITY.md`

## Contribution expectations

Required workflow:

1. understand the task
2. inspect the current code or docs
3. identify affected files and interfaces
4. update the plan if the work is substantial
5. implement the smallest correct change
6. run the required verification
7. fix failures instead of hiding them
8. update docs affected by the change
9. summarize results truthfully

## Documentation-only contributions

If the repository still contains documentation only, contributors should:

- keep `PLANNING.md` status markers truthful
- update related docs together so guidance stays consistent
- avoid documenting features as implemented when they are still planned

## Code contribution workflow

Once the workspace exists:

- branch from the active integration branch or `main` according to the team policy
- keep changes small and reviewable
- update tests with the behavior change
- update docs in the same change set when behavior or process changes

## Local setup

Supported toolchain baseline:

- Node `20.20.0` or newer
- npm `10.9.4` or newer
- Rust `1.93.1` with `rustfmt` and `clippy`

Platform prerequisites for Tauri:

- macOS: Xcode Command Line Tools and the system WebKit runtime
- Linux: WebKitGTK, an appindicator implementation, `librsvg`, and `patchelf`
- Windows: Visual Studio C++ build tools and Microsoft WebView2

Suggested bootstrap flow:

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

To run the desktop shell locally:

```bash
npm run tauri:dev
```

To run the deterministic desktop launch smoke check:

```bash
npm run test:tauri-launch
```

Runtime note:

- `npm run dev` is renderer-only and does not provide Tauri desktop APIs
- use `npm run tauri:dev` when validating typed IPC commands or window-management behavior
- use `npm run test:tauri-launch` to assert that the desktop shell process starts and emits the startup marker

Environment note:

- no repository-level `.env` file is required for the current Phase 0 through Phase 2 baseline

## Editor recommendations

Recommended baseline:

- VS Code or another editor with TypeScript, Rust, TOML, and JSON schema support
- Rust Analyzer
- ESLint extension
- Even Better TOML or equivalent TOML tooling

## Troubleshooting

Common setup issues:

- if `npm ci` fails, confirm you are using the supported Node and npm versions
- if local verification is inconsistent, run `npm ci` and then `npm run verify:local` from the repository root to enforce a deterministic check order
- if Tauri fails to build on Linux, verify the WebKitGTK and appindicator packages are installed
- if Tauri fails to build on macOS, install Xcode Command Line Tools with `xcode-select --install`
- if Tauri fails to launch on Windows, install or repair Microsoft WebView2
- if Rust checks fail after a toolchain update, run `rustup show` and confirm the workspace toolchain matches `rust-toolchain.toml`

## Branch and commit conventions

Recommended conventions:

- use short descriptive branch names
- use conventional commit prefixes where practical such as `feat:`, `fix:`, `docs:`, `test:`, or `chore:`
- keep commit messages specific to the change

## Pull request expectations

Each PR should explain:

- what changed
- why it changed
- how it was verified
- which risks remain
- which docs were updated

PRs should not:

- hide failing checks
- leave major architectural drift undocumented
- merge broad unrelated cleanup with focused feature work unless there is a documented reason

## Dependency changes

If a dependency changes:

- follow `DEPENDENCY_POLICY.md`
- document the rationale
- update `TECH_STACK.md` or `DECISIONS.md` when the change is architectural

## Verification

Contributors must run the relevant checks from `TESTING.md`.

If verification fails:

- capture the failing command
- determine whether the failure is introduced or pre-existing
- fix introduced failures before claiming completion

## Security and privacy

Rules:

- do not commit secrets
- do not introduce hidden network dependencies
- keep local-first guarantees intact unless there is an explicit reviewed product decision

## Reviews

Reviewers should look for:

- correctness
- regression risk
- missing tests
- documentation drift
- architecture drift

## Code of conduct

Treat collaborators respectfully and directly. If a separate `CODE_OF_CONDUCT.md` is added later, reference it here.
