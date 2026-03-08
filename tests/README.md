# tests

Cross-workspace fixtures and higher-level integration assets live here.

Current contents:

- `fixtures/metadata/` contains promoted copies of the seed XMP and IPTC reference files
- `validation/BOOTSTRAP_VALIDATION.md` records platform bootstrap and app-launch verification evidence

Current test execution split:

- app-local unit, integration, and shell smoke tests live under `apps/main-app/src/`
- root `tests/` holds shared fixtures, validation evidence logs, and future multi-package integration coverage
