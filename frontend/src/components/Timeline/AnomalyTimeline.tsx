import React from "react";
import {
  AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer
} from "recharts";
import { useMetricsStore } from "../../stores/metricsStore";
import { format } from "date-fns";
import { useT } from "../../lib/i18n";

export function AnomalyTimeline() {
  const { history } = useMetricsStore();
  const t = useT();

  const data = history.map(m => ({
    time: format(new Date(m.collected_at), "HH:mm:ss"),
    cpu: parseFloat(m.cpu.usage_percent.toFixed(1)),
    mem: parseFloat(m.memory.usage_percent.toFixed(1)),
  }));

  return (
    <div className="h-full p-4">
      <div className="text-xs text-slate-500 uppercase tracking-widest mb-4">{t("timeline.systemTimeline")}</div>
      {data.length < 2 ? (
        <div className="flex items-center justify-center h-32 text-slate-600 text-sm">
          {t("timeline.collectingData")}
        </div>
      ) : (
        <ResponsiveContainer width="100%" height={160}>
          <AreaChart data={data} margin={{ top: 4, right: 8, bottom: 0, left: 0 }}>
            <defs>
              <linearGradient id="cpuGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
              </linearGradient>
              <linearGradient id="memGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#8b5cf6" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#8b5cf6" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" />
            <XAxis dataKey="time" tick={{ fontSize: 10, fill: "#475569" }} interval="preserveStartEnd" />
            <YAxis domain={[0, 100]} tick={{ fontSize: 10, fill: "#475569" }} unit="%" />
            <Tooltip
              contentStyle={{ background: "#0f172a", border: "1px solid #334155", borderRadius: 8, fontSize: 12 }}
              labelStyle={{ color: "#94a3b8" }}
            />
            <Area type="monotone" dataKey="cpu" stroke="#3b82f6" fill="url(#cpuGrad)" name="CPU" />
            <Area type="monotone" dataKey="mem" stroke="#8b5cf6" fill="url(#memGrad)" name="Memory" />
          </AreaChart>
        </ResponsiveContainer>
      )}
    </div>
  );
}
