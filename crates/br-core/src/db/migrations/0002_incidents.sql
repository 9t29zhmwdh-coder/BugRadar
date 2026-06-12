CREATE TABLE IF NOT EXISTS anomalies (
    id                   TEXT PRIMARY KEY NOT NULL,
    detected_at          TEXT NOT NULL,
    kind                 TEXT NOT NULL,
    source_id            TEXT NOT NULL,
    severity             TEXT NOT NULL,
    value                REAL NOT NULL,
    baseline             REAL NOT NULL,
    deviation_factor     REAL NOT NULL,
    contributing_entries TEXT NOT NULL DEFAULT '[]',
    incident_id          TEXT
);

CREATE TABLE IF NOT EXISTS incidents (
    id              TEXT PRIMARY KEY NOT NULL,
    title           TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'open',
    severity        TEXT NOT NULL,
    anomaly_ids     TEXT NOT NULL DEFAULT '[]',
    source_ids      TEXT NOT NULL DEFAULT '[]',
    first_seen      TEXT NOT NULL,
    last_seen       TEXT NOT NULL,
    event_count     INTEGER NOT NULL DEFAULT 1,
    ai_analysis_id  TEXT,
    notes           TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_anomalies_source_time ON anomalies (source_id, detected_at DESC);
CREATE INDEX IF NOT EXISTS idx_anomalies_incident ON anomalies (incident_id);
CREATE INDEX IF NOT EXISTS idx_incidents_status ON incidents (status, last_seen DESC);
CREATE INDEX IF NOT EXISTS idx_incidents_severity ON incidents (severity, last_seen DESC);
