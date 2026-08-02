use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Duration;

use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use notify::RecursiveMode;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::models::log_entry::LogEntry;
use crate::plugin::LogParserPlugin;

pub struct FileWatcher {
    tx: mpsc::Sender<LogEntry>,
    /// Track file positions (path → byte offset)
    offsets: HashMap<PathBuf, u64>,
}

impl FileWatcher {
    pub fn new(tx: mpsc::Sender<LogEntry>) -> Self {
        Self { tx, offsets: HashMap::new() }
    }

    /// Read new bytes from file since last known offset, parse and send entries
    pub async fn tail_file(
        &mut self,
        path: &Path,
        _source_id: &str,
        parser: &mut Box<dyn LogParserPlugin>,
    ) {
        let offset = self.offsets.entry(path.to_path_buf()).or_insert(0);

        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Cannot open {}: {}", path.display(), e);
                return;
            }
        };

        let len = file.metadata().map(|m| m.len()).unwrap_or(0);

        // If file was rotated (shrunk), reset offset
        if *offset > len {
            *offset = 0;
        }

        if file.seek(SeekFrom::Start(*offset)).is_err() {
            return;
        }

        let mut buf = Vec::new();
        if file.read_to_end(&mut buf).is_err() {
            return;
        }

        *offset += buf.len() as u64;

        let content = String::from_utf8_lossy(&buf);
        for line in content.lines() {
            if let Some(entry) = parser.push_line(line) {
                debug!("Parsed log entry from {}: {}", path.display(), &entry.message[..entry.message.len().min(80)]);
                let _ = self.tx.send(entry).await;
            }
        }

        // Flush partial entry on silence
        if let Some(entry) = parser.flush() {
            let _ = self.tx.send(entry).await;
        }
    }
}

/// Spawn a notify-based file tail task.
/// Returns a handle that, when dropped, stops the watcher.
pub fn spawn_file_tail(
    path: PathBuf,
    source_id: String,
    mut parser: Box<dyn LogParserPlugin>,
    tx: mpsc::Sender<LogEntry>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let (event_tx, mut event_rx) = mpsc::channel::<()>(16);

        let event_tx_clone = event_tx.clone();
        let path_clone = path.clone();

        let mut debouncer = match new_debouncer(
            Duration::from_millis(200),
            None,
            move |result: DebounceEventResult| {
                if result.is_ok() {
                    let _ = event_tx_clone.try_send(());
                }
            },
        ) {
            Ok(d) => d,
            Err(e) => {
                warn!("Cannot create file watcher for {}: {}", path.display(), e);
                return;
            }
        };

        // `.watcher()` ist ab Debouncer 0.7 veraltet: der Debouncer bringt
        // die Watcher-Methoden selbst mit.
        if let Err(e) = debouncer.watch(&path_clone, RecursiveMode::NonRecursive) {
            warn!("Cannot watch {}: {}", path.display(), e);
            return;
        }

        // Initial read
        let mut watcher = FileWatcher::new(tx.clone());
        watcher.tail_file(&path, &source_id, &mut parser).await;

        // React to file changes
        while event_rx.recv().await.is_some() {
            watcher.tail_file(&path, &source_id, &mut parser).await;
        }
    })
}
