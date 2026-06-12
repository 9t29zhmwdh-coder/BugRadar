CREATE TABLE IF NOT EXISTS diagnostic_reports (
    id                   TEXT PRIMARY KEY NOT NULL,
    incident_id          TEXT NOT NULL,
    created_at           TEXT NOT NULL,
    summary              TEXT NOT NULL,
    root_cause           TEXT NOT NULL,
    contributing_factors TEXT NOT NULL DEFAULT '[]',
    fix_suggestions      TEXT NOT NULL DEFAULT '[]',
    config_conflicts     TEXT NOT NULL DEFAULT '[]',
    confidence           REAL NOT NULL DEFAULT 0.0,
    ai_provider          TEXT NOT NULL,
    model                TEXT NOT NULL,
    tokens_used          INTEGER,
    FOREIGN KEY (incident_id) REFERENCES incidents(id)
);

CREATE INDEX IF NOT EXISTS idx_reports_incident ON diagnostic_reports (incident_id);
CREATE INDEX IF NOT EXISTS idx_reports_created ON diagnostic_reports (created_at DESC);

CREATE TABLE IF NOT EXISTS app_settings (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);
