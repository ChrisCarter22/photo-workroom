# DEBT_REGISTER.md

This file tracks temporary exceptions, known gaps, and technical debt.

Use this register when:

- a known issue cannot be fixed in the current change
- a temporary exception to policy is required
- a design gap is accepted for a limited time with an owner and review date

Do not use this register for:

- normal planned work that simply has not started yet
- vague reminders with no owner or follow-up path

## Status meanings

- `Open`: unresolved and still active
- `Mitigated`: partial mitigation exists, but final resolution is pending
- `Closed`: resolved and ready to archive out of the active list

## Active debt

| ID | Date | Area | Title | Severity | Owner | Rationale | Exit or rollback criteria | Target review date | Status |
|---|---|---|---|---|---|---|---|---|---|
| DEBT-001 | 2026-03-08 | Preview | RAW preview fallback policy is not finalized for all formats | Medium | Unassigned | Some formats may require embedded-preview fallback before a full decode path is stable | Finalize per-format preview policy and add regression tests | 2026-06-01 | Open |
| DEBT-002 | 2026-03-08 | Metadata | Primary metadata library choice is documented, but not yet validated against a real fixture corpus | Medium | Unassigned | The planning baseline prefers an Exiv2-based path, but no implementation or format coverage validation exists yet | Validate the chosen library against representative JPEG, RAW, and sidecar fixtures or record an updated ADR | 2026-06-15 | Open |
| DEBT-003 | 2026-03-08 | Process | Documentation baseline currently leads implementation by a wide margin | Low | Unassigned | Detailed docs exist before code scaffolding, which creates drift risk if implementation diverges silently | Reconcile docs against real workspace structure during Phase 0 through Phase 3 and again before beta | 2026-05-15 | Open |
| DEBT-004 | 2026-03-08 | Product Scope | Standard Photo Mechanic parity is now the benchmark, but delivery integrations and shortcut customization remain planning-only | Medium | Unassigned | The target feature map now includes several Photo Mechanic-style workflows that have not been technically validated in this repository yet | Revisit the phase breakdown once the app scaffold exists and split any overlarge parity areas into implementation epics with fixture-backed verification | 2026-05-15 | Open |
