# BugRadar

[🇩🇪 Deutsche Version](README.de.md)

**AI-powered real-time diagnostics and monitoring tool — built with Rust + Tauri.**

BugRadar watches your log files, Docker containers, and system metrics in real-time, automatically detects anomalies, groups them into incidents, and generates AI-driven root-cause analyses with actionable fix suggestions.

![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

---

## Features

| Feature | Description |
|---|---|
| **Log Watching** | Real-time file tailing + Docker container log streaming |
| **Multi-Format Parsing** | JSON, plaintext, nginx, Docker — with stacktrace merging |
| **Anomaly Detection** | Rolling-window analysis: error spikes, latency jumps, memory leaks |
| **Incident Grouping** | Correlates anomalies into incidents within configurable time windows |
| **AI Root-Cause Analysis** | Claude Haiku or local Ollama generates structured fix suggestions |
| **System Monitoring** | CPU, RAM, Disk, Network, Docker container status |
| **Config Inspector** | Analyzes YAML/JSON/TOML files for issues and conflicts |
| **Timeline View** | Recharts-powered anomaly timeline and heatmap |

---

## Requirements

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- macOS / Windows / Linux

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/BugRadar
cd BugRadar

cd frontend && npm install && cd ..

# Development mode
cargo tauri dev

# Release build
cargo tauri build
```

### CLI Only

```bash
cargo install --path crates/br-cli

bugradar inspect /etc/nginx/nginx.conf
bugradar metrics
bugradar incidents --open
```

---

## AI Providers

| Provider | Setup |
|---|---|
| **Claude (Anthropic)** | Enter API key in Settings → saved securely in Keychain |
| **Ollama (local)** | Install [Ollama](https://ollama.ai), run `ollama pull llama3.2` |

AI analysis is triggered automatically when an incident reaches **High severity** with at least **3 anomalies** (30s debounce to avoid API spam).

---

## Architecture

```
BugRadar/
├── crates/br-core/      — Rust: collector, anomaly detection, AI, sysmon, DB
├── crates/br-cli/       — CLI binary (bugradar)
├── src-tauri/           — Tauri v2 backend + IPC commands
└── frontend/            — React + TypeScript + Tailwind + Recharts
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

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
