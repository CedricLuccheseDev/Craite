use notify_debouncer_full::notify::RecommendedWatcher;
use notify_debouncer_full::notify::{self, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use serde::Serialize;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::scanner::filesystem::is_audio_file;

/// Payload emitted when the watcher detects new audio files
#[derive(Debug, Clone, Serialize)]
pub struct SamplesChangedPayload {
    pub paths: Vec<String>,
}

pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
}

impl FileWatcher {
    /// Start watching directories for audio file changes.
    /// Uses NonRecursive mode to avoid thousands of inotify watches
    /// on slow filesystems (WSL2 /mnt/c). The background scanner
    /// handles deep subdirectory changes periodically.
    pub fn new(paths: &[String], app_handle: AppHandle) -> Result<Self, notify::Error> {
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

        for dir in paths {
            let path = Path::new(dir);
            if path.is_dir() {
                let _ = debouncer
                    .watcher()
                    .watch(path, notify::RecursiveMode::NonRecursive);
            }
        }

        let handle = app_handle.clone();
        std::thread::spawn(move || {
            while let Ok(events) = rx.recv() {
                let audio_paths: Vec<String> = events
                    .iter()
                    .flat_map(|e| e.paths.iter())
                    .filter(|p| is_audio_file(p) && p.exists())
                    .filter_map(|p| p.to_str().map(String::from))
                    .collect();

                if !audio_paths.is_empty() {
                    let payload = SamplesChangedPayload { paths: audio_paths };
                    let _ = handle.emit("samples-changed", &payload);
                }
            }
        });

        Ok(FileWatcher {
            _debouncer: debouncer,
        })
    }
}
