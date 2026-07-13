import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// ─── Types mirroring Rust models ─────────────────────────────────────────────

export type LogLevel = "trace" | "debug" | "info" | "warn" | "error" | "fatal" | "unknown";

export type WatchSourceKind =
  | { FilePath: { path: string } }
  | { DockerContainer: { container_id: string; container_name: string } }
  | "DockerAllContainers";

export interface WatchSource {
  id: string;
  label: string;
  kind: WatchSourceKind;
  parser_id: string;
  enabled: boolean;
  created_at: string;
}

export interface LogEntry {
  id: string;
  source_id: string;
  source_path: string;
  timestamp: string;
  level: LogLevel;
  message: string;
  stacktrace?: string[];
  fields: Record<string, unknown>;
  raw_lines: string[];
  parser_id: string;
  ingested_at: string;
}

export type AnomalyKind = "error_spike" | "latency_jump" | "memory_leak" | "crash_loop" | "unhandled_exception" | "database_timeout" | { Custom: string };
export type Severity = "low" | "medium" | "high" | "critical";

export interface Anomaly {
  id: string;
  detected_at: string;
  kind: AnomalyKind;
  source_id: string;
  severity: Severity;
  value: number;
  baseline: number;
  deviation_factor: number;
  contributing_entries: string[];
  incident_id?: string;
}

export type IncidentStatus = "open" | "investigating" | "resolved" | "suppressed";

export interface Incident {
  id: string;
  title: string;
  status: IncidentStatus;
  severity: Severity;
  anomaly_ids: string[];
  source_ids: string[];
  first_seen: string;
  last_seen: string;
  event_count: number;
  ai_analysis_id?: string;
  notes: Array<{ id: string; created_at: string; text: string }>;
}

export interface FixSuggestion {
  priority: number;
  title: string;
  description: string;
  command?: string;
  code_snippet?: { language: string; filename?: string; content: string; diff?: string };
}

export interface DiagnosticReport {
  id: string;
  incident_id: string;
  created_at: string;
  summary: string;
  root_cause: string;
  contributing_factors: string[];
  fix_suggestions: FixSuggestion[];
  config_conflicts: Array<{ file_path: string; key: string; current_value: string; suggested_value: string; reason: string }>;
  confidence: number;
  ai_provider: string;
  model: string;
  tokens_used?: number;
}

export interface SystemMetrics {
  cpu: { usage_percent: number; core_count: number };
  memory: { total_mb: number; used_mb: number; available_mb: number; usage_percent: number };
  disks: Array<{ name: string; mount_point: string; total_gb: number; free_gb: number; usage_percent: number }>;
  networks: Array<{ interface: string; bytes_sent: number; bytes_recv: number }>;
  load_avg_1m: number;
  collected_at: string;
}

export interface ContainerStatus {
  id: string;
  name: string;
  image: string;
  state: string;
  status: string;
  restart_count: number;
  is_crash_looping: boolean;
}

export interface ConfigInspectionResult {
  file_path: string;
  format: string;
  issues: Array<{ severity: string; key: string; message: string; suggestion?: string }>;
  key_count: number;
  parsed_ok: boolean;
}

export interface AnomalyConfig {
  window_seconds: number;
  error_spike_threshold: number;
  latency_jump_threshold: number;
  memory_growth_threshold_mb_per_min: number;
  min_samples_for_baseline: number;
  incident_correlation_window_seconds: number;
}

export interface PluginDetectorConfig {
  id: string;
  command: string;
  args: string[];
  timeout_ms: number;
}

export interface AppSettings {
  ai_provider: string;
  ollama_host: string;
  ollama_model: string;
  log_retention_days: number;
  anomaly_config: AnomalyConfig;
  custom_detectors: PluginDetectorConfig[];
}

// ─── Invoke wrappers ──────────────────────────────────────────────────────────

export const api = {
  // Settings
  getSettings: () => invoke<AppSettings>("get_settings"),
  saveSettings: (settings: AppSettings) => invoke<void>("save_settings", { settings }),
  checkAiBackend: (provider: string) => invoke<boolean>("check_ai_backend", { provider }),
  saveApiKey: (key: string) => invoke<void>("save_api_key", { key }),
  hasApiKey: () => invoke<boolean>("has_api_key"),

  // Collector
  watchSource: (source: WatchSource) => invoke<string>("watch_source", { source }),
  stopWatching: (sourceId: string) => invoke<void>("stop_watching", { sourceId }),
  listWatchSources: () => invoke<WatchSource[]>("list_watch_sources"),
  getRecentLogs: (sourceId: string, limit: number) => invoke<LogEntry[]>("get_recent_logs", { sourceId, limit }),

  // Incidents
  listIncidents: (filter?: object) => invoke<Incident[]>("list_incidents", { filter }),
  getIncident: (id: string) => invoke<Incident | null>("get_incident", { id }),
  updateIncidentStatus: (id: string, status: IncidentStatus) => invoke<void>("update_incident_status", { id, status }),
  addIncidentNote: (id: string, note: string) => invoke<void>("add_incident_note", { id, note }),

  // Anomaly
  getAnomalyConfig: () => invoke<AnomalyConfig>("get_anomaly_config"),
  saveAnomalyConfig: (config: AnomalyConfig) => invoke<void>("save_anomaly_config", { config }),

  // Sysmon
  getSystemMetrics: () => invoke<SystemMetrics>("get_system_metrics"),
  getContainerStatuses: () => invoke<ContainerStatus[]>("get_container_statuses"),
  startMetricsPolling: (intervalMs: number) => invoke<void>("start_metrics_polling", { intervalMs }),

  // Config
  inspectConfigFile: (path: string) => invoke<ConfigInspectionResult>("inspect_config_file", { path }),

  // AI
  triggerAiAnalysis: (incidentId: string) => invoke<string>("trigger_ai_analysis", { incidentId }),
  getDiagnosticReport: (id: string) => invoke<DiagnosticReport | null>("get_diagnostic_report", { id }),
};

// ─── Event listeners ──────────────────────────────────────────────────────────

export const events = {
  onLogEntry: (sourceId: string, handler: (entry: LogEntry) => void) =>
    listen<LogEntry>(`logs://entry/${sourceId}`, (e) => handler(e.payload)),

  onAnomalyDetected: (sourceId: string, handler: (anomaly: Anomaly) => void) =>
    listen<Anomaly>(`anomaly://detected/${sourceId}`, (e) => handler(e.payload)),

  onIncidentCreated: (handler: (incident: Incident) => void) =>
    listen<Incident>("incident://created", (e) => handler(e.payload)),

  onIncidentUpdated: (id: string, handler: (incident: Incident) => void) =>
    listen<Incident>(`incident://updated/${id}`, (e) => handler(e.payload)),

  onMetricsSnapshot: (handler: (metrics: SystemMetrics) => void) =>
    listen<SystemMetrics>("metrics://snapshot", (e) => handler(e.payload)),

  onAiAnalysisDone: (incidentId: string, handler: (report: DiagnosticReport) => void) =>
    listen<DiagnosticReport>(`ai://analysis/done/${incidentId}`, (e) => handler(e.payload)),
};
