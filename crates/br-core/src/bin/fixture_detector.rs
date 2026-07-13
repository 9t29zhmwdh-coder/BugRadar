//! Test-only fixture standing in for a real custom detector plugin.
//! Not part of the public BugRadar product; used by external_detector's
//! tests via `env!("CARGO_BIN_EXE_fixture_detector")`.

use std::io::Read;

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "ok".to_string());

    // Drain stdin so the parent's write (and its subsequent shutdown) never blocks.
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);

    match mode.as_str() {
        "ok" => {
            println!(
                r#"{{"anomalies":[{{"label":"disk full","value":9.0,"baseline":1.0,"contributing_entries":["disk full: /var"]}}]}}"#
            );
        }
        "empty" => println!(r#"{{"anomalies":[]}}"#),
        "badjson" => println!("not json"),
        "fail" => {
            eprintln!("fixture_detector: simulated failure");
            std::process::exit(1);
        }
        "sleep" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!(r#"{{"anomalies":[]}}"#);
        }
        other => {
            eprintln!("fixture_detector: unknown mode '{other}'");
            std::process::exit(2);
        }
    }
}
