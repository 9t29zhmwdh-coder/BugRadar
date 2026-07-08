import React, { useState } from "react";
import { useLogStore } from "../../stores/logStore";
import { WatchSource } from "../../lib/tauri";
import { v4 as uuidv4 } from "../../lib/uuid";
import { useT } from "../../lib/i18n";

export function WatchPathList() {
  const { sources, addSource, removeSource } = useLogStore();
  const [newPath, setNewPath] = useState("");
  const [label, setLabel] = useState("");
  const t = useT();

  const add = async () => {
    if (!newPath.trim()) return;
    const source: WatchSource = {
      id: crypto.randomUUID(),
      label: label || newPath.split("/").pop() || newPath,
      kind: { FilePath: { path: newPath.trim() } },
      parser_id: "auto",
      enabled: true,
      created_at: new Date().toISOString(),
    };
    await addSource(source);
    setNewPath("");
    setLabel("");
  };

  return (
    <div className="space-y-3">
      <div className="space-y-1.5">
        {sources.map(s => (
          <div key={s.id} className="flex items-center justify-between bg-slate-800 rounded px-3 py-2">
            <div>
              <div className="text-sm text-slate-200">{s.label}</div>
              <div className="text-xs text-slate-500 font-mono">
                {JSON.stringify(s.kind)}
              </div>
            </div>
            <button
              onClick={() => removeSource(s.id)}
              className="text-slate-600 hover:text-red-400 text-xs transition-colors"
            >
              {t("settings.remove")}
            </button>
          </div>
        ))}
      </div>

      <div className="space-y-2 pt-2 border-t border-slate-700">
        <input
          type="text"
          value={label}
          onChange={e => setLabel(e.target.value)}
          placeholder={t("settings.labelOptional")}
          className="w-full bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-slate-500"
        />
        <div className="flex gap-2">
          <input
            type="text"
            value={newPath}
            onChange={e => setNewPath(e.target.value)}
            placeholder="/var/log/app.log"
            className="flex-1 bg-slate-800 border border-slate-700 rounded px-3 py-1.5 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-slate-500 font-mono"
            onKeyDown={e => e.key === "Enter" && add()}
          />
          <button
            onClick={add}
            className="px-3 py-1.5 bg-blue-700 hover:bg-blue-600 text-sm text-white rounded transition-colors"
          >
            {t("settings.watch")}
          </button>
        </div>
      </div>
    </div>
  );
}
