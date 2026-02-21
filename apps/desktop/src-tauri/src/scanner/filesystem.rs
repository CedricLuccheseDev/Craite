use std::path::{Path, PathBuf};
use crate::error::CraiteError;

const AUDIO_EXTENSIONS: &[&str] = &["wav", "mp3", "flac", "ogg", "aiff", "aif"];

/// Recursively scan a directory for audio files
pub fn scan_directory(dir: &Path) -> Result<Vec<PathBuf>, CraiteError> {
    let mut results = Vec::new();
    scan_recursive(dir, &mut results)?;
    Ok(results)
}

fn scan_recursive(dir: &Path, results: &mut Vec<PathBuf>) -> Result<(), CraiteError> {
    if !dir.is_dir() {
        return Err(CraiteError::Scanner(
            format!("Not a directory: {}", dir.display()),
        ));
    }

    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_recursive(&path, results)?;
        } else if is_audio_file(&path) {
            results.push(path);
        }
    }

    Ok(())
}

fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Detect common sample directories on the system
pub fn detect_sample_directories() -> Vec<(String, String)> {
    let mut dirs = Vec::new();

    // HOME for Linux/Mac, USERPROFILE for Windows native — normalize to forward slashes
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default()
        .replace('\\', "/");

    // WSL only: derive Windows username from /home/<user> to build /mnt/c/ paths
    let win_user = home.trim_start_matches("/home/");
    let mnt_user = format!("/mnt/c/Users/{win_user}");

    let candidates: &[(&str, &str)] = &[
        // Splice — native (Linux/Mac/Windows)
        ("{home}/Splice/sounds", "Splice"),
        // Splice — WSL
        ("{mnt}/Splice/sounds", "Splice"),
        // FL Studio factory packs — Windows native
        ("C:/Program Files/Image-Line/FL Studio 20/Data/Patches/Packs", "FL Studio 20 Packs"),
        ("C:/Program Files/Image-Line/FL Studio 21/Data/Patches/Packs", "FL Studio 21 Packs"),
        // FL Studio factory packs — WSL
        ("/mnt/c/Program Files/Image-Line/FL Studio 20/Data/Patches/Packs", "FL Studio 20 Packs"),
        ("/mnt/c/Program Files/Image-Line/FL Studio 21/Data/Patches/Packs", "FL Studio 21 Packs"),
        // FL Studio user content — native + WSL
        ("{home}/Documents/Image-Line/FL Studio", "FL Studio"),
        ("{mnt}/Documents/Image-Line/FL Studio", "FL Studio"),
        // Common locations — native
        ("{home}/Documents/Samples", "Documents/Samples"),
        ("{home}/Music/Samples", "Music/Samples"),
        ("{home}/Desktop/Samples", "Desktop/Samples"),
        // Common locations — WSL
        ("{mnt}/Documents/Samples", "Documents/Samples"),
        ("{mnt}/Music/Samples", "Music/Samples"),
        ("{mnt}/Desktop/Samples", "Desktop/Samples"),
    ];

    for (template, label) in candidates {
        let path = template
            .replace("{home}", &home)
            .replace("{mnt}", &mnt_user);
        if Path::new(&path).is_dir() {
            dirs.push((path, label.to_string()));
        }
    }

    dirs
}
