import React, { useRef, useEffect } from "react";
import { useLogStore } from "../../stores/logStore";
import { LogLine } from "./LogLine";
import { useT } from "../../lib/i18n";

export function LogStreamView() {
  const { logs, activeSourceId, sources } = useLogStore();
  const bottomRef = useRef<HTMLDivElement>(null);
  const entries = activeSourceId ? (logs[activeSourceId] ?? []) : [];
  const t = useT();

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [entries.length]);

  return (
    <div className="flex flex-col h-full bg-slate-950">
      <div className="px-3 py-1.5 border-b border-slate-700 flex items-center gap-2 text-xs">
        <span className="text-slate-500">{t("logStream.source")}</span>
        {sources.map(s => (
          <button
            key={s.id}
            onClick={() => useLogStore.getState().setActiveSource(s.id)}
            className={`px-2 py-0.5 rounded text-xs transition-colors ${
              activeSourceId === s.id
                ? "bg-slate-600 text-slate-100"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            {s.label}
          </button>
        ))}
      </div>

      <div className="flex-1 overflow-auto">
        {entries.length === 0 ? (
          <div className="flex items-center justify-center h-full text-slate-600 text-sm">
            {activeSourceId ? t("logStream.noLogEntries") : t("logStream.selectSource")}
          </div>
        ) : (
          <>
            {[...entries].reverse().map(entry => (
              <LogLine key={entry.id} entry={entry} />
            ))}
            <div ref={bottomRef} />
          </>
        )}
      </div>
    </div>
  );
}
