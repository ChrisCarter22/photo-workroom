# DEPENDENCY_POLICY.md

Dependency additions are controlled. This project prefers built-in platform capabilities, narrow abstractions, and explicit rationale over package sprawl.

Current repository status on March 8, 2026:

- the workspace has not been scaffolded yet
- these rules apply immediately to future repository setup work
- any dependency decision that changes these defaults must be documented in `DECISIONS.md`

## Core policy

Rules:

- add the smallest dependency surface that solves the real problem
- prefer existing approved dependencies over new packages
- prefer built-in platform or language features when they are sufficient
- document why a dependency is needed before or when it is added

## Absolute rules

- JavaScript and TypeScript dependencies must use `npm` only
- `pnpm` is not approved
- Yarn is not approved
- broad speculative installs are forbidden
- duplicate libraries that solve the same problem are forbidden without documented justification
- adding a dependency without written rationale is forbidden
- disabling verification to hide dependency problems is forbidden

## Proposal requirements

Before adding a new dependency, document:

- the concrete problem being solved
- why built-in tools are not sufficient
- alternatives considered
- expected bundle, binary, or compile-time impact
- license implications
- security and maintenance posture

Accepted places to record rationale:

- `DECISIONS.md` for architectural dependencies
- PR description or task notes for small scoped additions
- both, if the dependency materially changes the system direction

## Review checklist

Every new dependency should be evaluated against these questions:

1. Does the standard library or existing code already solve this?
2. Is there already an approved dependency in the repo for this capability?
3. Is the package actively maintained and widely understood?
4. Is the license compatible with project goals?
5. What is the security posture and known vulnerability history?
6. What is the effect on package size, compile time, or runtime memory?
7. Can the same result be achieved with a smaller or narrower package?

## JavaScript and TypeScript rules

Allowed commands once the workspace exists:

- `npm install <package> --save-exact`
- `npm install -D <package> --save-exact`
- `npm ci`

Rules:

- add one dependency at a time
- use dev dependencies for lint, test, or build-only tools
- keep frontend runtime dependencies lean
- prefer workspace-aware commands such as `npm install <package> --workspace <workspace> --save-exact` when adding to a specific package
- prefer project-local installs over global installs

Not allowed:

- unreviewed bulk installs
- adding UI libraries "just in case"
- overlapping state managers, component libraries, or utility packs without explicit approval

## Rust dependency rules

Allowed direction:

- use `cargo add <crate>` with explicit version awareness once the workspace exists

Rules:

- isolate FFI-heavy crates behind narrow internal APIs
- prefer mature crates with clear maintenance
- avoid pulling in multiple crates for the same concern unless there is a documented boundary
- review transitive dependency cost when adding native bindings or heavy async frameworks

## Dev versus production scope

Classification rules:

- lint, formatting, and test tools belong in dev scope
- runtime packages must be justified against actual shipped behavior
- codegen and build helpers must not leak into production bundles unintentionally

## Security and licensing

Required checks:

- inspect license compatibility before approval
- review known vulnerabilities with `npm audit` and `cargo audit` once those toolchains exist
- avoid packages with abandoned maintenance or poor provenance
- avoid remote-code or plugin-style dependencies that bypass the project security model

## Updates and removals

Rules:

- keep lockfiles authoritative
- update dependencies intentionally, not casually
- remove unused dependencies promptly
- major-version updates require review and verification planning

## Documentation obligations

When dependency choices change:

- update `TECH_STACK.md` if the baseline stack changes
- update `ARCHITECTURE.md` if module responsibilities or boundaries change
- update `DECISIONS.md` for architectural choices
- update `PLANNING.md` if the change affects delivery phases or verification steps

## Examples

Acceptable:

- add a metadata library because the existing stack cannot write XMP sidecars and the alternatives were reviewed
- add a dev-only markdown linter if documentation validation becomes part of CI

Unacceptable:

- add a second database layer because it "might be useful later"
- add a runtime dependency without tests or documentation
- install several experimental packages and plan to clean them up later
