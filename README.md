<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>BugRadar</h1>

</div>

[🇩🇪 Deutsche Version](README.de.md)

**AI-powered real-time diagnostics and monitoring tool, built with Rust and Tauri.**

BugRadar watches your log files, Docker containers, and system metrics in real-time, automatically detects anomalies, groups them into incidents, and generates AI-driven root-cause analyses with actionable fix suggestions.

[![CI](https://github.com/9t29zhmwdh-coder/BugRadar/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/BugRadar/actions) ![Platform](https://img.shields.io/badge/Platform-macOS_%7C_Windows-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Claude](https://img.shields.io/badge/AI-Claude-black?logo=anthropic&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)

> **How it runs:** BugRadar is a native desktop app, not a server or browser tool. It opens as its own window and has no tray icon or background service; it only watches sources and collects metrics while the window is open.

![BugRadar](docs/screenshot.png)

---

BugRadar's UI is available in English (default) and German; switch anytime with the language toggle in the bottom-left corner.

**In practice:** you point BugRadar at a log file or Docker container, it flags anomalies (error spikes, latency jumps) as they happen, groups related ones into a single incident, and on request asks Claude or a local Ollama model to explain the root cause with concrete fix suggestions.

## Features

| Feature | Description |
|---|---|
| **Log Watching** | Real-time file tailing + Docker container log streaming |
| **Multi-Format Parsing** | JSON, plaintext, nginx, Docker: with stacktrace merging |
| **Anomaly Detection** | Rolling-window analysis: error spikes, latency jumps, memory leaks |
| **Incident Grouping** | Correlates anomalies into incidents within configurable time windows |
| **AI Root-Cause Analysis** | Claude (Anthropic API, default) or a local Ollama model generates structured fix suggestions |
| **System Monitoring** | CPU, RAM, Disk, Network, Docker container status |
| **Config Inspector** | Analyzes YAML/JSON/TOML files for issues and conflicts |
| **Timeline View** | Recharts-powered anomaly timeline and heatmap |

---

## Requirements

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- An [Anthropic API key](https://console.anthropic.com/) (default AI provider) or [Ollama](https://ollama.ai) running locally
- macOS / Windows / Linux

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/BugRadar
cd BugRadar

cd frontend && npm install && cd ..
cargo tauri dev
```

### CLI Only

```bash
cargo install --path crates/br-cli

bugradar inspect /etc/nginx/nginx.conf
bugradar metrics
bugradar incidents --open
```

---

## Uninstall / Cleanup

- Delete the app bundle
- Remove the local database: platform-specific app data directory (`bugradar.sqlite`), resolved via Tauri's `app_data_dir`
- Remove the stored API key from Keychain Access.app (search for "BugRadar")

No other files or background services are left behind.

---

## AI Providers

| Provider | Setup |
|---|---|
| **Claude (Anthropic)** | Default. Add your API key in Settings; stored in the OS keychain |
| **Ollama (local)** | Install [Ollama](https://ollama.ai), run `ollama pull llama3.2`, set the host/model in Settings |

AI analysis runs on demand: click "Run AI Analysis" on any incident. (Auto-triggering for High-severity incidents with 3+ anomalies is implemented in `Incident::should_trigger_ai()` but not wired up to the collector yet, see ROADMAP.md.)

---

## Architecture

```
BugRadar/
├── crates/br-core/      # Rust: collector, anomaly detection, AI, sysmon, DB
├── crates/br-cli/       # CLI binary (bugradar)
├── src-tauri/           # Tauri v2 backend + IPC commands
└── frontend/            # React + TypeScript + Tailwind + Recharts
```

### Data Flow

```
LogCollector ──► AnomalyEngine ──► IncidentGrouper
     │               │                    │
  File/Docker      1s tick            AI Trigger
  tail/stream      detect            (debounced)
                   anomaly
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/BugRadar?color=6b7280&style=flat-square) · **License:** MIT