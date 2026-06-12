import { create } from "zustand";
import { api, AppSettings } from "../lib/tauri";

interface SettingsState {
  settings: AppSettings | null;
  hasApiKey: boolean;
  load: () => Promise<void>;
  save: (settings: AppSettings) => Promise<void>;
  saveApiKey: (key: string) => Promise<void>;
}

const DEFAULT_SETTINGS: AppSettings = {
  ai_provider: "claude",
  ollama_host: "http://localhost:11434",
  ollama_model: "llama3.2",
  log_retention_days: 7,
  anomaly_config: {
    window_seconds: 300,
    error_spike_threshold: 3.0,
    latency_jump_threshold: 2.5,
    memory_growth_threshold_mb_per_min: 5.0,
    min_samples_for_baseline: 10,
    incident_correlation_window_seconds: 120,
  },
};

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: null,
  hasApiKey: false,

  load: async () => {
    const [settings, hasKey] = await Promise.all([api.getSettings(), api.hasApiKey()]);
    set({ settings: settings ?? DEFAULT_SETTINGS, hasApiKey: hasKey });
  },

  save: async (settings) => {
    await api.saveSettings(settings);
    set({ settings });
  },

  saveApiKey: async (key) => {
    await api.saveApiKey(key);
    set({ hasApiKey: !!key });
  },
}));
