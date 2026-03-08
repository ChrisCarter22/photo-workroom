# Documentation Index

This repository is currently a documentation-first planning workspace for Photo Workroom.

As of March 8, 2026:

- the repo contains reference documents and research outputs
- the implementation scaffold does not exist yet
- `PLANNING.md` is the source of truth for what is completed versus merely planned

## Recommended reading order

1. `AGENTS.md`
2. `README_INDEX.md`
3. `ARCHITECTURE.md`
4. `PLANNING.md`
5. `TECH_STACK.md`
6. `TESTING.md`
7. `DEPENDENCY_POLICY.md`
8. subsystem and process docs relevant to your task

## Core planning and policy docs

- `ARCHITECTURE.md`: target system shape and boundaries
- `PLANNING.md`: detailed phase tracker with completion checkboxes
- `TECH_STACK.md`: approved stack baseline and tooling direction
- `TESTING.md`: verification rules and target test strategy
- `DEPENDENCY_POLICY.md`: rules for adding, updating, and removing dependencies
- `REPO_STRUCTURE.md`: target monorepo layout
- `CI_CD.md`: target CI and release baseline
- `SECURITY.md`: security and privacy model

## Subsystem docs

- `DATA_MODEL.md`: target persistence and entity model
- `INGEST_PIPELINE.md`: ingest design and lifecycle
- `METADATA_SYSTEM.md`: metadata precedence, normalization, and write-back
- `IPTC_FIELD_MAP.md`: canonical IPTC 2025.1 field-by-field map and normalized target inventory
- `PERFORMANCE.md`: performance priorities and benchmarking direction
- `UX_WORKFLOWS.md`: target user workflows and interaction rules

## Process docs

- `CONTRIBUTING.md`: contributor workflow expectations
- `DECISIONS.md`: ADR-style decision log
- `DEBT_REGISTER.md`: tracked technical debt and exceptions
- `ROADMAP.md`: high-level milestone view

## Research support docs

- `deep-research-report.md`: original long-form research synthesis
- `RESEARCH_SUMMARY.md`: short research summary
- `SOURCE_PATTERN_SYNTHESIS.md`: condensed pattern takeaways

## Reference examples

- `examples/XMP Side Car.XMP`: current seed XMP sidecar reference file for metadata planning and future fixture promotion
- `examples/IPTC_Fields.XMP`: current broad XMP sidecar reference file for IPTC field-map coverage
- `examples/IPTC-PhotometadataRef-Std2025.1.jpg`: current broad IPTC JPEG reference file for metadata field-family planning

## Operating rule

If a change affects architecture, workflow, testing, dependencies, or planning:

- update the relevant markdown files in the same change
- keep `PLANNING.md` status markers truthful
- record architectural deviations in `DECISIONS.md`
