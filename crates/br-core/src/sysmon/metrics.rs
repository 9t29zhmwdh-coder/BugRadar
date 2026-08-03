use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks, Networks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub usage_percent: f32,
    pub core_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub total_mb: u64,
    pub used_mb: u64,
    pub available_mb: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSnapshot {
    pub name: String,
    pub mount_point: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub interface: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu: CpuSnapshot,
    pub memory: MemorySnapshot,
    pub disks: Vec<DiskSnapshot>,
    pub networks: Vec<NetworkSnapshot>,
    pub load_avg_1m: f64,
    pub collected_at: chrono::DateTime<chrono::Utc>,
}

pub struct MetricsCollector {
    sys: System,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        let cpu_usage = self.sys.global_cpu_usage();
        let total_mem = self.sys.total_memory() / 1024 / 1024;
        let used_mem = self.sys.used_memory() / 1024 / 1024;
        let avail_mem = self.sys.available_memory() / 1024 / 1024;

        let disks = Disks::new_with_refreshed_list();
        let disk_snapshots: Vec<DiskSnapshot> = disks.iter().map(|d| {
            let total = d.total_space() as f64 / 1_073_741_824.0;
            let avail = d.available_space() as f64 / 1_073_741_824.0;
            let used = total - avail;
            DiskSnapshot {
                name: d.name().to_string_lossy().to_string(),
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_gb: total,
                free_gb: avail,
                usage_percent: if total > 0.0 { (used / total * 100.0) as f32 } else { 0.0 },
            }
        }).collect();

        let networks = Networks::new_with_refreshed_list();
        let net_snapshots: Vec<NetworkSnapshot> = networks.iter()
            .map(|(name, data)| NetworkSnapshot {
                interface: name.clone(),
                bytes_sent: data.total_transmitted(),
                bytes_recv: data.total_received(),
            })
            .collect();

        let load_avg = System::load_average();

        SystemMetrics {
            cpu: CpuSnapshot {
                usage_percent: cpu_usage,
                core_count: self.sys.cpus().len(),
            },
            memory: MemorySnapshot {
                total_mb: total_mem,
                used_mb: used_mem,
                available_mb: avail_mem,
                usage_percent: if total_mem > 0 { (used_mem as f32 / total_mem as f32) * 100.0 } else { 0.0 },
            },
            disks: disk_snapshots,
            networks: net_snapshots,
            load_avg_1m: load_avg.one,
            collected_at: chrono::Utc::now(),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Haelt die Einheiten fest, in denen `sysinfo` meldet.
    ///
    /// `collect` teilt den Speicher zweimal durch 1024 und nennt das Ergebnis
    /// MB, den Plattenplatz einmal durch 1_073_741_824 fuer GB. Wechselt eine
    /// neue Version auf Kilobyte, sind alle Werte um Faktor 1024 zu klein: die
    /// Oberflaeche zeigt dann acht MB Arbeitsspeicher statt acht GB, und jede
    /// Schwelle, die auf diesen Zahlen sitzt, loest nie mehr aus. Ein Compiler
    /// bemerkt davon nichts.
    #[test]
    fn die_gemeldeten_einheiten_bleiben_wie_sie_sind() {
        let mut sammler = MetricsCollector::new();
        let werte = sammler.collect();

        assert!(
            werte.memory.total_mb >= 1024,
            "Gesamtspeicher {} MB ist zu klein. Bei Kilobyte statt Bytes laege \
             der Wert etwa um Faktor 1024 darunter.",
            werte.memory.total_mb
        );
        assert!(
            werte.memory.total_mb < 100_000_000,
            "Gesamtspeicher {} MB ist unplausibel gross",
            werte.memory.total_mb
        );
        assert!(werte.memory.used_mb <= werte.memory.total_mb);

        assert!(
            (0.0..=100.0).contains(&werte.cpu.usage_percent),
            "CPU-Auslastung {} liegt ausserhalb von 0 bis 100",
            werte.cpu.usage_percent
        );
        assert!(werte.cpu.core_count > 0);

        for platte in &werte.disks {
            assert!(
                (0.0..=100.0).contains(&platte.usage_percent),
                "Belegung {} von {} liegt ausserhalb von 0 bis 100",
                platte.usage_percent,
                platte.name
            );
        }
    }
}
