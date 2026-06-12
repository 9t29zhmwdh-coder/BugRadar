import React, { useState } from "react";
import type { LogEntry } from "../../lib/tauri";
import { levelColor, fmtDate } from "../../lib/format";
import { StacktraceExpander } from "./StacktraceExpander";

export function LogLine({ entry }: { entry: LogEntry }) {
  const [expanded, setExpanded] = useState(false);
  const hasStack = (entry.stacktrace?.length ?? 0) > 0;

  return (
    <div className={`font-mono text-xs leading-relaxed py-0.5 px-2 hover:bg-slate-800/50 ${hasStack ? "cursor-pointer" : ""}`}
      onClick={() => hasStack && setExpanded(x => !x)}>
      <span className="text-slate-600 select-none mr-2">{fmtDate(entry.timestamp)}</span>
      <span className={`mr-2 font-semibold uppercase w-5 inline-block ${levelColor(entry.level)}`}>
        {entry.level[0].toUpperCase()}
      </span>
      <span className="text-slate-300">{entry.message}</span>
      {hasStack && (
        <span className="ml-2 text-slate-600 text-xs">{expanded ? "▲" : "▼"} stack</span>
      )}
      {expanded && entry.stacktrace && (
        <StacktraceExpander lines={entry.stacktrace} />
      )}
    </div>
  );
}
