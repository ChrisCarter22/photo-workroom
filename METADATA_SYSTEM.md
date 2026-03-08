# METADATA_SYSTEM.md

This document defines the target metadata system for Photo Workroom.

Current repository status on March 8, 2026:

- no metadata engine exists yet
- `examples/XMP Side Car.XMP` now exists and should be used as the first reference sidecar file for metadata planning and future tests
- `examples/IPTC_Fields.XMP` now exists and should be used as the broader XMP sidecar reference for field-map coverage work
- the rules below define the implementation target for Phase 9

## Goals

The metadata system must support:

- accurate metadata reads
- safe metadata writes
- sidecar awareness
- RAW and JPEG pair awareness
- batch edits
- template expansion inputs
- code replacement expansion for repetitive captioning
- controlled and structured keyword workflows
- rights and licensing metadata coverage
- search and index synchronization
- auditability

## Source hierarchy

The metadata system may read from:

- embedded EXIF
- embedded IPTC
- embedded XMP
- sidecar XMP
- database state

Baseline precedence rules:

- on import, the file and sidecar sources become the initial DB truth
- if sidecar and embedded data overlap, sidecar wins for the overlapping fields
- after user edits, the DB becomes the application source of truth and writes back to file or sidecar according to the chosen format policy

## Normalized metadata model

The metadata layer should normalize:

- title
- caption or description
- extended description
- keywords
- structured keyword paths and synonym-aware vocab entries
- author
- creator identity or related authorship fields where exposed
- copyright
- credit, source, and rights fields
- data-mining permission fields if the product exposes them
- rating
- color label
- GPS coordinates
- capture timestamp
- camera and lens fields
- orientation

Rules:

- normalization should be explicit and testable
- avoid scattering per-format field mapping rules across unrelated modules
- document unsupported fields rather than silently dropping them

## Current reference sidecars

The repository currently includes these sidecar reference files:

- `examples/XMP Side Car.XMP`
- `examples/IPTC_Fields.XMP`

This file already contains representative fields we expect to preserve and round-trip correctly, including:

- descriptive text such as caption or description and headline
- creator and rights-style fields
- location fields such as city, state, country, country code, and location
- keyword data
- accessibility text
- event metadata

Implementation expectation:

- use this file as an early parsing and precedence-validation input
- extend the example set with additional files once real fixture coverage is built out

Canonical mapping expectation:

- the complete field-by-field IPTC property inventory and normalized target map live in `IPTC_FIELD_MAP.md`
- changes to IPTC field support should update that file together with this document

## IPTC 2025.1 reference coverage

The JPEG reference file `examples/IPTC-PhotometadataRef-Std2025.1.jpg` represents a much broader metadata surface than a basic captioning sample.

The XMP sidecar `examples/IPTC_Fields.XMP` complements that JPEG by giving us a parseable sidecar reference for a large subset of the IPTC 2025.1 field inventory.

From local inspection of the file and the official IPTC 2025.1 tech reference, the app needs to account for these field families:

- core descriptive fields such as title, headline, caption or description, instructions, job identifier, and identifiers
- creator fields such as creator names, description writer, job title, and creator contact info
- legacy location fields such as city, sublocation, state or province, country, and country code
- structured location fields such as location created, location shown, world region, location identifiers, and reverse-geocoded place data
- keyword and classification fields such as flat keywords, subject codes, scene codes, intellectual genre, and controlled vocabulary terms
- rights and licensing fields such as copyright notice, usage terms, web statement, licensor records, copyright owner records, image creator or supplier records, encoded rights expressions, linked rights expressions, model releases, property releases, other constraints, and data-mining permissions
- accessibility fields such as alt text accessibility and extended description accessibility
- event, genre, contributor, and registry fields
- person, organisation, and product-in-image fields, including image regions and region geometry
- artwork or object metadata for works depicted in an image
- AI and digital-origin fields such as digital source type, AI system used, AI system version used, AI prompt writer, and AI prompt information
- GPS and capture-time fields

That breadth means the metadata subsystem cannot be designed around only caption, keywords, and rights notice.

## Support tiers

The metadata plan should distinguish between what users can edit directly in the first UI, what must be supported structurally, and what must at minimum be preserved.

### Tier 1: first-class editable metadata

These fields should be readable, searchable where appropriate, and directly editable in early metadata UI:

- title
- headline
- caption or description
- creator names
- creator job title
- description writer
- credit line
- source
- copyright notice
- rights usage terms and web statement
- instructions
- job identifier or transmission reference
- flat keywords
- event name
- digital source type
- rating and color label
- date created or capture time
- GPS coordinates
- legacy location fields such as city, sublocation, province, country, and country code
- accessibility alt text and extended description

### Tier 2: structured editable metadata

These fields should have explicit domain models and eventual editing support, even if the first UI is more specialized:

- creator contact info
- subject codes and scene codes
- intellectual genre and genre controlled terms
- location created and location shown structures
- contributors
- person in image, organisation in image, and product in image
- image-region geometry and region-linked entities
- licensor, copyright owner, image creator, and image supplier structures
- model release and property release identifiers and statuses
- rights-expression structures, registry IDs, and other constraints
- artwork or object structures
- AI-origin and AI-prompt metadata

### Tier 3: read, preserve, and round-trip safely

If a field is not exposed in the early UI, the system must still avoid destroying it on write-back.

Examples:

- unknown-but-valid IPTC Extension structures
- controlled vocabulary refinements and identifiers
- nested region metadata
- registry or provenance-style identifiers

Rule:

- unsupported metadata is not an excuse for destructive writes
- if the app cannot edit a field yet, it should preserve it through normalization and write-back wherever the chosen metadata library permits

## Authoring surfaces

The metadata UX should eventually expose two distinct authoring surfaces similar to Photo Mechanic:

- a batch-oriented metadata template flow for stamping shared fields across many assets
- a single-asset metadata info flow optimized for fast next and previous navigation while captioning

These are separate UX surfaces, but they must use the same underlying metadata engine and validation rules.

## Template, variable, and code replacement support

Required capabilities:

- variable expansion for filenames, captions, locations, and workflow fields
- code replacement expansion for shorthand captioning workflows
- load and save of reusable metadata templates
- predictable preview of expanded values before applying bulk writes

Captioning workflow expectation:

- code replacements are a required workflow capability for fast single-image captioning, not an optional stretch feature
- a captioner should be able to type a short code and have it expand into the intended person, team, place, or other repeated text before save

Rules:

- variable expansion must be deterministic
- unresolved variables or invalid replacement sets must fail clearly
- code replacement configuration should be portable between machines where practical

## Read pipeline

Target read flow:

1. read source file metadata
2. read sidecar metadata if present
3. normalize field names and data types
4. apply precedence rules
5. persist or refresh DB state
6. expose normalized metadata to the UI and search index

## Write pipeline

Target write flow:

1. validate user edits
2. persist the intended change in the DB transactionally
3. determine per-format write target
4. write to file or sidecar
5. refresh DB and search indexes if necessary
6. log the outcome in the audit trail

## Per-format write strategy

Baseline direction:

- JPEG and other safely writable formats: write embedded metadata directly where the chosen library supports it
- RAW formats: prefer sidecar XMP writes
- if a required format cannot be written safely with the primary metadata library, document and implement a controlled fallback

## Sidecar handling

Rules:

- detect existing sidecars during ingest and scan
- update sidecar paths during rename or move operations
- create sidecars for RAW assets when policy requires them
- never orphan a sidecar without logging and DB updates

## RAW and JPEG pair behavior

The metadata system must define:

- whether edits apply to the RAW master, the paired JPEG, or both
- how user-visible fields are presented when paired files disagree
- how paired asset relationships affect rename and export workflows

These rules must stay aligned with `DATA_MODEL.md`.

## Batch edits

Required capabilities:

- apply shared values across multiple assets
- skip or warn on fields that cannot be written safely for all selected files
- preserve per-asset values for fields not being changed
- update search indexes after the batch transaction completes

## Keyword and vocabulary support

Required capabilities:

- flat keyword entry
- controlled vocabulary support
- structured keyword hierarchies
- merge or import workflows for shared vocabularies
- synchronization between keyword UI and normalized DB tables

## Rights and policy metadata

Required capabilities:

- support common rights and licensing metadata
- preserve those fields through round-trip reads and writes
- expose policy-oriented fields such as AI or data-mining permission values if the product includes them

## Compatibility and interoperability controls

Required capabilities:

- preference snapshots or presets for metadata compatibility with other tools
- explicit control over sidecar versus embedded metadata strategies where formats allow it
- troubleshooting-oriented metadata diagnostics when interoperability issues appear

Notes:

- Photo Mechanic documents interoperability preferences and troubleshooting tools such as metadata masking and unmasking
- our open source project does not need to copy that exact UI, but it should plan for equivalent compatibility controls if real workflows need them
- the IPTC reference JPEG confirms that interoperability is not just about basic captions; it also affects structured extension fields that other tools may ignore or damage

## Time and location utilities

Required capabilities:

- capture-date adjustment workflows for multi-camera sync
- GPS metadata validation
- GPS log import or interpolation support if the workflow is included in the MVP
- reverse geocoding support for location fields when users want place names populated from coordinates

## Validation rules

Validation examples:

- GPS coordinates must stay in valid ranges
- date and time values must be parseable and time-zone assumptions must be documented
- controlled vocabulary fields such as ratings or color labels must reject invalid values
- filename-derived metadata tokens should fail clearly if required fields are missing

## Failure handling

Rules:

- metadata read failure on one file should not crash the session
- write failures must be explicit and actionable
- partial bulk-edit failures must be surfaced as partial failures, not silent success
- if fallback tooling is required for a format, document that debt and test it explicitly

## Synchronization requirements

Whenever metadata changes:

- DB state must stay consistent
- search indexes must be updated
- preview caches should be invalidated only if the change affects rendered overlays or display assumptions
- audit logs should record the change at the appropriate granularity

## Verification expectations

Target tests:

- read normalization tests
- write round-trip tests
- sidecar creation and update tests
- precedence rule tests
- batch-edit integration tests
- paired asset behavior tests

Initial fixture expectation:

- promote `examples/XMP Side Car.XMP` into the formal metadata fixture set once `tests/fixtures/metadata/` exists
- promote `examples/IPTC_Fields.XMP` into the formal metadata fixture set as the broad XMP-sidecar field-map fixture
- add `examples/IPTC-PhotometadataRef-Std2025.1.jpg` as the canonical broad-coverage IPTC fixture for JPEG metadata support

Reference-fixture expectations from the IPTC JPEG:

- assert Tier 1 fields directly
- assert that Tier 2 fields parse into structured models without loss
- assert that Tier 3 fields survive read-modify-write cycles even when not shown in the initial UI

Reference-fixture expectations from the XMP sidecar:

- assert that `examples/IPTC_Fields.XMP` remains valid XML and parseable as XMP
- assert that the parsed top-level properties stay aligned with `IPTC_FIELD_MAP.md`
