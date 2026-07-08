import React, { useState } from "react";
import { useIncidentStore } from "../../stores/incidentStore";
import { AiReportCard } from "./AiReportCard";
import { severityColor, statusBadge, fmtDate, timeAgo } from "../../lib/format";
import { useT } from "../../lib/i18n";
import type { IncidentStatus } from "../../lib/tauri";

const STATUSES: IncidentStatus[] = ["open", "investigating", "resolved", "suppressed"];

export function IncidentDetail() {
  const { selected, reports, aiLoading, updateStatus, addNote, triggerAi } = useIncidentStore();
  const [note, setNote] = useState("");
  const t = useT();

  if (!selected) return (
    <div className="flex items-center justify-center h-full text-slate-600 text-sm">
      {t("incidents.selectIncident")}
    </div>
  );

  const report = reports[selected.id];
  const isAiLoading = aiLoading[selected.id] ?? false;

  return (
    <div className="h-full overflow-auto p-4 space-y-4">
      <div className="flex items-start justify-between gap-2">
        <div>
          <h2 className="text-base font-semibold text-slate-100">{selected.title}</h2>
          <div className="text-xs text-slate-500 mt-1">
            {t("incidents.firstSeen")} {fmtDate(selected.first_seen)} · {t("incidents.lastSeen")} {timeAgo(selected.last_seen)}
          </div>
        </div>
        <div className="flex items-center gap-2 shrink-0">
          <span className={`text-xs px-2 py-1 rounded font-medium ${severityColor(selected.severity)}`}>
            {selected.severity}
          </span>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <span className="text-xs text-slate-500">{t("incidents.status")}</span>
        {STATUSES.map(s => (
          <button
            key={s}
            onClick={() => updateStatus(selected.id, s)}
            className={`text-xs px-2 py-0.5 rounded border transition-colors ${
              selected.status === s
                ? "border-slate-500 " + statusBadge(s)
                : "border-slate-700 text-slate-500 hover:text-slate-300"
            }`}
          >
            {s}
          </button>
        ))}
      </div>

      <div className="text-xs text-slate-500">
        {selected.anomaly_ids.length} {t("incidents.anomalies")} · {selected.event_count} {t("incidents.events")} ·
        {t("incidents.sources")} {selected.source_ids.join(", ")}
      </div>

      {/* AI Analysis */}
      <div className="border-t border-slate-700 pt-4">
        {report ? (
          <AiReportCard report={report} />
        ) : (
          <div className="flex items-center gap-3">
            <button
              onClick={() => triggerAi(selected.id)}
              disabled={isAiLoading}
              className="flex items-center gap-2 px-3 py-1.5 bg-blue-700 hover:bg-blue-600 disabled:opacity-50 text-white text-sm rounded-lg transition-colors"
            >
              {isAiLoading ? (
                <><span className="animate-spin">⟳</span> {t("incidents.analyzing")}</>
              ) : (
                <><span>✦</span> {t("incidents.runAiAnalysis")}</>
              )}
            </button>
            <span className="text-xs text-slate-500">
              {t("incidents.triggersInfo")}
            </span>
          </div>
        )}
      </div>

      {/* Notes */}
      {selected.notes.length > 0 && (
        <div className="border-t border-slate-700 pt-4 space-y-2">
          <div className="text-xs text-slate-500 uppercase tracking-widest">{t("incidents.notes")}</div>
          {selected.notes.map(n => (
            <div key={n.id} className="bg-slate-800 rounded p-2 text-sm text-slate-300">
              <div className="text-xs text-slate-600 mb-1">{fmtDate(n.created_at)}</div>
              {n.text}
            </div>
          ))}
        </div>
      )}

      <div className="border-t border-slate-700 pt-4 flex gap-2">
        <input
          type="text"
          value={note}
          onChange={e => setNote(e.target.value)}
          placeholder={t("incidents.addNotePlaceholder")}
          className="flex-1 bg-slate-800 border border-slate-600 rounded px-3 py-1.5 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-slate-400"
          onKeyDown={e => { if (e.key === "Enter" && note.trim()) { addNote(selected.id, note); setNote(""); } }}
        />
        <button
          onClick={() => { if (note.trim()) { addNote(selected.id, note); setNote(""); } }}
          className="px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-sm text-slate-200 rounded transition-colors"
        >
          {t("incidents.add")}
        </button>
      </div>
    </div>
  );
}
