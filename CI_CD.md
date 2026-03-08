# CI_CD.md

This document defines the target CI and release process for Photo Workroom.

Current repository status on March 8, 2026:

- `.github/` now exists with issue and PR templates
- no workflows or release automation have been implemented
- the content below defines the baseline that Phase 16 should deliver

## Goals

CI and CD exist to:

- prevent unverifiable merges
- enforce lint, typecheck, test, and build standards
- validate cross-platform behavior
- produce trustworthy artifacts
- support signed release workflows
- surface failures early and truthfully

## CI jobs and matrix

Required baseline jobs once the workspace exists:

| Job | Purpose | Commands |
|---|---|---|
| `lint` | enforce style and static checks | `npm run lint`, `npm run typecheck`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| `test` | run unit and integration coverage | `npm run test`, `npm run test:integration`, `cargo test --workspace` |
| `build` | validate production outputs | `npm run build`, `cargo build --workspace` |
| `e2e` | validate core workflows | `npm run test:e2e` |
| `docs` | optional documentation validation if adopted | markdown lint, link checks, or other doc tooling |
| `release-dry-run` | validate packaging and release metadata | release workflow in non-publishing mode |

Required matrix coverage:

- Ubuntu latest
- macOS latest
- Windows latest

## Workflow design rules

Rules:

- fast feedback jobs should run first
- slower E2E and packaging jobs can run after core lint and unit suites
- CI must fail loudly and specifically
- no workflow may hide a failing command behind a passing wrapper

## Cache strategy

Preferred caches:

- npm cache
- cargo registry
- cargo git cache
- carefully scoped cargo target cache when it provides stable value

Rules:

- caches are for speed, not correctness
- if a cache creates flaky behavior, fix or remove it
- document cache keys and invalidation assumptions in workflow files

## Artifacts and release

Target release outputs:

- Windows installer or package
- macOS package or DMG
- Linux package formats appropriate to the final distribution plan

Release workflow responsibilities:

- build artifacts for all supported platforms
- sign artifacts where required
- create a release draft with notes
- attach checksums or integrity metadata
- record the exact app version and migration implications

## Secrets and signing

Rules:

- signing keys live in CI secret storage, never in the repository
- release secrets must not be exposed to untrusted forks
- signing steps should be isolated to protected workflows or environments
- document any manual notarization or review steps that remain

## PR gates and branch protection

Recommended protections:

- require passing CI checks before merge
- require code review approval
- restrict direct pushes to protected branches
- require release workflow validation before tagging stable releases

## Failure policy

When CI fails:

- treat the failure as actionable work
- capture the exact failing job and command
- determine whether the failure is new or pre-existing
- fix the underlying problem instead of weakening the check
- if an exception is unavoidable, document it in `DEBT_REGISTER.md`

Not allowed:

- disabling a failing job to make the branch look green
- narrowing the matrix without a documented reason
- silently skipping expensive suites that are meant to be required

## Release process expectations

Before a stable release:

- all required CI jobs are green
- security and dependency audits are reviewed
- release notes are written
- packaging artifacts are smoke-tested
- platform-specific signing and notarization steps are complete

## Documentation requirements

When CI or release behavior changes:

- update this file
- update `PLANNING.md` if the phase work changes
- update `CONTRIBUTING.md` if contributor workflow changes
