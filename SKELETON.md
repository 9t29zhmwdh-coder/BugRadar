# BugRadar — Repository Skeleton

**Repo:** `9t29zhmwdh-coder/BugRadar`
**Stack:** Rust workspace · Tauri v2 · React/TypeScript · SQLite
**Initial commit:** `2ba65ea6416ad12e53e31130544b0970951ade31` (2026-06-12)

---

## File Tree

```
BugRadar/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── src-tauri/          # Rust workspace root
│   ├── br-core/        # Core library crate
│   │   └── src/
│   │       ├── collector/    # file_watcher, docker_collector, parser
│   │       ├── anomaly/      # AnomalyEngine, rolling_window, incident_grouper
│   │       ├── ai/           # ClaudeAnalyzer, OllamaAnalyzer
│   │       ├── sysmon/       # metrics, docker_monitor
│   │       └── db/           # SQLite migrations
│   └── br-cli/         # CLI binary crate
├── src/                # React/TypeScript frontend
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── PRIVACY.md
├── ROADMAP.md
├── SECURITY.md
└── SKELETON.md
```

---

## Migration Checklist

| File | Status |
|------|--------|
| SKELETON.md | ✅ pushed |
| ARCHITECTURE.md | ✅ pushed |
| CHANGELOG.md | ✅ pushed |
| CODE_OF_CONDUCT.md | ✅ pushed |
| CONTRIBUTING.md | ✅ already present |
| PRIVACY.md | ✅ pushed |
| ROADMAP.md | ✅ pushed |
| SECURITY.md | ✅ pushed |
| .github/ISSUE_TEMPLATE/bug_report.md | ✅ pushed |
| .github/ISSUE_TEMPLATE/feature_request.md | ✅ pushed |
| .github/PULL_REQUEST_TEMPLATE.md | ✅ pushed |

---

## Notes

- CI/CD workflows are not included in this skeleton (GitHub Actions requires secrets setup).
- Tauri v2 capabilities are defined in `src-tauri/capabilities/` — IPC commands must be explicitly allowlisted there.
