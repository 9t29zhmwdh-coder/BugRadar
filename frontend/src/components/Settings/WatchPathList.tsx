import { useState } from "react";
import { useLogStore } from "../../stores/logStore";
import { WatchSource, WatchSourceKind } from "../../lib/tauri";
import { useT } from "../../lib/i18n";

type Kind = "file" | "container" | "all";

function describeKind(kind: WatchSourceKind): string {
  if (kind === "docker_all_containers") return "docker: *";
  if ("file_path" in kind) return kind.file_path.path;
  return `docker: ${kind.docker_container.container_name}`;
}

function kindFor(kind: Kind, value: string): WatchSourceKind {
  if (kind === "file") return { file_path: { path: value } };
  if (kind === "container") return { docker_container: { container_id: value, container_name: value } };
  return "docker_all_containers";
}

export function WatchPathList() {
  const { sources, addSource, removeSource } = useLogStore();
  const [newPath, setNewPath] = useState("");
  const [label, setLabel] = useState("");
  const [kind, setKind] = useState<Kind>("file");
  const [error, setError] = useState<string | null>(null);
  const t = useT();

  const add = async () => {
    const value = newPath.trim();
    if (kind !== "all" && !value) return;
    const fallbackLabel = kind === "all" ? t("settings.kindAllContainers") : value.split("/").pop() || value;
    const source: WatchSource = {
      id: crypto.randomUUID(),
      label: label || fallbackLabel,
      kind: kindFor(kind, value),
      parser_id: "auto",
      enabled: true,
      created_at: new Date().toISOString(),
    };
    setError(null);
    try {
      await addSource(source);
      setNewPath("");
      setLabel("");
    } catch (e) {
      // Used to be an unhandled rejection: the click did nothing visible.
      setError(`${t("settings.watchFailed")} ${String(e)}`);
    }
  };

  return (
    <div className="space-y-3">
      <div className="space-y-1.5">
        {sources.map(s => (
          <div key={s.id} className="flex items-center justify-between bg-slate-800 rounded px-3 py-2">
            <div>
              <div className="text-sm text-slate-200">{s.label}</div>
              <div className="text-xs text-slate-500 font-mono">
                {describeKind(s.kind)}
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
          <select
            value={kind}
            onChange={e => setKind(e.target.value as Kind)}
            className="bg-slate-800 border border-slate-700 rounded px-2 py-1.5 text-sm text-slate-200"
          >
            <option value="file">{t("settings.kindFile")}</option>
            <option value="container">{t("settings.kindContainer")}</option>
            <option value="all">{t("settings.kindAllContainers")}</option>
          </select>
          <input
            type="text"
            value={newPath}
            onChange={e => setNewPath(e.target.value)}
            disabled={kind === "all"}
            placeholder={kind === "file" ? "/var/log/app.log" : kind === "container" ? t("settings.containerPlaceholder") : ""}
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
        {error && <p className="text-xs text-red-400">{error}</p>}
      </div>
    </div>
  );
}
