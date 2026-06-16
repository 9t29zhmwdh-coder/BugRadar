# BugRadar — Roadmap

## v0.1.0 — Initial Release ✅ (2026-06-12)

- Real-time log file monitoring via `notify`
- Docker container log streaming via `bollard`
- System metrics collection via `sysinfo`
- AnomalyEngine with pluggable detectors (error spike, OOM, latency)
- IncidentGrouper: correlates anomalies into grouped incidents
- AI root-cause analysis via Ollama (local) and Claude API (optional)
- SQLite persistence for incidents and AI reports
- Tauri v2 desktop shell with React/TypeScript frontend
- Bilingual README (EN/DE)

---

## v0.2.0 — Detector Expansion (planned)

- [ ] Log level trend detector (warn→error escalation)
- [ ] Pattern-based detector (regex rules, user-defined)
- [ ] Docker health-check failure detector
- [ ] Alert suppression / snooze per incident
- [ ] Export incidents to JSON/CSV
- [ ] Keyboard shortcuts for incident triage

---

## v0.3.0 — Integrations & Notifications (planned)

- [ ] Desktop notifications (OS-native) on new incidents
- [ ] Webhook output (POST to user-defined endpoint)
- [ ] Multi-source correlation (link incidents across files + containers)
- [ ] Configurable retention policy (TTL per source)
- [ ] Dark / light theme toggle
- [ ] Plugin API for custom detectors (Rust trait, dynamically loaded)

---

## v1.0.0 — Stable Release (planned)

- [ ] Full test coverage for br-core (unit + integration)
- [ ] Signed macOS / Windows / Linux binaries
- [ ] Automated update check (offline-first, no telemetry)
- [ ] Comprehensive user documentation
- [ ] Accessibility audit (WCAG 2.1 AA)
- [ ] Performance: handle 100k log lines/s without UI stutter
