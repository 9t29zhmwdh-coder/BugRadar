import { create } from "zustand";
import { api, events, LogEntry, WatchSource } from "../lib/tauri";

const MAX_LOG_ENTRIES = 2000;

interface LogState {
  sources: WatchSource[];
  logs: Record<string, LogEntry[]>;
  activeSourceId: string | null;
  unsubscribers: Record<string, () => void>;

  loadSources: () => Promise<void>;
  addSource: (source: WatchSource) => Promise<void>;
  removeSource: (sourceId: string) => Promise<void>;
  setActiveSource: (sourceId: string) => void;
  subscribeToLogs: (sourceId: string) => Promise<void>;
}

export const useLogStore = create<LogState>((set, get) => ({
  sources: [],
  logs: {},
  activeSourceId: null,
  unsubscribers: {},

  loadSources: async () => {
    const sources = await api.listWatchSources();
    set({ sources });
    for (const source of sources) {
      if (source.enabled) {
        const existing = get().logs[source.id];
        if (!existing) {
          const recent = await api.getRecentLogs(source.id, 100);
          set(s => ({ logs: { ...s.logs, [source.id]: recent } }));
        }
        await get().subscribeToLogs(source.id);
      }
    }
  },

  addSource: async (source) => {
    await api.watchSource(source);
    set(s => ({ sources: [...s.sources, source] }));
    await get().subscribeToLogs(source.id);
  },

  removeSource: async (sourceId) => {
    await api.stopWatching(sourceId);
    const unsub = get().unsubscribers[sourceId];
    if (unsub) unsub();
    set(s => {
      const { [sourceId]: _, ...rest } = s.logs;
      const { [sourceId]: __, ...unsubs } = s.unsubscribers;
      return {
        sources: s.sources.filter(x => x.id !== sourceId),
        logs: rest,
        unsubscribers: unsubs,
      };
    });
  },

  setActiveSource: (sourceId) => set({ activeSourceId: sourceId }),

  subscribeToLogs: async (sourceId) => {
    const unsub = await events.onLogEntry(sourceId, (entry) => {
      set(s => {
        const current = s.logs[sourceId] ?? [];
        const updated = [entry, ...current].slice(0, MAX_LOG_ENTRIES);
        return { logs: { ...s.logs, [sourceId]: updated } };
      });
    });
    set(s => ({ unsubscribers: { ...s.unsubscribers, [sourceId]: unsub } }));
  },
}));
