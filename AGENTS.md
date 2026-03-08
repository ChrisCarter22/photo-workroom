# AGENTS.md

This repository is designed for Codex-style coding agents and human contributors working with agent assistance.

## Mandatory reading before substantial changes

Before making any substantial change, agents must read:

- `ARCHITECTURE.md`
- `TECH_STACK.md`
- `TESTING.md`
- `DEPENDENCY_POLICY.md`
- `PLANNING.md`

For tasks that touch a subsystem, agents must also read the relevant subsystem docs such as:

- `DATA_MODEL.md`
- `INGEST_PIPELINE.md`
- `METADATA_SYSTEM.md`
- `PERFORMANCE.md`
- `UX_WORKFLOWS.md`
- `CI_CD.md`
- `SECURITY.md`

Do not begin implementation until the relevant documents have been reviewed.

## Operating contract

Agents must work in a strict, truthful, verification-first manner.

Planning-order rule:

- `PLANNING.md` is the execution source of truth for phase sequence.
- Agents must execute phases in order and must not vary or skip ahead without an explicit documented reason.
- If a task requires out-of-order work, agents must:
  - document the reason in `PLANNING.md`
  - record any architectural impact in `DECISIONS.md` when applicable
  - keep the change narrowly scoped to unblock the ordered plan

Required behavior:

1. Understand the task.
2. Inspect the existing code and tests.
3. Identify affected modules and interfaces.
4. Write or update a plan.
5. Implement the smallest change that solves the task.
6. Run required lint, typecheck, test, and build commands.
7. Diagnose and fix failures.
8. Update docs affected by the change.
9. Summarize changes, verification, and residual risks truthfully.

Agents must prefer:

- small, reversible changes
- modifying existing files over creating new abstractions
- explicit behavior over hidden magic
- built-in platform capabilities over new dependencies
- approved existing dependencies over adding packages
- narrow, well-tested changes over speculative rewrites

## Git permissions

Agents have permission to run Git commit and push operations to GitHub for task-related work.

Rules:

- commit and push only task-relevant changes
- run required verification before pushing
- do not hide failing checks when committing or pushing
- follow repository branch and PR policy documented in `CONTRIBUTING.md`

## Hard rules

Agents must not:

- skip tests
- mute failures
- hide errors
- claim completion when verification is failing
- use `pnpm` or Yarn
- add broad sets of dependencies speculatively
- rewrite unrelated code without a documented reason
- introduce silent architecture drift
- replace real fixes with TODOs unless documented in `DEBT_REGISTER.md`
- disable lint, typecheck, or test gates to make work appear done
- narrow test scope without justification
- comment out assertions to pass CI
- swallow runtime errors silently
- claim “manual verification only” when automated verification is required

## Task execution flow

### 1. Understand task

- Restate the task in operational terms.
- Identify the user-visible or system-visible outcome.
- Note constraints from architecture, testing, dependency, and performance docs.

### 2. Inspect existing code

- Read the relevant modules before editing.
- Search for existing patterns and approved utilities.
- Identify existing tests, fixtures, and scripts relevant to the task.
- Determine whether the task touches UI, Rust core, database, filesystem, metadata, ingest, export, or CI.

### 3. Identify affected modules

Typical affected areas may include:

- `apps/main-app/src/` for UI logic
- `apps/main-app/src-tauri/` for desktop runtime integration
- `crates/core/` for domain logic
- `crates/db/` for persistence and migrations
- `crates/image/` for previews and transforms
- `crates/metadata/` for metadata read/write
- `tests/` for integration and e2e flows
- root docs for architectural or workflow changes

### 4. Write or update a plan

Before substantial edits, agents must write a brief plan in the task summary, issue, PR description, or local notes.

The plan must include:

- modules to change
- expected behavior
- verification steps
- risks and rollback approach if relevant

### 5. Implement minimally

- Change the smallest surface that can solve the task correctly.
- Reuse existing abstractions where they fit.
- Prefer incremental extension over introducing a new framework or layer.
- Keep changes readable and reviewable.

### 6. Run verification

Every implementation must include verification.

At minimum, run the relevant subset of:

```bash
npm ci
npm run lint
npm run typecheck
npm run test
npm run test:integration
npm run test:e2e
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
npm run build
```

### 7. Fix failures

If verification fails, the next required action is root-cause analysis and remediation.

Do not stop at the first error and report “done except tests”.

Required sequence when verification fails:

1. Capture the exact failing command.
2. Summarize the error truthfully.
3. Determine whether the failure is pre-existing or introduced.
4. Fix the failure if it was introduced by the change.
5. If pre-existing, isolate blast radius and verify that the task did not worsen it.
6. Re-run the relevant checks after remediation.

### 8. Update docs

Code changes that alter architecture, workflows, commands, dependencies, data models, or test expectations must update the relevant markdown files.

This is mandatory.

### 9. Summarize changes and residual risks

The task summary must include:

- files changed
- behavior changed
- commands run
- pass/fail status of each command
- any residual risks
- any documented debt created

## Definition of Done

A task is done only when all of the following are true:

- the requested behavior is implemented
- the code compiles
- lint passes
- typecheck passes
- required tests pass
- required builds pass
- docs are updated where relevant
- no hidden bypasses were introduced
- any exceptions are explicitly documented in `DEBT_REGISTER.md` or `DECISIONS.md`

A task is not complete if any required verification step is failing.
