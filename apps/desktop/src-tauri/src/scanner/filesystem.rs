use std::path::{Path, PathBuf};
use crate::error::CraiteError;
use crate::scanner::source_paths::PlatformPaths;

pub const AUDIO_EXTENSIONS: &[&str] = &["wav", "mp3", "flac", "ogg", "aiff", "aif"];
const MAX_SCAN_DEPTH: usize = 20;

/// Recursively scan a directory for audio files
pub fn scan_directory(dir: &Path) -> Result<Vec<PathBuf>, CraiteError> {
    let mut results = Vec::new();
    scan_recursive(dir, &mut results, 0)?;
    Ok(results)
}

fn scan_recursive(dir: &Path, results: &mut Vec<PathBuf>, depth: usize) -> Result<(), CraiteError> {
    if depth > MAX_SCAN_DEPTH {
        log::warn!("Max scan depth ({}) exceeded at: {}", MAX_SCAN_DEPTH, dir.display());
        return Ok(()); // Continue scanning other directories instead of failing
    }

    if !dir.is_dir() {
        return Err(CraiteError::Scanner(
            format!("Not a directory: {}", dir.display()),
        ));
    }

    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Skip symlinks to avoid potential security issues and infinite loops
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_symlink() {
                log::debug!("Skipping symlink: {}", path.display());
                continue;
            }
        }

        if path.is_dir() {
            scan_recursive(&path, results, depth + 1)?;
        } else if is_audio_file(&path) {
            results.push(path);
        }
    }

    Ok(())
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
