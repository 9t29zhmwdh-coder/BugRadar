import React, { useState } from "react";
import { api, ConfigInspectionResult } from "../../lib/tauri";
import { ConflictHighlight } from "./ConflictHighlight";

export function ConfigView() {
  const [path, setPath] = useState("");
  const [result, setResult] = useState<ConfigInspectionResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const inspect = async () => {
    if (!path.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const r = await api.inspectConfigFile(path.trim());
      setResult(r);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 space-y-4">
      <div className="text-xs text-slate-500 uppercase tracking-widest">Config Inspector</div>

      <div className="flex gap-2">
        <input
          type="text"
          value={path}
          onChange={e => setPath(e.target.value)}
          placeholder="/path/to/config.yaml"
          className="flex-1 bg-slate-800 border border-slate-600 rounded px-3 py-1.5 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-slate-400 font-mono"
          onKeyDown={e => e.key === "Enter" && inspect()}
        />
        <button
          onClick={inspect}
          disabled={loading}
          className="px-4 py-1.5 bg-slate-700 hover:bg-slate-600 text-sm text-slate-200 rounded transition-colors disabled:opacity-50"
        >
          {loading ? "..." : "Inspect"}
        </button>
      </div>

      {error && <div className="text-sm text-red-400">{error}</div>}

      {result && (
        <div className="space-y-3">
          <div className="flex items-center gap-4 text-xs text-slate-500">
            <span>Format: <span className="text-slate-300">{result.format.toUpperCase()}</span></span>
            <span>Keys: <span className="text-slate-300">{result.key_count}</span></span>
            <span className={result.parsed_ok ? "text-green-400" : "text-red-400"}>
              {result.parsed_ok ? "✓ Valid" : "✗ Parse Error"}
            </span>
          </div>

          {result.issues.length === 0 ? (
            <div className="text-sm text-green-400">No issues found</div>
          ) : (
            <ConflictHighlight issues={result.issues} />
          )}
        </div>
      )}
    </div>
  );
}
