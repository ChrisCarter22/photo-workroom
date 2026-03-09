# DATA_MODEL.md

This document defines the target data model for Photo Workroom.

Current repository status on March 9, 2026:

- a Phase 3 baseline now exists in `crates/db` with incremental SQL migrations and startup initialization
- a Phase 4 ingest persistence helper now upserts scanned canonical assets and writes scan summary events into `audit_log`
- only a foundational subset of the target model is implemented; the structures below still describe the broader target beyond the current baseline

## Data model goals

The data model must support:

- large local libraries
- metadata-heavy workflows
- paired RAW and JPEG behavior
- sidecar and XMP behavior
- Photo Mechanic-style culling state and workflow markers
- auditability
- performant local search
- preview caching without bloating the DB
- ingest and export session history

## Core entities

| Entity | Purpose | Notes |
|---|---|---|
| `asset` | canonical record for a source file | stores path identity, type, timing, and key metadata |
| `asset_variant` | relationship between a primary asset and related variants | supports RAW plus JPEG pairs and future derived relationships |
| `sidecar` | track external XMP or other metadata sidecars | must stay in sync with file and DB rules |
| `tag` | normalized tag dictionary | avoids repeated free-form strings |
| `asset_tag` | many-to-many join between assets and tags | supports search and filtering |
| `keyword_list` | controlled vocabulary store | supports flat and structured keyword workflows |
| `keyword_term` | hierarchical or synonym-aware keyword entries | supports structured keyword paths and synonyms |
| `asset_region` | normalized region geometry and annotations | supports person, organisation, and product regions in image |
| `asset_contributor` | contributor records linked to an asset | supports structured contributor metadata |
| `asset_location` | structured location-created or location-shown records | supports IPTC Extension location structures |
| `asset_rights_party` | licensor, copyright owner, creator, supplier, and related party records | supports structured rights entities |
| `asset_release` | model and property release records | supports release identifiers and statuses |
| `asset_registry_ref` | registry and rights-expression references | supports registry IDs and linked rights metadata |
| `asset_artwork` | artwork or object metadata linked to an asset | supports depicted-work metadata |
| `ingest_session` | audit trail for imports and scans | records source, destination, timing, and result summary |
| `export_job` | audit trail for export operations | records template, destination, and result summary |
| `task_run` | visible long-running task history | ingest, upload, email, gallery, and export progress surfaces |
| `favorite_target` | saved folders or multi-folder worksets | supports Favorites-like workflow reopening |
| `preview_cache_entry` | optional DB reference to on-disk previews | keeps cache discoverable without storing image blobs in SQLite |
| `audit_log` | append-only user-visible history | records critical state changes and failures |

## Asset model

The `asset` record should eventually support fields such as:

- stable internal ID
- canonical file path
- parent folder
- original filename
- normalized file extension or media type
- file size
- filesystem timestamps as needed
- capture timestamp from metadata
- camera and lens fields
- orientation
- rating
- color label
- tag or keep flag
- reject state if adopted separately from tag
- crop state and crop parameters
- rotated or mirrored state
- manual arrangement or custom sequence position when supported
- checksum or fingerprint data when duplicate logic requires it

Rules:

- DB identity is authoritative for the application
- the file path remains the canonical lookup into the source library
- path changes must be synchronized transactionally with rename or move operations

## Variants and pairing

RAW and JPEG pair expectations:

- one asset may act as the primary or master asset
- related files such as sidecar JPEG, exported derivative, or alternate format should be modeled explicitly rather than inferred ad hoc
- pairing rules must be deterministic and documented

Example pair behaviors:

- RAW plus same-basename JPEG imported together
- sidecar XMP linked to the RAW master
- exported derivatives tracked separately from source assets unless product direction says otherwise

## Sidecar model

Sidecar expectations:

- if an external XMP file exists, store its path and relationship to the source asset
- RAW formats default to sidecar writes when direct metadata writes are unsafe or undesirable
- sidecar updates should be reflected in both the DB and audit log
- the current repository seed examples are `examples/XMP Side Car.XMP` and `examples/IPTC_Fields.XMP`

## Metadata fields

The normalized metadata model should cover:

- title
- caption or description
- extended description
- keywords
- structured keyword paths
- author
- creator identity and creator-contact structures
- copyright
- credit and source-style fields
- rights and licensing fields
- AI or data-mining permission fields if supported
- rating
- color label
- GPS coordinates
- region-linked person, organisation, and product metadata
- event, genre, subject code, and scene metadata
- release, licensor, registry, and rights-expression metadata
- camera model
- lens model
- capture date and time
- orientation
- custom metadata fields only when there is a real product need

Canonical field-inventory note:

- the complete IPTC 2025.1 property inventory and planned normalized targets are defined in `IPTC_FIELD_MAP.md`

## Metadata precedence

Baseline precedence rules:

- import: file metadata and sidecar metadata become the initial DB state
- if a sidecar exists and overlaps embedded metadata, the sidecar wins for the overlapping fields
- user edits are persisted to the DB first, then written back to file or sidecar according to format rules
- if a RAW file has no sidecar and the chosen policy requires one, the sidecar is created

Any deviation from these defaults must be documented in `METADATA_SYSTEM.md` and `DECISIONS.md`.

## Search index model

Search expectations:

- full-text search over title, caption, and tag-derived text
- indexed filter fields for capture date, rating, color label, camera data, and path-related queries
- normalized tag tables rather than repeated free-form search strings
- find and replace workflows over supported metadata fields
- selection filters for tagged, untagged, rated, colored, rotated, and cropped states
- optional geospatial indexing if map workflows become part of the MVP

## Cache model

Preview cache rules:

- previews and thumbnails are stored on disk by default
- SQLite may store cache references, sizes, timestamps, and invalidation metadata
- avoid storing large preview blobs directly in SQLite unless benchmarks prove it is necessary

## Audit and history model

Audit expectations:

- record ingest session start and completion
- record export session start and completion
- record task state changes for uploads, email sends, or gallery generation if implemented
- record metadata edits, renames, and destructive actions
- preserve enough detail to understand what happened without exposing unnecessary sensitive data

## Schema evolution rules

Rules:

- all schema changes require migrations
- migrations must be incremental and reversible where practical
- update this document and `PLANNING.md` when the data model changes materially
- if a schema change introduces user-visible risk, document it in `DECISIONS.md` and `DEBT_REGISTER.md` as needed
