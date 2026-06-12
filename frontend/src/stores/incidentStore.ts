import { create } from "zustand";
import { api, events, Incident, IncidentStatus, DiagnosticReport } from "../lib/tauri";

interface IncidentState {
  incidents: Incident[];
  selected: Incident | null;
  reports: Record<string, DiagnosticReport>;
  loading: boolean;
  aiLoading: Record<string, boolean>;

  loadIncidents: () => Promise<void>;
  selectIncident: (id: string) => Promise<void>;
  updateStatus: (id: string, status: IncidentStatus) => Promise<void>;
  addNote: (id: string, note: string) => Promise<void>;
  triggerAi: (incidentId: string) => Promise<void>;
  subscribeToEvents: () => Promise<() => void>;
}

export const useIncidentStore = create<IncidentState>((set, get) => ({
  incidents: [],
  selected: null,
  reports: {},
  loading: false,
  aiLoading: {},

  loadIncidents: async () => {
    set({ loading: true });
    const incidents = await api.listIncidents();
    set({ incidents, loading: false });
  },

  selectIncident: async (id) => {
    const incident = get().incidents.find(i => i.id === id) ?? await api.getIncident(id) ?? null;
    set({ selected: incident });

    if (incident?.ai_analysis_id) {
      const report = await api.getDiagnosticReport(incident.ai_analysis_id);
      if (report) set(s => ({ reports: { ...s.reports, [incident.id]: report } }));
    }
  },

  updateStatus: async (id, status) => {
    await api.updateIncidentStatus(id, status);
    set(s => ({
      incidents: s.incidents.map(i => i.id === id ? { ...i, status } : i),
      selected: s.selected?.id === id ? { ...s.selected, status } : s.selected,
    }));
  },

  addNote: async (id, note) => {
    await api.addIncidentNote(id, note);
    await get().selectIncident(id);
  },

  triggerAi: async (incidentId) => {
    set(s => ({ aiLoading: { ...s.aiLoading, [incidentId]: true } }));
    try {
      const reportId = await api.triggerAiAnalysis(incidentId);
      const report = await api.getDiagnosticReport(reportId);
      if (report) {
        set(s => ({
          reports: { ...s.reports, [incidentId]: report },
          incidents: s.incidents.map(i => i.id === incidentId ? { ...i, ai_analysis_id: reportId } : i),
        }));
      }
    } finally {
      set(s => ({ aiLoading: { ...s.aiLoading, [incidentId]: false } }));
    }
  },

  subscribeToEvents: async () => {
    const unsub1 = await events.onIncidentCreated((incident) => {
      set(s => ({ incidents: [incident, ...s.incidents] }));
    });

    return () => {
      unsub1();
    };
  },
}));
