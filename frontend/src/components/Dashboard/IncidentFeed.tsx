import React from "react";
import { useIncidentStore } from "../../stores/incidentStore";
import { severityColor, statusBadge, timeAgo } from "../../lib/format";
import { useT } from "../../lib/i18n";
import type { Incident } from "../../lib/tauri";

export function IncidentFeed() {
  const { incidents, selectIncident, loading } = useIncidentStore();
  const t = useT();
  const open = incidents.filter(i => i.status === "open" || i.status === "investigating").slice(0, 5);

  if (loading) return <div className="text-slate-500 text-sm p-4">{t("dashboard.loadingIncidents")}</div>;
  if (open.length === 0) return (
    <div className="p-4 text-sm text-green-400 flex items-center gap-2">
      <span className="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
      {t("dashboard.allClear")}
    </div>
  );

  return (
    <div className="space-y-2 p-4">
      <div className="text-xs text-slate-500 uppercase tracking-widest mb-3">{t("dashboard.activeIncidents")}</div>
      {open.map(incident => (
        <IncidentRow key={incident.id} incident={incident} onClick={() => selectIncident(incident.id)} />
      ))}
    </div>
  );
}

function IncidentRow({ incident, onClick }: { incident: Incident; onClick: () => void }) {
  const t = useT();
  return (
    <button
      onClick={onClick}
      className="w-full text-left bg-slate-800 hover:bg-slate-700 rounded-lg p-3 border border-slate-700 transition-colors"
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <div className="text-sm font-medium text-slate-100 truncate">{incident.title}</div>
          <div className="text-xs text-slate-500 mt-0.5">
            {incident.anomaly_ids.length} {t("dashboard.anomalies")} · {timeAgo(incident.last_seen)}
          </div>
        </div>
        <div className="flex items-center gap-1.5 shrink-0">
          <span className={`text-xs px-1.5 py-0.5 rounded font-medium ${severityColor(incident.severity)}`}>
            {incident.severity}
          </span>
          <span className={`text-xs px-1.5 py-0.5 rounded ${statusBadge(incident.status)}`}>
            {incident.status}
          </span>
        </div>
      </div>
    </button>
  );
}
