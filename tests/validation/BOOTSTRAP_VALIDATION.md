# Bootstrap Validation Log

This file records concrete bootstrap and launch verification evidence by platform.

| Date (UTC) | Platform | Context | Commands | Result | Notes |
|---|---|---|---|---|---|
| 2026-03-08 | macOS | Local run in repository root | `npm ci`, `npm run verify:local`, `npm run test:tauri-launch` | pass | Verified from a clean local install and deterministic verification helper. |
| 2026-03-08 | Windows | GitHub Actions `Bootstrap Validation` run `22816712309` | `npm ci`, split frontend and Rust verification steps, `npm run test:tauri-launch` | fail | Failed at `cargo clippy` because `apps/main-app/src-tauri/icons/icon.ico` was missing for the Windows Tauri resource build step. |
| 2026-03-08 | Linux | GitHub Actions `Bootstrap Validation` run `22816838497` | `npm ci`, split frontend and Rust verification steps, `xvfb-run ... npm run test:tauri-launch` | pass | Hosted Ubuntu runner completed all checks and desktop launch smoke under `xvfb`. |
| 2026-03-08 | macOS | GitHub Actions `Bootstrap Validation` run `22816838497` | `npm ci`, split frontend and Rust verification steps, `npm run test:tauri-launch` | pass | Hosted macOS runner completed all checks and desktop launch smoke. |
| 2026-03-08 | Windows | GitHub Actions `Bootstrap Validation` run `22816838497` | `npm ci`, split frontend and Rust verification steps, `npm run test:tauri-launch` | pass | Hosted Windows runner completed all checks after adding `apps/main-app/src-tauri/icons/icon.ico`. |
