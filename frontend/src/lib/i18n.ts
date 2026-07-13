import { create } from "zustand";

export type Lang = "en" | "de";

const STORAGE_KEY = "bugradar_lang";

interface Dict {
  [key: string]: string | Dict;
}

const translations: Record<Lang, Dict> = {
  en: {
    nav: {
      dashboard: "Dashboard", logs: "Logs", incidents: "Incidents",
      timeline: "Timeline", config: "Config", settings: "Settings",
    },
    dashboard: {
      collectingMetrics: "Collecting metrics...",
      allClear: "All clear, no open incidents",
      activeIncidents: "Active Incidents",
      loadingIncidents: "Loading incidents...",
      anomalies: "anomalies",
    },
    logStream: {
      source: "Source:",
      noLogEntries: "No log entries yet...",
      selectSource: "Select a source to view logs",
      stack: "stack",
    },
    incidents: {
      title: "Incidents", total: "total",
      selectIncident: "Select an incident to view details",
      status: "Status:", sources: "Sources:",
      analyzing: "Analyzing...", runAiAnalysis: "Run AI Analysis",
      triggersInfo: "Triggers Claude/Ollama root-cause analysis",
      notes: "Notes", addNotePlaceholder: "Add a note...", add: "Add",
      firstSeen: "First seen", lastSeen: "Last seen",
      anomalies: "anomalies", events: "events",
    },
    aiReport: {
      title: "AI Diagnostic Report", confidence: "confidence",
      summary: "Summary", rootCause: "Root Cause",
      contributingFactors: "Contributing Factors", fixSuggestions: "Fix Suggestions",
      priority: "Priority", tokensUsed: "tokens used",
    },
    config: {
      title: "Config Inspector", inspecting: "...", inspect: "Inspect",
      format: "Format:", keys: "Keys:", valid: "Valid", parseError: "Parse Error",
      noIssues: "No issues found",
    },
    timeline: {
      systemTimeline: "System Timeline", collectingData: "Collecting data...",
    },
    settings: {
      loading: "Loading...", title: "Settings",
      aiProvider: "AI Provider", provider: "Provider",
      apiKey: "API Key", saved: "saved", notSet: "not set",
      save: "Save", ollamaHost: "Ollama Host", model: "Model",
      anomalyThresholds: "Anomaly Thresholds",
      errorSpikeThreshold: "Error Spike Threshold",
      latencyJumpThreshold: "Latency Jump Threshold",
      windowSeconds: "Window (seconds)",
      watchSources: "Watch Sources", remove: "Remove",
      labelOptional: "Label (optional)", watch: "Watch",
      savedCheck: "Saved", saveSettings: "Save Settings",
      customDetectors: "Custom Detectors",
      customDetectorsHint: "Runs your own executable as a detector: BugRadar sends it a JSON snapshot of a source's window on stdin once per tick and reads anomalies back from stdout. See README.",
      command: "Command", args: "Args (space-separated)",
      timeoutMs: "Timeout (ms)", addDetector: "Add Detector",
    },
  },
  de: {
    nav: {
      dashboard: "Übersicht", logs: "Logs", incidents: "Vorfälle",
      timeline: "Verlauf", config: "Konfiguration", settings: "Einstellungen",
    },
    dashboard: {
      collectingMetrics: "Metriken werden erfasst...",
      allClear: "Alles klar, keine offenen Vorfälle",
      activeIncidents: "Aktive Vorfälle",
      loadingIncidents: "Vorfälle werden geladen...",
      anomalies: "Anomalien",
    },
    logStream: {
      source: "Quelle:",
      noLogEntries: "Noch keine Log-Einträge...",
      selectSource: "Wähle eine Quelle, um Logs anzuzeigen",
      stack: "Stack",
    },
    incidents: {
      title: "Vorfälle", total: "gesamt",
      selectIncident: "Wähle einen Vorfall, um Details zu sehen",
      status: "Status:", sources: "Quellen:",
      analyzing: "Analysiere...", runAiAnalysis: "KI-Analyse starten",
      triggersInfo: "Löst Claude/Ollama-Ursachenanalyse aus",
      notes: "Notizen", addNotePlaceholder: "Notiz hinzufügen...", add: "Hinzufügen",
      firstSeen: "Zuerst gesehen", lastSeen: "Zuletzt gesehen",
      anomalies: "Anomalien", events: "Ereignisse",
    },
    aiReport: {
      title: "KI-Diagnosebericht", confidence: "Konfidenz",
      summary: "Zusammenfassung", rootCause: "Ursache",
      contributingFactors: "Mitwirkende Faktoren", fixSuggestions: "Lösungsvorschläge",
      priority: "Priorität", tokensUsed: "Tokens verwendet",
    },
    config: {
      title: "Konfigurations-Inspektor", inspecting: "...", inspect: "Prüfen",
      format: "Format:", keys: "Schlüssel:", valid: "Gültig", parseError: "Parse-Fehler",
      noIssues: "Keine Probleme gefunden",
    },
    timeline: {
      systemTimeline: "System-Verlauf", collectingData: "Daten werden erfasst...",
    },
    settings: {
      loading: "Lädt...", title: "Einstellungen",
      aiProvider: "KI-Anbieter", provider: "Anbieter",
      apiKey: "API-Key", saved: "gespeichert", notSet: "nicht gesetzt",
      save: "Speichern", ollamaHost: "Ollama-Host", model: "Modell",
      anomalyThresholds: "Anomalie-Schwellenwerte",
      errorSpikeThreshold: "Fehler-Spitzen-Schwelle",
      latencyJumpThreshold: "Latenz-Sprung-Schwelle",
      windowSeconds: "Fenster (Sekunden)",
      watchSources: "Überwachte Quellen", remove: "Entfernen",
      labelOptional: "Bezeichnung (optional)", watch: "Beobachten",
      savedCheck: "Gespeichert", saveSettings: "Einstellungen speichern",
      customDetectors: "Eigene Detektoren",
      customDetectorsHint: "Führt eine eigene ausführbare Datei als Detektor aus: BugRadar sendet ihr einmal pro Tick einen JSON-Snapshot des Fenster-Zustands einer Quelle via stdin und liest Anomalien von stdout zurück. Siehe README.",
      command: "Befehl", args: "Argumente (leerzeichengetrennt)",
      timeoutMs: "Timeout (ms)", addDetector: "Detektor hinzufügen",
    },
  },
};

interface LangState {
  lang: Lang;
  setLang: (lang: Lang) => void;
  toggle: () => void;
}

export const useLangStore = create<LangState>((set) => ({
  lang: (localStorage.getItem(STORAGE_KEY) as Lang) || "en",
  setLang: (lang) => {
    localStorage.setItem(STORAGE_KEY, lang);
    set({ lang });
  },
  toggle: () =>
    set((s) => {
      const next: Lang = s.lang === "en" ? "de" : "en";
      localStorage.setItem(STORAGE_KEY, next);
      return { lang: next };
    }),
}));

export function getLang(): Lang {
  return useLangStore.getState().lang;
}

function resolve(dict: Dict, path: string): string {
  const parts = path.split(".");
  let node: string | Dict | undefined = dict;
  for (const p of parts) {
    node = typeof node === "object" ? node[p] : undefined;
  }
  return typeof node === "string" ? node : path;
}

export function t(path: string): string {
  return resolve(translations[getLang()], path);
}

export function useT() {
  const lang = useLangStore((s) => s.lang);
  return (path: string) => resolve(translations[lang], path);
}

export function dateLocale(): string {
  return getLang() === "de" ? "de-CH" : "en-US";
}
