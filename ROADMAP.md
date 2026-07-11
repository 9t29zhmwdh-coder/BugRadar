# BugRadar Roadmap

## v0.1.0, Initial Release ✅ (2026-06-12)

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

## v0.2.0, Detector Expansion (planned)

- [ ] Wire up `Incident::should_trigger_ai()` to the collector so AI analysis
      actually auto-triggers on High-severity incidents with 3+ anomalies
      (the method exists but nothing calls it yet)
- [ ] Log level trend detector (warn→error escalation)
- [ ] Pattern-based detector (regex rules, user-defined)
- [ ] Docker health-check failure detector
- [ ] Alert suppression / snooze per incident
- [ ] Export incidents to JSON/CSV
- [ ] Keyboard shortcuts for incident triage

---

## v0.3.0, Integrations & Notifications (planned)

- [ ] Desktop notifications (OS-native) on new incidents
- [ ] Webhook output (POST to user-defined endpoint)
- [ ] Multi-source correlation (link incidents across files + containers)
- [ ] Configurable retention policy (TTL per source)
- [ ] Dark / light theme toggle
- [ ] Plugin API for custom detectors (Rust trait, dynamically loaded)

---

## v1.0.0, Stable Release (planned)

- [ ] Full test coverage for br-core (unit + integration)
- [ ] Signed macOS / Windows / Linux binaries
- [ ] Automated update check (offline-first, no telemetry)
- [ ] Comprehensive user documentation
- [ ] Accessibility audit (WCAG 2.1 AA)
- [ ] Performance: handle 100k log lines/s without UI stutter

---

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier), with an important caveat: observability/incident-management is an established commercial category (Datadog, Sentry, PagerDuty), but BugRadar is deliberately local-first and privacy-first (no cloud calls, no telemetry, RAM-only processing). A conventional multi-tenant SaaS Enterprise tier would conflict with that identity. Not ready yet; blocked on:

- [ ] No multi-machine or team aggregation story at all, by design: a Commercial tier here would need to stay a licensed fleet-dashboard companion (still local/on-prem) rather than a hosted rewrite
- [ ] No plugin API for custom detectors yet (only the built-in Docker and nginx plugins, v0.3.0 item above): the most natural Community/Commercial split would be "core engine free, paid detector packs"
- [ ] Webhook and notification integrations are still only roadmap entries (v0.3.0), nothing to gate yet

Once the plugin API (v0.3.0) lands, revisit: candidate Enterprise-only features would be paid detector packs distributed through the plugin registry and a licensed fleet-dashboard companion for aggregating multiple local BugRadar instances, with the core collector/anomaly/incident engine and desktop app staying Community/MIT.
