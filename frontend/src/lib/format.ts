import { formatDistanceToNow, format } from "date-fns";
import type { Severity, IncidentStatus, LogLevel } from "./tauri";

export function timeAgo(iso: string): string {
  return formatDistanceToNow(new Date(iso), { addSuffix: true });
}

export function fmtDate(iso: string): string {
  return format(new Date(iso), "MMM d, HH:mm:ss");
}

export function severityColor(s: Severity): string {
  switch (s) {
    case "critical": return "text-red-400 bg-red-950";
    case "high":     return "text-orange-400 bg-orange-950";
    case "medium":   return "text-yellow-400 bg-yellow-950";
    default:         return "text-blue-400 bg-blue-950";
  }
}

export function levelColor(l: LogLevel): string {
  switch (l) {
    case "fatal":   return "text-red-300";
    case "error":   return "text-red-400";
    case "warn":    return "text-yellow-400";
    case "info":    return "text-green-400";
    case "debug":   return "text-blue-400";
    case "trace":   return "text-slate-400";
    default:        return "text-slate-300";
  }
}

export function statusBadge(s: IncidentStatus): string {
  switch (s) {
    case "open":          return "bg-red-900 text-red-300";
    case "investigating": return "bg-orange-900 text-orange-300";
    case "resolved":      return "bg-green-900 text-green-300";
    case "suppressed":    return "bg-slate-700 text-slate-300";
  }
}
