import React from "react";

interface HeatCell {
  label: string;
  value: number;
  max: number;
}

export function HeatmapGrid({ cells, title }: { cells: HeatCell[]; title: string }) {
  return (
    <div className="p-4">
      <div className="text-xs text-slate-500 uppercase tracking-widest mb-3">{title}</div>
      <div className="grid grid-cols-12 gap-0.5">
        {cells.map((cell, i) => {
          const intensity = cell.max > 0 ? cell.value / cell.max : 0;
          const bg = intensity > 0.8 ? "bg-red-600" :
                     intensity > 0.6 ? "bg-orange-600" :
                     intensity > 0.4 ? "bg-yellow-600" :
                     intensity > 0.1 ? "bg-green-800" : "bg-slate-800";
          return (
            <div
              key={i}
              title={`${cell.label}: ${cell.value}`}
              className={`h-4 rounded-sm ${bg} cursor-default`}
            />
          );
        })}
      </div>
    </div>
  );
}
