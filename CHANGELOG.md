# Changelog, BugRadar

All notable changes to this project will be documented in this file.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions, the Cargo workspace and the frontend npm packages, with grouped weekly updates. The file was missing, and without it a repository gets no version updates at all: security alerts only fire for disclosed vulnerabilities. The pinned `actions/checkout` SHA here had been sitting on v6 while v7.0.1 was current. Follows `engineering-standards` v0.10.0.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Removed a stray em-dash from the CHANGELOG.
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real, packaged, installable distribution exists
for end users. Real macOS/Windows/Linux installers (DMG, NSIS, AppImage/deb/rpm).

## [0.3.2] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.3.1] - 2026-07-17

### Changed

- README/README.de: marked the Anthropic/Ollama requirement as
  "(optional, for AI root-cause explanations)": detection, clustering
  and incident grouping already work without either configured.

## [0.3.0] - 2026-07-13

### Added

- Plugin API for custom detectors: configure any executable in Settings → Custom Detectors, and BugRadar spawns it once per tick per active log source, sending a JSON snapshot of that source's window on stdin and reading anomalies back from stdout. Implemented as a subprocess boundary (like an mdBook preprocessor or a pre-commit hook) rather than a dynamically loaded `.so`/`.dll`, since Rust gives no ABI stability guarantee across compiler versions. Closes the plugin-API blocker in this repo's Dual-Licensing Readiness assessment (ROADMAP.md); the multi-machine/fleet-aggregation blocker remains open by design.
- `SourceWindow` now retains the last 50 log messages per source (`recent_messages`), so custom detectors can match on real log text, not just level counts.

## [0.2.7] - 2026-07-12

### Fixed

- Removed 20 em-dashes across `GETTING_STARTED.md`, `CONTRIBUTING.md`, `SKELETON.md`, `ARCHITECTURE.md`, three Rust source comments, one CLI output string (`crates/br-cli/src/main.rs`), and one TypeScript comment. Swiss German orthography rule: no em-dash/en-dash anywhere in the repo.

## [0.2.6] - 2026-07-12

### Added

- Release workflow (`release.yml`) producing installable cross-platform artifacts (dmg, exe, msi, deb, rpm, AppImage) on every `v*` tag push, using the portfolio's established `tauri-apps/tauri-action` pattern.
- README download section (macOS DMG, Windows installer, Linux AppImage links) in both English and German.

### Fixed

- Pinned all GitHub Actions in `ci.yml` to a commit SHA instead of a mutable tag, per the portfolio's supply-chain integrity standard.
- Removed an unscoped `[build]` rustflag (`-C target-cpu=native`) from `.cargo/config.toml`. This forced every compilation, including release builds on shared CI runners, to target the exact CPU of the build machine, which can crash with an illegal instruction on end-user hardware with a different instruction set.
- Bumped `vite`/`@vitejs/plugin-react` to major versions 8/6 to resolve a moderate/high-severity esbuild dev-server request-forwarding vulnerability (npm audit).
- Switched the frontend's forced `esbuild` minifier to Vite's default minifier; the explicit `esbuild` setting was incompatible with Vite 8's destructuring lowering for the configured multi-target build list.

## [0.2.5] - 2026-07-11

### Fixed

- Removed an eszett and em-dashes from TEMPLATE_NOTES.md; the project uses Swiss German orthography.

## [0.2.4] - 2026-07-11

### Fixed

- SemVer correction: v0.1.1 added a genuine new feature (full English/German UI translation) but was versioned as a patch. Renumbered v0.1.1 through v0.1.4 to v0.2.0 through v0.2.3 (same commits, tags and releases recreated at identical SHAs), per the portfolio's SemVer discipline (patch = fix, minor = feature, major = finished product).

## [0.2.3] - 2026-07-11

### Added

- Documented Dual-Licensing readiness assessment in ROADMAP.md.

### Fixed

- Removed an em-dash from the SECURITY.md heading.

## [0.2.2] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.1] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md to the top of the file (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.2.0] - 2026-07-08

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
