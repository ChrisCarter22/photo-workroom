# SECURITY.md

This document defines the security model for Photo Workroom.

Current repository status on March 8, 2026:

- a minimal Tauri shell now exists with an explicit `health_check` command, a strict CSP, and no renderer-side filesystem or shell escape hatches
- broader privileged workflows such as filesystem scanning, ingest, and metadata writes are not implemented yet
- these rules remain the security baseline for future privileged work

## Security goals

- protect local user data
- preserve local-first expectations
- minimize attack surface
- constrain privileged operations
- keep supply-chain risk manageable
- avoid unsafe plugin and scripting behavior

## Core security posture

Design assumptions:

- the application is local-first and should not send user data over the network by default
- the UI must not have unrestricted access to the filesystem or shell
- privileged operations must be concentrated in the backend and validated explicitly
- no hidden telemetry, analytics, or cloud sync should appear without separate review

## Trust boundaries

The primary trust boundary is:

- Rust core and explicitly exposed backend commands: trusted and privileged
- WebView renderer: less trusted, sandboxed, and limited to explicit IPC calls

Rules:

- expose only the minimum IPC surface required for real features
- validate inputs on every IPC boundary
- never expose generic "run command" or unrestricted file APIs to the UI
- keep Tauri permissions and capabilities narrow

## Filesystem and data handling

Rules:

- user-selected paths must be validated and normalized before use
- writes should prefer temporary files plus atomic rename where practical
- destructive operations must be explicit and user-visible
- logs and audit records should avoid storing unnecessary sensitive data
- the app must remain functional offline

## Dependency hygiene

Rules:

- follow `DEPENDENCY_POLICY.md`
- review new dependencies for security posture and maintenance health
- run `npm audit` and `cargo audit` once the workspace exists
- remove abandoned or unnecessary packages promptly

## Tauri and renderer security

Required posture:

- keep the renderer sandboxed
- enforce a strict content security policy
- do not enable remote script loading
- do not enable Node-style unrestricted renderer access
- route privileged work through explicit `invoke()` commands only

## Plugin and extension security

Default rule:

- arbitrary plugin execution is not allowed

If extensions are introduced later:

- prefer signed sidecar binaries or another tightly controlled extension mechanism
- define a versioned API contract
- isolate extension failures from the core app
- never auto-download and execute untrusted code

## Secrets management

Rules:

- do not commit secrets
- prefer OS keychain or other secure local secret storage if secrets ever become necessary
- document secret lifecycle and storage if integrations requiring credentials are added

## Logging and privacy

Rules:

- keep logs local by default
- do not ship remote telemetry without explicit opt-in and documentation
- redact or minimize personally identifying or sensitive path information when possible
- if crash logs are kept, explain where they live and how users can remove them

## Update integrity

If application updates are implemented:

- require signed update artifacts
- validate update origin and integrity
- document rollback behavior for failed upgrades

## Incident response expectations

If a security issue is identified:

- document scope and severity
- fix the issue instead of hiding the symptom
- review whether the issue changes architecture or dependency rules
- update `DEBT_REGISTER.md` and `DECISIONS.md` if the fix changes policy or leaves residual risk
