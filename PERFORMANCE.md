# PERFORMANCE.md

Performance is a first-class requirement. This document records the performance philosophy and target measurement areas for Photo Workroom.

Current repository status on March 8, 2026:

- no benchmarks or implementation code exist yet
- the targets below are planning guidance for Phases 6, 7, 11, 12, 13, and 14

## Performance goals

The system must feel fast in the workflows users care about most:

- ingest
- culling
- contact-sheet browsing
- preview switching
- metadata editing
- search, filter, and sort
- export planning and execution

## Performance principles

- optimize the workflows users experience directly
- measure before changing architecture for speed
- keep the UI responsive even when background work is heavy
- prefer bounded concurrency over uncontrolled parallelism
- treat correctness regressions as unacceptable performance tradeoffs

## Key workloads

### Ingest and scan

Goals:

- discover files quickly
- avoid blocking the UI
- batch DB writes efficiently
- prevent disk thrashing

### Contact sheet and browsing

Goals:

- fast initial render of visible assets
- smooth scrolling at realistic library sizes
- lazy loading of thumbnails
- bounded memory use

### Preview loading

Goals:

- make next and previous navigation feel immediate whenever possible
- use cache intelligently
- avoid redundant decode work

### Search and filter

Goals:

- keep text search and filter changes responsive
- index the fields people actually query
- debounce expensive UI query churn without making the app feel sluggish

### Export

Goals:

- provide honest progress for long-running work
- keep throughput high without overwhelming the system
- isolate per-file failures from whole-job collapse where possible

## System-level guidance

### Filesystem and scanning

Rules:

- throttle scan worker counts
- prefer incremental detection over repeated full rescans when the product supports it
- collect the minimum required file metadata in the first pass and defer heavier work when practical

### Database

Rules:

- use transactions for bulk writes
- keep WAL mode enabled during normal runtime
- index frequently queried fields deliberately
- tune pragmas only after measuring real workloads

### Previews and thumbnails

Rules:

- generate previews in background workers
- cache preview outputs on disk
- avoid loading full-resolution images when a thumbnail or preview will do
- prune cache entries according to size and staleness policy

### Renderer behavior

Rules:

- virtualize large lists and grids
- avoid unnecessary re-renders during scrolling
- debounce expensive search or filter recomputation when appropriate
- preserve keyboard responsiveness during background work

### Helper prioritization

Rules:

- visible thumbnails and the active preview get first claim on decode and I/O capacity
- ingest, rename, metadata, FTP, upload, email, gallery, or print helpers must yield enough capacity for scrolling and preview navigation to stay responsive
- long-running bulk jobs should publish progress through task surfaces instead of blocking interaction
- separate helper queues are preferred over one shared background pool when the workloads have different latency needs

### Memory

Rules:

- keep only visible or near-visible previews in memory
- release decoded data promptly
- avoid whole-library in-memory models when the DB can serve paged results

## Benchmarking program

The benchmark suite should eventually cover:

- scan throughput on a realistic fixture library
- ingest throughput for large batches
- thumbnail generation time
- preview latency by media type
- search latency for common queries
- export throughput by format and settings

Benchmark rules:

- use a stable fixture corpus
- record environment assumptions
- compare against prior baselines
- document accepted regressions explicitly

## Performance review triggers

Run focused performance review when:

- a feature introduces a new heavy dependency
- query shape changes materially
- preview generation strategy changes
- a release candidate shows user-visible lag

## Documentation and verification

When a performance optimization lands:

- update benchmarks or measurement notes
- update `DECISIONS.md` if the optimization changes architecture
- update `PLANNING.md` if the optimization affects acceptance criteria
