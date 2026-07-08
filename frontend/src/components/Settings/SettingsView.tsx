import React, { useEffect, useState } from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import { WatchPathList } from "./WatchPathList";
import { useT } from "../../lib/i18n";
import type { AppSettings } from "../../lib/tauri";

export function SettingsView() {
  const { settings, hasApiKey, load, save, saveApiKey } = useSettingsStore();
  const [form, setForm] = useState<AppSettings | null>(null);
  const [apiKeyInput, setApiKeyInput] = useState("");
  const [saved, setSaved] = useState(false);
  const t = useT();

  useEffect(() => { if (settings) setForm(settings); }, [settings]);

  const handleSave = async () => {
    if (!form) return;
    await save(form);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  const handleSaveKey = async () => {
    if (!apiKeyInput.trim()) return;
    await saveApiKey(apiKeyInput.trim());
    setApiKeyInput("");
  };

  if (!form) return <div className="p-4 text-slate-500 text-sm">{t("settings.loading")}</div>;

  return (
    <div className="p-4 space-y-6 max-w-lg">
      <div className="text-xs text-slate-500 uppercase tracking-widest">{t("settings.title")}</div>

      <Section title={t("settings.aiProvider")}>
        <div className="space-y-3">
          <div>
            <label className="text-xs text-slate-400 block mb-1">{t("settings.provider")}</label>
            <select
              value={form.ai_provider}
              onChange={e => setForm({ ...form, ai_provider: e.target.value })}
              className="bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 focus:outline-none"
            >
              <option value="claude">Claude (Anthropic)</option>
              <option value="ollama">Ollama (Local)</option>
            </select>
          </div>

          {form.ai_provider === "claude" && (
            <div>
              <label className="text-xs text-slate-400 block mb-1">
                {t("settings.apiKey")} {hasApiKey ? `✓ ${t("settings.saved")}` : `(${t("settings.notSet")})`}
              </label>
              <div className="flex gap-2">
                <input
                  type="password"
                  value={apiKeyInput}
                  onChange={e => setApiKeyInput(e.target.value)}
                  placeholder="sk-ant-..."
                  className="flex-1 bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 placeholder-slate-600 focus:outline-none font-mono"
                />
                <button onClick={handleSaveKey} className="px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-sm text-slate-200 rounded">
                  {t("settings.save")}
                </button>
              </div>
            </div>
          )}

          {form.ai_provider === "ollama" && (
            <div className="space-y-2">
              <div>
                <label className="text-xs text-slate-400 block mb-1">{t("settings.ollamaHost")}</label>
                <input
                  type="text"
                  value={form.ollama_host}
                  onChange={e => setForm({ ...form, ollama_host: e.target.value })}
                  className="w-full bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 focus:outline-none font-mono"
                />
              </div>
              <div>
                <label className="text-xs text-slate-400 block mb-1">{t("settings.model")}</label>
                <input
                  type="text"
                  value={form.ollama_model}
                  onChange={e => setForm({ ...form, ollama_model: e.target.value })}
                  className="w-full bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 focus:outline-none font-mono"
                />
              </div>
            </div>
          )}
        </div>
      </Section>

      <Section title={t("settings.anomalyThresholds")}>
        <div className="space-y-2">
          <Slider label={t("settings.errorSpikeThreshold")} value={form.anomaly_config.error_spike_threshold}
            min={1} max={10} step={0.5}
            onChange={v => setForm({ ...form, anomaly_config: { ...form.anomaly_config, error_spike_threshold: v } })} />
          <Slider label={t("settings.latencyJumpThreshold")} value={form.anomaly_config.latency_jump_threshold}
            min={1} max={10} step={0.5}
            onChange={v => setForm({ ...form, anomaly_config: { ...form.anomaly_config, latency_jump_threshold: v } })} />
          <Slider label={t("settings.windowSeconds")} value={form.anomaly_config.window_seconds}
            min={60} max={3600} step={60}
            onChange={v => setForm({ ...form, anomaly_config: { ...form.anomaly_config, window_seconds: v } })} />
        </div>
      </Section>

      <Section title={t("settings.watchSources")}>
        <WatchPathList />
      </Section>

      <button
        onClick={handleSave}
        className="px-4 py-1.5 bg-blue-700 hover:bg-blue-600 text-sm text-white rounded transition-colors"
      >
        {saved ? `${t("settings.savedCheck")} ✓` : t("settings.saveSettings")}
      </button>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <div className="text-xs text-slate-500 uppercase tracking-widest mb-3">{title}</div>
      {children}
    </div>
  );
}

function Slider({ label, value, min, max, step, onChange }: {
  label: string; value: number; min: number; max: number; step: number;
  onChange: (v: number) => void;
}) {
  return (
    <div className="flex items-center gap-3">
      <span className="text-xs text-slate-400 w-48">{label}</span>
      <input type="range" min={min} max={max} step={step} value={value}
        onChange={e => onChange(Number(e.target.value))}
        className="flex-1 accent-blue-500" />
      <span className="text-xs text-slate-300 font-mono w-10 text-right">{value}</span>
    </div>
  );
}
