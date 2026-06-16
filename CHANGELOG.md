# Changelog — BugRadar

All notable changes to this project will be documented in this file.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [0.1.0] — 2026-06-12

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
