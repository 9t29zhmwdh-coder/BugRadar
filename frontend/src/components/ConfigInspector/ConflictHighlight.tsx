import React from "react";

interface Issue {
  severity: string;
  key: string;
  message: string;
  suggestion?: string;
}

export function ConflictHighlight({ issues }: { issues: Issue[] }) {
  return (
    <div className="space-y-2">
      {issues.map((issue, i) => {
        const color = issue.severity === "error" ? "border-red-700 bg-red-950/30"
          : issue.severity === "warning" ? "border-yellow-700 bg-yellow-950/30"
          : "border-slate-700 bg-slate-800/30";

        const badge = issue.severity === "error" ? "bg-red-900 text-red-300"
          : issue.severity === "warning" ? "bg-yellow-900 text-yellow-300"
          : "bg-slate-700 text-slate-300";

        return (
          <div key={i} className={`border rounded-lg p-3 ${color}`}>
            <div className="flex items-center gap-2 mb-1">
              <span className={`text-xs px-1.5 py-0.5 rounded ${badge}`}>{issue.severity}</span>
              <code className="text-xs text-slate-300 font-mono">{issue.key}</code>
            </div>
            <div className="text-sm text-slate-300">{issue.message}</div>
            {issue.suggestion && (
              <div className="text-xs text-slate-500 mt-1">→ {issue.suggestion}</div>
            )}
          </div>
        );
      })}
    </div>
  );
}
