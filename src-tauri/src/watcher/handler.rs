use notify_debouncer_full::notify::RecommendedWatcher;
use notify_debouncer_full::notify::{self, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
}

impl FileWatcher {
    /// Start watching a directory for file changes
    pub fn new<F>(path: &Path, callback: F) -> Result<Self, notify::Error>
    where
        F: Fn(Vec<String>) + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(
            Duration::from_secs(2),
            None,
            move |result: DebounceEventResult| {
                if let Ok(events) = result {
                    let _ = tx.send(events);
                }
            },
        )?;

        debouncer.watcher().watch(
            path,
            notify::RecursiveMode::Recursive,
        )?;

        // Spawn handler thread
        std::thread::spawn(move || {
            while let Ok(events) = rx.recv() {
                let paths: Vec<String> = events
                    .iter()
                    .flat_map(|e| e.paths.iter())
                    .filter_map(|p| p.to_str().map(String::from))
                    .collect();

                if !paths.is_empty() {
                    callback(paths);
                }
            }
        });

        Ok(FileWatcher {
            _debouncer: debouncer,
        })
    }
}
