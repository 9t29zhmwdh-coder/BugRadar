import React from "react";
import { useIncidentStore } from "../../stores/incidentStore";
import { severityColor, statusBadge, timeAgo } from "../../lib/format";
import { useT } from "../../lib/i18n";
import type { Incident } from "../../lib/tauri";

export function IncidentList() {
  const { incidents, selected, selectIncident, loading } = useIncidentStore();
  const t = useT();

  return (
    <div className="h-full flex flex-col">
      <div className="p-3 border-b border-slate-700 flex items-center justify-between">
        <div className="text-sm font-medium text-slate-300">{t("incidents.title")}</div>
        <div className="text-xs text-slate-500">{incidents.length} {t("incidents.total")}</div>
      </div>
      <div className="flex-1 overflow-auto">
        {incidents.map(incident => (
          <IncidentRow
            key={incident.id}
            incident={incident}
            isSelected={selected?.id === incident.id}
            onClick={() => selectIncident(incident.id)}
          />
        ))}
      </div>
    </div>
  );
}

function IncidentRow({
  incident, isSelected, onClick
}: { incident: Incident; isSelected: boolean; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      className={`w-full text-left px-3 py-2.5 border-b border-slate-800 hover:bg-slate-800 transition-colors ${
        isSelected ? "bg-slate-800 border-l-2 border-l-blue-500" : ""
      }`}
    >
      <div className="flex items-center gap-2 mb-1">
        <span className={`text-xs px-1.5 py-0.5 rounded font-medium ${severityColor(incident.severity)}`}>
          {incident.severity[0].toUpperCase()}
        </span>
        <span className="text-sm text-slate-200 truncate">{incident.title}</span>
      </div>
      <div className="flex items-center gap-2">
        <span className={`text-xs px-1.5 py-0.5 rounded ${statusBadge(incident.status)}`}>
          {incident.status}
        </span>
        <span className="text-xs text-slate-600">{timeAgo(incident.last_seen)}</span>
        {incident.ai_analysis_id && (
          <span className="text-xs text-blue-500">✦ AI</span>
        )}
      </div>
    </button>
  );
}
