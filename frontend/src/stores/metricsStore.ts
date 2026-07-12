import { create } from "zustand";
import { api, events, SystemMetrics, ContainerStatus } from "../lib/tauri";

interface MetricsState {
  metrics: SystemMetrics | null;
  history: SystemMetrics[];
  containers: ContainerStatus[];

  startPolling: (intervalMs?: number) => Promise<void>;
  refreshContainers: () => Promise<void>;
}

export const useMetricsStore = create<MetricsState>((set) => ({
  metrics: null,
  history: [],
  containers: [],

  startPolling: async (intervalMs = 3000) => {
    const initial = await api.getSystemMetrics();
    set({ metrics: initial });

    await api.startMetricsPolling(intervalMs);

    await events.onMetricsSnapshot((snapshot) => {
      set(s => ({
        metrics: snapshot,
        history: [...s.history.slice(-60), snapshot],
      }));
    });
  },

  refreshContainers: async () => {
    try {
      const containers = await api.getContainerStatuses();
      set({ containers });
    } catch {
      // Docker not available: ignore
    }
  },
}));
