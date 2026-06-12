import React from "react";
import { MetricsBar } from "./MetricsBar";
import { IncidentFeed } from "./IncidentFeed";

export function Dashboard() {
  return (
    <div className="flex flex-col h-full">
      <MetricsBar />
      <div className="flex-1 overflow-auto">
        <IncidentFeed />
      </div>
    </div>
  );
}
