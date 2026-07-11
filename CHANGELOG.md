# Changelog, BugRadar

All notable changes to this project will be documented in this file.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [0.1.4] - 2026-07-11

### Added

- Documented Dual-Licensing readiness assessment in ROADMAP.md.

### Fixed

- Removed an em-dash from the SECURITY.md heading.

## [0.1.3] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.1.2] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md to the top of the file (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.1.1] - 2026-07-08

### Fixed

- App crashed on every launch: `#[tokio::main]` created a tokio runtime, then the
  setup hook called `tauri::async_runtime::block_on()` from within it, which
  panics ("Cannot start a runtime from within a runtime"). Changed `main()` to
  a plain sync function, the standard Tauri v2 pattern
- Missing `keyring`, `reqwest` and `sqlx` dependencies in `src-tauri/Cargo.toml`;
  the app crate failed to compile at all. Promoted both to workspace dependencies
  shared with `br-core`
- Missing `src-tauri/capabilities/` permissions were blocking the event system
- Icons referenced in `tauri.conf.json` did not exist in the repo
- Duplicate "Watch Sources" heading rendered twice on the Settings page
- CI excluded the `bugradar-app` crate from all checks, hiding all of the above
- README claimed AI analysis auto-triggers on high-severity incidents; the
  underlying `should_trigger_ai()` method exists but is not called anywhere.
  Corrected to describe the actual manual trigger, noted as a ROADMAP item
- README's AI Providers table listed "Ollama" twice and never mentioned Claude,
  despite Claude being the default provider

### Added

- Full English/German UI translation (the app was previously English-only
  with no language toggle)
- README onboarding sections: how it runs, screenshot, in practice, uninstall/cleanup

## [0.1.0] - 2026-06-12

### Added

- Real-time log file monitoring using `notify` (cross-platform)
- Docker container log streaming via `bollard`
- System metrics collection (CPU, RAM, disk) via `sysinfo`
- `AnomalyEngine` with 1 s tick and pluggable detector traits
- `IncidentGrouper`: correlates anomalies across sources into grouped incidents
- AI root-cause analysis with fix suggestions:
  - Local inference via Ollama (`localhost:11434`)
  - Optional Claude API backend (user-supplied key, no telemetry)
- SQLite persistence for log entries, incidents, anomalies, and AI reports
- Tauri v2 desktop shell with React/TypeScript frontend
- Dashboard view with live metric graphs and incident feed
- Incident timeline with AI analysis panel
- Source configuration UI (log files + Docker targets)
- Bilingual README (English / German)
- CONTRIBUTING.md with development setup guide
