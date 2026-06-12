import React from "react";
import type { DiagnosticReport } from "../../lib/tauri";

export function AiReportCard({ report }: { report: DiagnosticReport }) {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="text-sm font-semibold text-slate-200">AI Diagnostic Report</div>
        <div className="flex items-center gap-2 text-xs text-slate-500">
          <span>{report.ai_provider} / {report.model}</span>
          <span className="px-1.5 py-0.5 bg-slate-700 rounded">
            {Math.round(report.confidence * 100)}% confidence
          </span>
        </div>
      </div>

      <Section title="Summary">
        <p className="text-slate-300 text-sm leading-relaxed">{report.summary}</p>
      </Section>

      <Section title="Root Cause">
        <p className="text-amber-300 text-sm leading-relaxed">{report.root_cause}</p>
      </Section>

      {report.contributing_factors.length > 0 && (
        <Section title="Contributing Factors">
          <ul className="space-y-1">
            {report.contributing_factors.map((f, i) => (
              <li key={i} className="text-sm text-slate-400 flex gap-2">
                <span className="text-slate-600 shrink-0">·</span>
                {f}
              </li>
            ))}
          </ul>
        </Section>
      )}

      {report.fix_suggestions.length > 0 && (
        <Section title="Fix Suggestions">
          <div className="space-y-3">
            {report.fix_suggestions.sort((a, b) => a.priority - b.priority).map((fix, i) => (
              <div key={i} className="bg-slate-800 rounded-lg p-3">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-xs bg-blue-900 text-blue-300 px-1.5 py-0.5 rounded">
                    Priority {fix.priority}
                  </span>
                  <span className="text-sm font-medium text-slate-200">{fix.title}</span>
                </div>
                <p className="text-xs text-slate-400 mb-2">{fix.description}</p>
                {fix.command && (
                  <code className="block text-xs bg-slate-900 text-green-400 px-3 py-2 rounded font-mono">
                    $ {fix.command}
                  </code>
                )}
                {fix.code_snippet && (
                  <div className="mt-2">
                    <div className="text-xs text-slate-500 mb-1">
                      {fix.code_snippet.filename ?? fix.code_snippet.language}
                    </div>
                    <pre className="text-xs bg-slate-900 text-slate-300 px-3 py-2 rounded overflow-auto max-h-48">
                      {fix.code_snippet.content}
                    </pre>
                  </div>
                )}
              </div>
            ))}
          </div>
        </Section>
      )}

      {report.tokens_used && (
        <div className="text-xs text-slate-600 text-right">{report.tokens_used.toLocaleString()} tokens used</div>
      )}
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <div className="text-xs text-slate-500 uppercase tracking-widest mb-1.5">{title}</div>
      {children}
    </div>
  );
}
