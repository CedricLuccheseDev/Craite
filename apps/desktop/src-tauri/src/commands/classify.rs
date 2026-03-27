use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::db::connection::open_connection;
use crate::error::{run_blocking, ResultExt};
use crate::linker::strategy::{create_link, determine_strategy};

/// Maps a leaf category to its parent group folder name.
fn category_group(category: &str) -> &'static str {
    match category {
        "kick" | "snare" | "hihat" | "clap" | "cymbal" | "tom" | "perc" => "drums",
        "pad" | "lead" | "arp" | "chord" => "synths",
        "bass" => "bass",
        "vocal" => "vocals",
        "fx" => "fx",
        "loop" => "loops",
        "unknown" => "unknown",
        _ => "other",
    }
}

/// Single-category groups use a flat structure: output/group/filename.wav
/// Multi-category groups nest: output/group/category/filename.wav
fn is_flat_group(category: &str) -> bool {
    matches!(category, "bass" | "vocal" | "fx" | "loop" | "unknown")
}

/// Reusable link generation logic, callable from commands and background tasks.
/// Uses rayon to parallelize filesystem linking operations.
/// Multi-category groups: output_dir/<group>/<category>/<filename>
/// Single-category groups: output_dir/<group>/<filename>
/// Excluded categories have their output folder removed and are skipped during linking.
pub fn regenerate_links(output_dir: &str, excluded_categories: &[String]) -> Result<usize, String> {
    let output_path = Path::new(output_dir);
    std::fs::create_dir_all(output_path).str_err()?;

    let conn = open_connection().str_err()?;
    let excluded_set: std::collections::HashSet<&str> =
        excluded_categories.iter().map(|s| s.as_str()).collect();

    let mut stmt = conn
        .prepare("SELECT path, category, name FROM samples WHERE category != '' AND hidden = 0")
        .str_err()?;

    let output_prefix = output_path.to_string_lossy().to_string();

    let samples: Vec<(String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .str_err()?
        .filter_map(|r| r.ok())
        .filter(|(_, cat, _)| !excluded_set.contains(cat.as_str()))
        .filter(|(path, _, _)| !path.starts_with(&output_prefix))
        .collect();

    let total = samples.len();
    let output = output_path.to_path_buf();

    // Build set of expected target paths
    let expected: std::collections::HashSet<PathBuf> = samples
        .iter()
        .map(|(_, category, name)| {
            let group = category_group(category);
            if is_flat_group(category) {
                output.join(group).join(name)
            } else {
                output.join(group).join(category).join(name)
            }
        })
        .collect();

    // Remove existing links that are no longer expected
    remove_stale_links(output_path, &expected);

    // Create only missing links (in parallel)
    samples
        .par_iter()
        .for_each(|(source_path, category, name)| {
            let source = PathBuf::from(source_path);
            let group = category_group(category);
            let target = if is_flat_group(category) {
                output.join(group).join(name)
            } else {
                output.join(group).join(category).join(name)
            };

            if target.exists() {
                return;
            }

            let strategy = determine_strategy(&source, &target);
            if let Err(e) = create_link(&source, &target, strategy) {
                log::warn!(
                    "Failed to link {} to {}: {}",
                    source.display(),
                    target.display(),
                    e
                );
            }
        });

    Ok(total)
}

/// Walk the output directory and remove files not in the expected set.
/// Also cleans up empty directories left behind.
fn remove_stale_links(dir: &Path, expected: &std::collections::HashSet<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_stale_links(&path, expected);
            // Remove empty directories
            if std::fs::read_dir(&path).map(|mut d| d.next().is_none()).unwrap_or(false) {
                let _ = std::fs::remove_dir(&path);
            }
        } else if !expected.contains(&path) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

#[tauri::command]
pub async fn create_links(output_dir: String, excluded_categories: Vec<String>) -> Result<usize, String> {
    run_blocking(move || regenerate_links(&output_dir, &excluded_categories)).await
}

/// Convert a WSL `/mnt/X/...` path to a Windows `X:\...` path.
fn wsl_to_windows_path(path: &str) -> Option<String> {
    let stripped = path.strip_prefix("/mnt/")?;
    let drive = stripped.chars().next()?;
    let rest = &stripped[1..];
    Some(format!("{}:{}", drive.to_uppercase(), rest.replace('/', "\\")))
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    let resolved = wsl_to_windows_path(&path).unwrap_or_else(|| path.clone());

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&resolved)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    {
        // On WSL/Linux, try explorer.exe first (works in WSL), fall back to xdg-open
        if std::process::Command::new("explorer.exe")
            .arg(&resolved)
            .spawn()
            .is_err()
        {
            std::process::Command::new("xdg-open")
                .arg(&path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

