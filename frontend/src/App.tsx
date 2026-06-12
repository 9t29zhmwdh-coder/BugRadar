import React, { useEffect, useState } from "react";
import { Dashboard } from "./components/Dashboard/Dashboard";
import { LogStreamView } from "./components/LogStream/LogStreamView";
import { IncidentList } from "./components/Incidents/IncidentList";
import { IncidentDetail } from "./components/Incidents/IncidentDetail";
import { AnomalyTimeline } from "./components/Timeline/AnomalyTimeline";
import { ConfigView } from "./components/ConfigInspector/ConfigView";
import { SettingsView } from "./components/Settings/SettingsView";
import { useIncidentStore } from "./stores/incidentStore";
import { useLogStore } from "./stores/logStore";
import { useMetricsStore } from "./stores/metricsStore";
import { useSettingsStore } from "./stores/settingsStore";

type View = "dashboard" | "logs" | "incidents" | "timeline" | "config" | "settings";

const NAV: Array<{ id: View; label: string; icon: string }> = [
  { id: "dashboard",  label: "Dashboard",  icon: "◈" },
  { id: "logs",       label: "Logs",       icon: "≡" },
  { id: "incidents",  label: "Incidents",  icon: "⚡" },
  { id: "timeline",   label: "Timeline",   icon: "⌛" },
  { id: "config",     label: "Config",     icon: "⚙" },
  { id: "settings",   label: "Settings",   icon: "✦" },
];

export default function App() {
  const [view, setView] = useState<View>("dashboard");
  const { loadIncidents, subscribeToEvents } = useIncidentStore();
  const { loadSources } = useLogStore();
  const { startPolling, refreshContainers } = useMetricsStore();
  const { load: loadSettings } = useSettingsStore();

  useEffect(() => {
    loadSettings();
    loadIncidents();
    loadSources();
    startPolling(3000);
    refreshContainers();
    subscribeToEvents();
  }, []);

  return (
    <div className="flex h-screen bg-slate-950 text-slate-100 overflow-hidden">
      {/* Sidebar */}
      <nav className="w-14 bg-slate-900 border-r border-slate-800 flex flex-col items-center py-3 gap-1 shrink-0">
        <div className="text-red-500 text-lg font-bold mb-4">⦿</div>
        {NAV.map(item => (
          <button
            key={item.id}
            onClick={() => setView(item.id)}
            title={item.label}
            className={`w-10 h-10 rounded-lg flex items-center justify-center text-base transition-colors ${
              view === item.id
                ? "bg-slate-700 text-slate-100"
                : "text-slate-500 hover:text-slate-300 hover:bg-slate-800"
            }`}
          >
            {item.icon}
          </button>
        ))}
      </nav>

      {/* Main content */}
      <div className="flex-1 overflow-hidden">
        {view === "dashboard" && <Dashboard />}
        {view === "logs" && <LogStreamView />}
        {view === "incidents" && (
          <div className="flex h-full">
            <div className="w-72 border-r border-slate-800 overflow-auto">
              <IncidentList />
            </div>
            <div className="flex-1 overflow-auto">
              <IncidentDetail />
            </div>
          </div>
        )}
        {view === "timeline" && <AnomalyTimeline />}
        {view === "config" && <ConfigView />}
        {view === "settings" && (
          <div className="overflow-auto h-full">
            <SettingsView />
          </div>
        )}
      </div>
    </div>
  );
}
