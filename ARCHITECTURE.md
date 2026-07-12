# BugRadar: Architecture

## Overview

BugRadar is a Rust/Tauri v2 desktop application for real-time diagnostics and monitoring. It watches log files, Docker containers, and system metrics; detects anomalies; groups them into Incidents; and generates AI-powered root-cause analysis with fix suggestions.

---

## Workspace Structure

```
src-tauri/
├── br-core/          # Library crate: all business logic
└── br-cli/           # Binary crate: Tauri shell + CLI entry point
```

### br-core

| Module | Responsibility |
|--------|----------------|
| `collector/file_watcher` | Watches log files using `notify`; emits raw log lines |
| `collector/docker_collector` | Polls Docker daemon via `bollard`; streams container logs |
| `collector/parser` | Normalises raw lines into structured `LogEntry` events |
| `anomaly/rolling_window` | Maintains a sliding time window of recent entries |
| `anomaly/AnomalyEngine` | Ticks every 1 s; scores entries against anomaly detectors |
| `anomaly/detectors` | Pluggable detector traits (error spike, latency, OOM, …) |
| `anomaly/incident_grouper` | Correlates anomalies across sources into `Incident` objects |
| `ai/OllamaAnalyzer` | Sends incident context to Ollama at `localhost:11434` |
| `ai/ClaudeAnalyzer` | Optional Claude API backend (user-supplied key, no telemetry) |
| `sysmon/metrics` | Collects CPU/RAM/disk metrics via `sysinfo` |
| `sysmon/docker_monitor` | Aggregates container health and resource usage |
| `db/` | SQLite migrations; persists incidents, entries, and AI reports |

### br-cli

Tauri v2 shell: registers IPC commands, mounts the React frontend, and starts all background tasks via `tokio`.

---

## Data Flow

```
LogCollector (file/docker)
        │
        ▼
    LogEntry (parsed, normalised)
        │
        ▼
  AnomalyEngine  ◄──── 1 s tick
  (rolling window + detectors)
        │  anomaly detected
        ▼
  IncidentGrouper  ──► SQLite (incidents table)
        │  new/updated incident
        ▼
  AI trigger (debounced, 5 s cooldown)
        │
        ├──► OllamaAnalyzer  →  root-cause + fix suggestions
        └──► ClaudeAnalyzer  →  (optional, if API key configured)
                │
                ▼
         SQLite (ai_reports table)
                │
                ▼
         Tauri IPC → React Frontend
```

---

## Frontend

React/TypeScript SPA served by Tauri v2. Communicates with the Rust backend exclusively via `invoke()` IPC calls. No HTTP server is exposed.

Key views:
- **Dashboard**: live metric graphs + incident feed
- **Incidents**: grouped anomaly timeline with AI analysis panel
- **Sources**: configure watched log files and Docker targets
- **Settings**: Ollama model selection, thresholds, retention policy

---

## Storage

SQLite database in the OS application data directory (`$APPDATA/BugRadar/` / `~/Library/Application Support/BugRadar/`).

Tables: `log_entries`, `incidents`, `anomalies`, `ai_reports`, `sources`, `migrations`.

---

## Security

- No external network calls except `localhost:11434` (Ollama) and optional Claude API (user-configured).
- All Tauri IPC commands are explicitly allowlisted in `src-tauri/capabilities/`.
- No telemetry, no crash reporting, no analytics.
