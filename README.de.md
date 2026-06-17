<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>BugRadar</h1>
</div>

[🇬🇧 English Version](README.md)

**KI-gestütztes Echtzeit-Diagnose- und Monitoring-Tool, entwickelt mit Rust und Tauri.**

BugRadar überwacht Logdateien, Docker-Container und Systemmetriken in Echtzeit, erkennt Anomalien automatisch, gruppiert sie zu Incidents und generiert AI-basierte Root-Cause-Analysen mit konkreten Fix-Vorschlägen.

![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Plattform](https://img.shields.io/badge/Plattform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![Lizenz](https://img.shields.io/badge/Lizenz-MIT-green)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| **Log-Überwachung** | Echtzeit File-Tailing + Docker-Container Log-Streaming |
| **Multi-Format-Parser** | JSON, Plaintext, Nginx, Docker: mit Stacktrace-Zusammenführung |
| **Anomalie-Erkennung** | Rolling-Window-Analyse: Fehler-Spikes, Latenz-Sprünge, Memory Leaks |
| **Incident-Gruppierung** | Korreliert Anomalien innerhalb konfigurierbarer Zeitfenster |
| **KI-Root-Cause-Analyse** | Claude Haiku oder lokales Ollama generiert strukturierte Fix-Vorschläge |
| **System-Monitoring** | CPU, RAM, Disk, Netzwerk, Docker-Container-Status |
| **Config-Inspector** | Analysiert YAML/JSON/TOML-Dateien auf Fehler und Konflikte |
| **Timeline-Ansicht** | Recharts-basierte Anomalie-Timeline und Heatmap |

---

## Voraussetzungen

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- [Ollama](https://ollama.ai) oder Claude-API-Key (für KI-Analysen)
- macOS / Windows / Linux

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/BugRadar
cd BugRadar

cd frontend && npm install && cd ..
cargo tauri dev
```

### Nur CLI

```bash
cargo install --path crates/br-cli

bugradar inspect /etc/nginx/nginx.conf
bugradar metrics
bugradar incidents --open
```

---

## KI-Anbieter

| Anbieter | Einrichtung |
|---|---|
| **Claude (Anthropic)** | API-Key in Einstellungen eingeben → sicher im Keychain gespeichert |
| **Ollama (lokal)** | [Ollama](https://ollama.ai) installieren, `ollama pull llama3.2` ausführen |

Die KI-Analyse wird automatisch ausgelöst, wenn ein Incident **High Severity** mit mindestens **3 Anomalien** erreicht (30s Debounce gegen API-Spam).

---

## Architektur

```
BugRadar/
├── crates/br-core/      — Rust: Collector, Anomalie-Engine, KI, Sysmon, DB
├── crates/br-cli/       — CLI-Binary (bugradar)
├── src-tauri/           — Tauri v2 Backend + IPC-Commands
└── frontend/            — React + TypeScript + Tailwind + Recharts
```

### Datenfluss

```
LogCollector ──► AnomalyEngine ──► IncidentGrouper
     │               │                    │
  File/Docker      1s Tick            AI-Trigger
  Tail/Stream      detect            (Debounced)
                   Anomalie
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
