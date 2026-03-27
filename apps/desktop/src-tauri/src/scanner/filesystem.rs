use crate::error::CraiteError;
use crate::scanner::source_paths::PlatformPaths;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{AppHandle, Emitter};

pub const AUDIO_EXTENSIONS: &[&str] = &["wav", "mp3", "flac", "ogg", "aiff", "aif"];
const MAX_SCAN_DEPTH: usize = 20;

/// Recursively scan a directory for audio files, optionally emitting file names.
pub fn scan_directory(
    dir: &Path,
    app: Option<&AppHandle>,
    file_counter: &Arc<AtomicUsize>,
    last_emit: &Arc<Mutex<Instant>>,
    output_dir: &str,
) -> Result<Vec<PathBuf>, CraiteError> {
    let mut results = Vec::new();
    scan_recursive(dir, &mut results, 0, app, last_emit, file_counter, output_dir)?;
    Ok(results)
}

fn scan_recursive(
    dir: &Path,
    results: &mut Vec<PathBuf>,
    depth: usize,
    app: Option<&AppHandle>,
    last_emit: &Mutex<Instant>,
    file_counter: &Arc<AtomicUsize>,
    output_dir: &str,
) -> Result<(), CraiteError> {
    if depth > MAX_SCAN_DEPTH {
        log::warn!(
            "Max scan depth ({}) exceeded at: {}",
            MAX_SCAN_DEPTH,
            dir.display()
        );
        return Ok(());
    }

    if !dir.is_dir() {
        return Err(CraiteError::Scanner(format!(
            "Not a directory: {}",
            dir.display()
        )));
    }

    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Use file_type() — avoids an extra stat syscall vs entry.metadata()
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_symlink() {
            log::debug!("Skipping symlink: {}", path.display());
            continue;
        }

        if file_type.is_dir() {
            // Skip the output directory entirely
            if !output_dir.is_empty() && path.to_string_lossy().starts_with(output_dir) {
                continue;
            }
            scan_recursive(&path, results, depth + 1, app, last_emit, file_counter, output_dir)?;
        } else if file_type.is_file() && is_audio_file(&path) {
            file_counter.fetch_add(1, Ordering::Relaxed);
            // Emit file name for UI feedback (throttled ~60fps)
            if let Some(app) = app {
                let should_emit = last_emit
                    .lock()
                    .map(|mut last| {
                        let now = Instant::now();
                        if now.duration_since(*last).as_millis() >= 16 {
                            *last = now;
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);
                if should_emit {
                    let count = file_counter.load(Ordering::Relaxed);
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let _ = app.emit("scan-file", (name, count));
                    }
                }
            }
            results.push(path);
        }
    }

    Ok(())
}

/// Quick recursive count of audio files in a directory (no event emission).
pub fn count_audio_files(dir: &Path, output_dir: &str) -> usize {
    count_recursive(dir, 0, output_dir)
}

fn count_recursive(dir: &Path, depth: usize, output_dir: &str) -> usize {
    if depth > MAX_SCAN_DEPTH || !dir.is_dir() {
        return 0;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            if !output_dir.is_empty() && path.to_string_lossy().starts_with(output_dir) {
                continue;
            }
            count += count_recursive(&path, depth + 1, output_dir);
        } else if ft.is_file() && is_audio_file(&path) {
            count += 1;
        }
    }
    count
}

/// Check if a file has an audio extension
pub fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Detect common sample directories on the system
pub fn detect_sample_directories() -> Vec<(String, String)> {
    PlatformPaths::resolve().detect_sources()
}
