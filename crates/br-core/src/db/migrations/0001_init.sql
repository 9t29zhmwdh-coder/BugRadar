CREATE TABLE IF NOT EXISTS watch_sources (
    id         TEXT PRIMARY KEY NOT NULL,
    label      TEXT NOT NULL,
    kind       TEXT NOT NULL,
    parser_id  TEXT NOT NULL,
    enabled    INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS log_entries (
    id          TEXT PRIMARY KEY NOT NULL,
    source_id   TEXT NOT NULL,
    source_path TEXT NOT NULL,
    timestamp   TEXT NOT NULL,
    level       TEXT NOT NULL,
    message     TEXT NOT NULL,
    stacktrace  TEXT,
    fields      TEXT NOT NULL DEFAULT '{}',
    raw_lines   TEXT NOT NULL DEFAULT '[]',
    parser_id   TEXT NOT NULL,
    ingested_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_log_source_time ON log_entries (source_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_log_level ON log_entries (level, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_log_timestamp ON log_entries (timestamp DESC);
