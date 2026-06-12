import React from "react";
import { useMetricsStore } from "../../stores/metricsStore";

export function MetricsBar() {
  const { metrics } = useMetricsStore();

  if (!metrics) return (
    <div className="h-12 bg-slate-900 border-b border-slate-700 flex items-center px-4 text-slate-500 text-sm">
      Collecting metrics...
    </div>
  );

  const cpuColor = metrics.cpu.usage_percent > 80 ? "text-red-400" : metrics.cpu.usage_percent > 60 ? "text-yellow-400" : "text-green-400";
  const memColor = metrics.memory.usage_percent > 85 ? "text-red-400" : metrics.memory.usage_percent > 70 ? "text-yellow-400" : "text-green-400";

  return (
    <div className="h-12 bg-slate-900 border-b border-slate-700 flex items-center px-4 gap-6 text-sm">
      <MetricItem label="CPU" value={`${metrics.cpu.usage_percent.toFixed(1)}%`} color={cpuColor} />
      <MetricItem label="RAM" value={`${metrics.memory.used_mb.toLocaleString()} / ${metrics.memory.total_mb.toLocaleString()} MB`} color={memColor} />
      <MetricItem label="Load" value={metrics.load_avg_1m.toFixed(2)} color="text-slate-300" />
      {metrics.disks.slice(0, 2).map(d => (
        <MetricItem key={d.mount_point} label={`Disk ${d.mount_point}`} value={`${d.usage_percent.toFixed(0)}%`}
          color={d.usage_percent > 90 ? "text-red-400" : "text-slate-300"} />
      ))}
    </div>
  );
}

function MetricItem({ label, value, color }: { label: string; value: string; color: string }) {
  return (
    <div className="flex items-center gap-1.5">
      <span className="text-slate-500">{label}</span>
      <span className={`font-mono font-medium ${color}`}>{value}</span>
    </div>
  );
}
