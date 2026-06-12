import React from "react";

export function StacktraceExpander({ lines }: { lines: string[] }) {
  return (
    <div className="mt-1 ml-4 pl-3 border-l border-slate-700 text-slate-500">
      {lines.map((line, i) => (
        <div key={i} className="whitespace-pre-wrap break-all">{line}</div>
      ))}
    </div>
  );
}
