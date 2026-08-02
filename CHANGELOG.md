# Changelog, BugRadar

All notable changes to this project will be documented in this file.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [1.1.5] - 2026-08-02

### Changed

- `bollard` 0.18.1 to 0.21.0, the Docker client. The options types moved out of the topic modules into `query_parameters` and lost their generic parameter, so `bollard::container::LogsOptions::<String>` becomes `bollard::query_parameters::LogsOptions`. `ListContainersOptions::filters` is optional now rather than a bare map.
- `ContainerSummary.state` is a typed enum in 0.21 instead of a string. Its `Display` prints exactly the lowercase names Docker used to send, `running`, `exited` and the rest, so the mapping onto `ContainerState` is unchanged and the monitor keeps classifying containers the way it did.

---

## [1.1.4] - 2026-08-02

### Security

- `keyring` switches from `crypto-openssl` to `crypto-rust`, which takes OpenSSL out of the Linux build. The Secret Service protocol encrypts the session between the application and the keyring daemon, and that encryption came from the OpenSSL C library, reaching this tree through `keyring` and `secret-service`. `crypto-rust` implements the same algorithms the specification prescribes, AES-CBC with SHA-2 and HKDF, from the RustCrypto crates. The wire format belongs to the specification rather than to either implementation, so an existing keyring stays readable.
- Afterwards `Cargo.lock` holds no `openssl` package at all, where it held one before. With it goes a C library with a long CVE history and the requirement to have its development headers present when building for Linux. macOS and Windows never compiled this path; both use their native keychain.

---

## [1.1.3] - 2026-08-01

### Fixed

- Dependency groups outside `github-actions` now carry `update-types: ["minor", "patch"]`, so a major version arrives as its own pull request. Without it, a breaking change gets bundled with a dozen harmless patches: the combined diff cannot be reviewed honestly, and one incompatible package blocks every other update in the group, including a security patch that is needed at once. The template in `engineering-standards` has documented this since it happened; this repository had not applied it.

---

## [1.1.2] - 2026-07-31

### Changed

- Dependabot no longer retries the `glib` update it cannot perform. GHSA-wrw7-89jp-8q8g is fixed in 0.20, and this project cannot reach it: `tauri` 2.x pins `gtk ^0.18`, `gtk` 0.18 requires `glib ^0.18`, and no patched 0.18.x exists, so cargo rejects the upgrade rather than resolving it. Three attempts had already failed identically, each one a red run on `main` that meant nothing. Only the unreachable versions are ignored, so a backported 0.18.x fix would still arrive, and the advisory itself stays visible in the Security tab. The block goes away when Tauri moves to gtk-rs 0.20, which is the condition already recorded in `SECURITY.md`.

---

## [1.1.1] - 2026-07-31

### Changed

- `@vitejs/plugin-react` 6.0.3 to 6.0.4 and `vite` to its matching patch, both build-time dependencies that do not ship in the application. The Dependabot pull request had been open since 29 July without being mergeable: the ruleset requires `Check (ubuntu-latest)`, `(macos-latest)` and `(windows-latest)`, and the branch predated the move to a three-platform matrix, so it only ever reported a single `Check` and the required contexts never arrived. A rebase onto current main picked up the new workflow.

---

## [1.1.0] - 2026-07-31

### Security

- **Log lines were sent to Anthropic even with no API key configured.** Triggering an incident analysis without a stored key built the Claude analyzer with an empty key and issued the request anyway. The call failed at the API's authentication check, but the incident summary and the surrounding log lines had already been transmitted by then. The empty-key case now returns an error before any request is made.
- **The default AI provider is now Ollama rather than Claude.** A fresh installation previously defaulted to the cloud backend, so the first analysis anyone ran sent their logs off the machine unless they had found the setting first. Existing installations keep whatever they have stored; only the default for new ones changes.
- `SECURITY.md` claimed "no external network calls except localhost (Ollama)". That was untrue whenever the Claude backend was selected. It now describes both backends and what each transmits.

### Added

- `SECURITY.md` records GHSA-wrw7-89jp-8q8g against `glib` 0.18.5, which cannot be fixed from this repository because Tauri 2.11.5 pins `gtk ^0.18` and no patched 0.18.x exists. The entry states the dependency path, the failed upgrade attempt, the exposure, and what would end it.

### Fixed

- The supported-versions table still listed `0.1.x`, a line that no longer exists. It lists `1.0.x`.

---

## [1.0.14] - 2026-07-31

### Changed

- The exclusion paragraph now names LogLens as well. LogLens 1.0.10 points here for the live-watching case; without the reverse pointer a visitor landing on this side still had to work out which of the two applies to them.

---

## [1.0.13] - 2026-07-31

### Changed

- Both READMEs now open with the situation the tool is for, which is an error spike you notice too late to reconstruct from the log, rather than with the four things the code does. A short paragraph sends people who need fleet alerting with on-call routing to Grafana, Prometheus or Datadog instead.

---

## [1.0.12] - 2026-07-30

### Added

- `Cargo.lock` is committed. It was listed in `.gitignore`, so every build resolved dependencies afresh and no two builds were guaranteed to use the same versions. For an application rather than a library the lock file belongs in the repository: it is what makes a release reproducible and what lets a security advisory be checked against what actually shipped.
- A test that stores a secret and reads it back from a second process. A store-then-read inside one process is satisfied by `keyring`'s in-memory fallback, so it would pass even with no platform backend compiled in, which is how the same defect went unnoticed in two other tools this month. The test treats a failure to reach the store as a missing service rather than a missing backend, which is what a Linux CI runner without a D-Bus secret service looks like.

---

## [1.0.11] - 2026-07-30

### Changed

- The `Check` job runs on Linux, macOS and Windows instead of macOS alone. The release builds artifacts for all three, so a fault that only shows on one of the other two reached a release before anything noticed.
- The Linux leg installs the GTK and WebKit packages Tauri builds against. The runner ships neither, and without them `cargo check` fails at `gobject-2.0` before reaching any code. The release workflow already installed the same packages, which is why releases worked while no Linux check existed.
- The ruleset now requires `Check (ubuntu-latest)`, `Check (macos-latest)` and `Check (windows-latest)` in place of the single `Check`. A matrix renames the job, so leaving the old context required would have left a check that can never report again.

---

## [1.0.10] - 2026-07-29

### Changed

- TypeScript 5.9 to 7.0, the native compiler. Type checking this project drops from about 0.60 s to about 0.21 s, measured over three runs each. The published figure of ten times faster describes large codebases; at 25 source files the gain is smaller but real.

### Added

- `src/vite-env.d.ts`, referencing `vite/client`. Vite has always declared modules for `*.css` and the other asset types it handles, but nothing in this project pulled that declaration in. TypeScript 5 accepted the untyped side-effect import of `index.css` regardless; TypeScript 7 rejects it with `TS2882`. The file is part of Vite's own project scaffold and was simply missing here.
### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.9] - 2026-07-29

### Changed

- Tailwind CSS 3 to 4. This is a migration rather than a version bump: the PostCSS plugin is replaced by `@tailwindcss/vite`, and the three `@tailwind` directives in `index.css` become a single `@import "tailwindcss"`.
- The generated stylesheet grows from 3.6 KB to 6.1 KB gzipped. Tailwind 4 exposes its design tokens as CSS custom properties in a `@layer theme` block, which is emitted whether or not a token is referenced. That is the cost of the new architecture, not a regression in what gets purged.

### Removed

- `autoprefixer` and `postcss`. Tailwind 4 handles vendor prefixing itself, and with the Vite plugin in place there is no PostCSS pipeline left for them to sit in.
- `postcss.config.js`, which only wired up those two plugins.
- `tailwind.config.ts`. Its `content` array is now detected automatically, and its only other content was a `brand` colour scale that no file referenced: none of the five hex values appeared in the built stylesheet, and no `brand-*` class appeared in any component. Tailwind 4 no longer picks up a JavaScript config automatically, so leaving the file would have been worse than deleting it: someone editing a colour there would see no effect.

---

## [1.0.8] - 2026-07-29

### Changed

- React 18 to 19, with the matching `@types` packages. The upgrade needs no source change here: the codebase uses no `forwardRef`, no `defaultProps`, no `propTypes` and no `ReactDOM.render`, which is where React 19's removals land.
- `zustand` 4 to 5, `date-fns` 3 to 4 and `recharts` 2 to 3, none of which required a source change either.
- These four arrived in one grouped pull request together with the Tailwind 4 upgrade. Tailwind is a real migration and is kept separate, so the part that is a version bump can be reviewed as one.

---

## [1.0.7] - 2026-07-29

### Changed

- `reqwest` updated from 0.12 to 0.13. The `rustls-tls` feature no longer exists in 0.13 and is replaced by `rustls`, so the automated dependency update could not build: it can raise a version number but not rename a feature.

### Security

- TLS now trusts the operating system's certificate store rather than a bundled root set. The `rustls` feature in 0.13 pulls in `rustls-platform-verifier`, where 0.12 resolved roots independently of the host. A machine that trusts an internal certificate authority, which is the normal case behind a corporate proxy, now works without extra configuration. The other side of that is real and worth naming: the trust decision moves to the machine the tool runs on, so a tampered local certificate store is enough to intercept the connection.
- The rustls crypto provider changes from `ring` to `aws-lc-rs`, which is what the `rustls` feature selects in 0.13.

---

## [1.0.6] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.5:

- chore(ci): bump the actions group across 1 directory with 4 updates

---

## [1.0.5] - 2026-07-28

### Fixed

- The CodeQL job requested `packages: read`, `actions: read` and `contents: read` at job level, repeating grants the workflow level already provides. OpenSSF Scorecard counts that as excessive token permissions and scores `Token-Permissions` at 0 out of 10 for it. The job now requests only `security-events: write`, which is the one grant that genuinely exceeds the workflow default.

## [1.0.4] - 2026-07-28

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup decides on its own when to run and skips pull requests that touch no code of a given language. A dependency pull request changing only a lock file therefore reported `skipping` on `Analyze (actions)`, `Analyze (javascript-typescript)` and `Analyze (rust)` forever, and since those are required checks, every such pull request was permanently unmergeable. The workflow runs on every pull request regardless of what changed, so the checks are always produced.
- The generic `CodeQL` status check was removed from the ruleset's required checks. It is produced only by the default setup and no longer exists after this change. The four language analyses stay required, so the merge gate is unchanged in substance and now fires more reliably than before.

## [1.0.3] - 2026-07-28

### Security

- `postcss` updated to 8.5.24, closing a high-severity path traversal in the source map auto-loading via `sourceMappingURL` that affects all versions up to and including 8.5.17.

Applied as a normal pull request rather than by merging Dependabot's, because Dependabot pull requests cannot currently pass this repository's required checks: CodeQL runs through GitHub's default setup, which does not trigger on a pull request that only touches a lock file, so its checks report `skipping` and never turn green. Bypassing a required check is not an option per `standards/ci-cd.md` section 7, so the fix takes the route that runs the full pipeline.

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
