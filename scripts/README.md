# scripts

This directory contains checked-in utility scripts for deterministic local and CI verification.

Current scripts:

- `tauri-launch-smoke.mjs`: launches `npm --workspace apps/main-app run tauri:dev -- --no-watch`, waits for the backend startup marker, then terminates the app process tree and reports pass or fail
