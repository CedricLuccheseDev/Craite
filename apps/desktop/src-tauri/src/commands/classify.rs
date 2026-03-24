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

    // Wipe and recreate the output directory to ensure a clean structure.
    // This handles renamed groups (e.g. melodic→synths, bass/bass→bass) automatically.
    if output_path.is_dir() {
        std::fs::remove_dir_all(output_path).str_err()?;
    }
    std::fs::create_dir_all(output_path).str_err()?;

    let conn = open_connection().str_err()?;
    let excluded_set: std::collections::HashSet<&str> =
        excluded_categories.iter().map(|s| s.as_str()).collect();

    let mut stmt = conn
        .prepare("SELECT path, category, name FROM samples WHERE category != ''")
        .str_err()?;

    // Normalize output path for prefix comparison
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
        // Skip samples whose source is inside the output directory (prevents self-linking
        // when the output folder is nested inside a scanned source folder)
        .filter(|(path, _, _)| !path.starts_with(&output_prefix))
        .collect();

    let total = samples.len();
    let output = output_path.to_path_buf();

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

    // Return total included samples (not just newly created links)
    Ok(total)
}

#[tauri::command]
pub async fn create_links(output_dir: String, excluded_categories: Vec<String>) -> Result<usize, String> {
    run_blocking(move || regenerate_links(&output_dir, &excluded_categories)).await
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
