use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bugradar", about = "AI-powered log diagnostics and monitoring", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Watch a log file in real-time and detect anomalies
    Watch {
        /// Path to the log file
        path: PathBuf,
        /// Parser to use (auto, plaintext, json, nginx, docker)
        #[arg(short, long, default_value = "auto")]
        parser: String,
    },
    /// List incidents
    Incidents {
        /// Show only open incidents
        #[arg(long)]
        open: bool,
        /// Maximum number of incidents to show
        #[arg(short, long, default_value_t = 20)]
        limit: usize,
    },
    /// Show AI diagnostic report for an incident
    Report {
        /// Incident ID
        incident_id: String,
    },
    /// Inspect a config file (YAML, JSON, TOML)
    Inspect {
        /// Path to config file
        path: PathBuf,
    },
    /// Show system metrics snapshot
    Metrics,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Command::Watch { path, parser } => {
            println!("Watching {} with parser '{}'", path.display(), parser);
            println!("Use the Tauri desktop app for full real-time monitoring.");
            println!("CLI watch mode coming in a future release.");
        }
        Command::Incidents { open, limit } => {
            println!("Incidents (open={}, limit={}):", open, limit);
            println!("Run the BugRadar desktop app to view live incidents.");
        }
        Command::Report { incident_id } => {
            println!("Diagnostic report for incident: {}", incident_id);
            println!("Run the BugRadar desktop app to trigger and view AI reports.");
        }
        Command::Inspect { path } => {
            use br_core::config_inspector::inspect_file;
            let result = inspect_file(&path)?;
            println!("=== Config Inspection: {} ===", result.file_path);
            println!("Format: {} | Keys: {} | Parsed: {}", result.format, result.key_count, result.parsed_ok);
            if result.issues.is_empty() {
                println!("No issues found.");
            } else {
                for issue in &result.issues {
                    println!("[{}] {} — {}", issue.severity.to_uppercase(), issue.key, issue.message);
                    if let Some(ref suggestion) = issue.suggestion {
                        println!("  → {}", suggestion);
                    }
                }
            }
        }
        Command::Metrics => {
            use br_core::sysmon::MetricsCollector;
            let mut collector = MetricsCollector::new();
            let m = collector.collect();
            println!("=== System Metrics ===");
            println!("CPU:    {:.1}% ({} cores)", m.cpu.usage_percent, m.cpu.core_count);
            println!("Memory: {}MB / {}MB ({:.1}%)", m.memory.used_mb, m.memory.total_mb, m.memory.usage_percent);
            println!("Load:   {:.2}", m.load_avg_1m);
            for disk in &m.disks {
                println!("Disk [{}]: {:.1}GB / {:.1}GB ({:.1}%)", disk.mount_point, disk.total_gb - disk.free_gb, disk.total_gb, disk.usage_percent);
            }
        }
    }

    Ok(())
}
