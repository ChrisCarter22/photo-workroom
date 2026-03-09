# TESTING.md

Testing is mandatory. This document defines the quality gates for both future implementation work and documentation changes.

Current repository status on March 9, 2026:

- the repository now contains application code under `apps/main-app/` and Rust workspace crates under `crates/`
- app-local Vitest coverage exists for unit, integration, and shell smoke flows
- `crates/db` now includes Rust unit coverage for fresh migration bootstrap, upgrade migration application, and typed CRUD behavior
- metadata reference files have been promoted into `tests/fixtures/metadata/`
- a cross-platform bootstrap validation workflow now exists at `.github/workflows/bootstrap-validation.yml`
- the repository now includes `examples/XMP Side Car.XMP` as a seed XMP sidecar reference file
- the repository now also includes `examples/IPTC_Fields.XMP` as a broad XMP sidecar reference for IPTC field-map coverage
- the repository now also includes `examples/IPTC-PhotometadataRef-Std2025.1.jpg` as a broad IPTC JPEG reference fixture
- `npm run test:e2e` currently exercises a Vitest-based renderer smoke flow until Playwright-based desktop E2E automation is implemented
- `npm run test:tauri-launch` now provides a deterministic desktop-process launch smoke check for the shell bootstrap milestone

## Testing policy

Required rules:

- every behavior change must include verification
- agents and contributors must run the relevant automated checks before claiming completion
- failing verification must be treated as work to diagnose, not as a footnote
- no feature is done until the required checks pass or a documented exception exists in `DEBT_REGISTER.md`

## Testing strategy

### 1. Rust unit tests

Use for:

- pure domain logic
- metadata normalization helpers
- rename template expansion
- DB repositories and validation helpers

Rules:

- test success cases and error cases
- prefer deterministic fixtures
- avoid unnecessary mocking of core domain logic

### 2. TypeScript unit tests

Use for:

- UI component logic
- query builders
- state transitions
- formatting utilities

Rules:

- keep component tests focused on behavior, not implementation details
- mock privileged backend calls at the boundary
- use snapshots sparingly and only when they provide real value

### 3. Integration tests

Use for:

- ingest plus DB synchronization
- metadata edit plus write-back
- rename plus path persistence
- export planning plus output manifest generation

Rules:

- cross module boundaries deliberately
- validate real data flow, not just isolated functions
- use temp directories or explicit fixtures

### 4. End-to-end tests

Use for:

- core user workflows in the running app
- import, browse, rate, tag, search, rename, and export flows

Rules:

- test representative happy paths first
- expand to critical regressions and edge cases after baseline stability
- keep E2E focused on workflows that matter to shipping quality

Bootstrap note:

- until Playwright and Tauri-specific automation land, `npm run test:e2e` is backed by a renderer-shell smoke suite that validates the current bootstrap workflow honestly

### 5. Performance regression checks

Use for:

- ingest throughput
- scan latency
- contact-sheet scroll responsiveness
- preview generation latency
- search response times

Rules:

- benchmark with a stable fixture corpus
- compare against recorded baselines
- do not accept large regressions without a documented reason

### 6. Cross-platform verification

Use for:

- path handling
- packaging differences
- filesystem permission differences
- native dialog behavior

Rules:

- Linux, macOS, and Windows must all be exercised in CI or release validation
- platform-specific failures are release-blocking for the affected platform

### 7. Desktop launch smoke

Use for:

- Phase 2 shell verification that the Tauri desktop process starts successfully

Rules:

- `npm run test:tauri-launch` must detect the backend startup marker and terminate cleanly
- Linux CI runs should execute launch smoke under `xvfb-run`

## Fixtures and test data

The target fixture layout is:

- `tests/fixtures/images/` for JPEG, PNG, and small representative media files
- `tests/fixtures/raw/` for RAW samples and paired JPEG files
- `tests/fixtures/metadata/` for sidecars and metadata edge cases
- `tests/fixtures/corrupt/` for broken or truncated files
- `tests/fixtures/export/` for expected export outputs

Fixture rules:

- keep fixtures small enough for CI
- include copyright-safe files only
- document fixture provenance when it matters
- do not use production or personal photo libraries in tests

Current seed examples:

- until `tests/fixtures/` exists, use `examples/XMP Side Car.XMP` as the baseline sidecar reference for metadata parsing, precedence, and write-back planning
- use `examples/IPTC_Fields.XMP` together with `IPTC_FIELD_MAP.md` as the broad XMP-sidecar reference for field-by-field IPTC coverage checks
- use `examples/IPTC-PhotometadataRef-Std2025.1.jpg` as the broad JPEG metadata reference for IPTC Core, IPTC Extension, PLUS, accessibility, rights, and AI-related field coverage
- if `examples/PhotoMechanic.XMP` is intended to be part of the fixture set, it must exist on disk before it is referenced as a real test input

Metadata-specific fixture rules:

- `examples/IPTC_Fields.XMP` must remain valid XML and parseable as XMP
- the canonical expected IPTC property inventory is defined in `IPTC_FIELD_MAP.md`

## Writing tests

Required practice:

- write tests before or alongside code
- add tests for new behavior and regressions
- cover edge cases such as empty inputs, duplicates, missing sidecars, and partial failures
- keep tests readable and localized to the behavior they protect

Not allowed:

- merging new behavior with no tests when automated coverage is expected
- deleting assertions to make a failing suite pass
- muting flaky failures without root-cause analysis

## Verification rules

After implementation work:

- run the relevant lint, typecheck, test, and build commands
- capture the exact failing command if anything breaks
- determine whether the failure is introduced or pre-existing
- fix introduced failures before reporting completion
- if a failure is pre-existing, isolate the blast radius and prove the change did not worsen it

For documentation-only changes:

- run any available markdown or link validation tooling if the repository has it
- if no documentation tooling exists, perform a careful manual review and state that the repo lacks automated doc checks today
- do not claim code test coverage was run when the repository does not yet contain code to test

## Required commands

These commands are the default verification set once the workspace is scaffolded:

```bash
npm ci
npm run lint
npm run typecheck
npm run test
npm run test:integration
npm run test:e2e
npm run test:tauri-launch
npm run build

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
```

Optional accelerators:

```bash
cargo nextest run
npm --workspace <workspace> run test
npm run verify:local
```

## CI expectations

Once CI exists, it must enforce:

- lint and format checks
- type checking
- unit and integration tests
- end-to-end tests on the appropriate cadence
- release build validation

CI must not:

- silently skip failing suites
- narrow scope without explanation
- pass while hiding broken steps

## Failure handling contract

When verification fails:

1. capture the exact command
2. summarize the real error
3. determine whether the failure is new or pre-existing
4. fix the failure if it was introduced by the current change
5. re-run the relevant checks
6. document any remaining exceptions in `DEBT_REGISTER.md`

## Definition of done for testing

Work is not complete until:

- the requested behavior exists
- the required automated checks pass, or a documented exception exists
- any new risks are called out explicitly
- the final summary reports pass and fail status truthfully
