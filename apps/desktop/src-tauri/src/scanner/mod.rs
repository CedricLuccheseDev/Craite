pub mod daw_detection;
pub mod filesystem;
pub mod source_paths;
pub mod wsl;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::classifier::filename::classify_by_filename;
use crate::classifier::rules::RULES;
use crate::db::models::{Category, Sample, ScanResult};
use crate::error::ResultExt;
use crate::scanner::filesystem::scan_directory;

/// Scan source directories, classify files, persist results, and return a ScanResult.
/// Uses rayon for parallel classification and incremental scanning (mtime-based).
pub fn execute_scan(source_paths: &[String]) -> Result<ScanResult, String> {
    let conn = crate::db::connection::open_connection().str_err()?;
    let mut all_samples: Vec<Sample> = Vec::new();

    for source_path in source_paths {
        let path = Path::new(source_path);
        let files = scan_directory(path).str_err()?;
        let source_samples = classify_incremental(&conn, source_path, &files)?;
        all_samples.extend(source_samples);
    }

    let categories = build_categories(&all_samples);
    let classified_count = all_samples
        .iter()
        .filter(|s| s.category != "unknown")
        .count();

    Ok(ScanResult {
        total_files: all_samples.len(),
        classified_files: classified_count,
        categories,
        samples: all_samples,
    })
}

/// Incremental + parallel classification for a single source.
/// Only re-classifies files that are new or modified since last scan.
fn classify_incremental(
    conn: &rusqlite::Connection,
    source_path: &str,
    files: &[PathBuf],
) -> Result<Vec<Sample>, String> {
    let existing = crate::db::repository::load_mtimes_by_source(conn, source_path).str_err()?;

    let disk_paths: std::collections::HashSet<String> = files
        .iter()
        .map(|f| f.to_string_lossy().to_string())
        .collect();

    // Find files removed from disk
    let removed: Vec<String> = existing
        .keys()
        .filter(|p| !disk_paths.contains(p.as_str()))
        .cloned()
        .collect();

    if !removed.is_empty() {
        crate::db::repository::remove_samples_by_paths(conn, &removed).str_err()?;
    }

    // Partition files into unchanged vs needs-classification
    let (to_classify, unchanged_paths): (Vec<&PathBuf>, Vec<&PathBuf>) =
        files.iter().partition(|file| {
            let path_str = file.to_string_lossy();
            let file_mtime = get_mtime(file);
            match existing.get(path_str.as_ref()) {
                Some(&stored_mtime) => file_mtime != stored_mtime,
                None => true,
            }
        });

    // Parallel classification of new/modified files using rayon
    let new_samples: Vec<Sample> = to_classify
        .par_iter()
        .map(|file| {
            let classification = classify_by_filename(file);
            Sample {
                id: 0,
                name: file
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                path: file.to_string_lossy().to_string(),
                category: classification.category,
                subcategory: classification.subcategory,
                confidence: classification.confidence,
                source: source_path.to_string(),
                duration: 0.0,
                sample_rate: 0,
                linked_path: None,
                mtime: get_mtime(file),
            }
        })
        .collect();

    // Persist only the new/modified samples
    if !new_samples.is_empty() {
        crate::db::repository::insert_samples(conn, &new_samples).str_err()?;
    }

    // Reload all samples for this source (includes unchanged + newly inserted)
    let all_source_samples = if unchanged_paths.is_empty() && removed.is_empty() {
        new_samples
    } else {
        crate::db::repository::load_samples_by_source(conn, source_path).str_err()?
    };

    Ok(all_source_samples)
}

fn get_mtime(path: &Path) -> u64 {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn build_categories(samples: &[Sample]) -> Vec<Category> {
    let mut map: HashMap<String, (Vec<String>, usize)> = HashMap::new();

    for sample in samples {
        if sample.category == "unknown" {
            continue;
        }
        let entry = map
            .entry(sample.category.clone())
            .or_insert_with(|| (Vec::new(), 0));
        entry.1 += 1;
        if !sample.subcategory.is_empty() && !entry.0.contains(&sample.subcategory) {
            entry.0.push(sample.subcategory.clone());
        }
    }

    let mut categories: Vec<Category> = map
        .into_iter()
        .map(|(name, (subcategories, count))| {
            let color = RULES
                .iter()
                .find(|r| r.category == name)
                .map(|r| r.color.to_string())
                .unwrap_or_else(|| "#888888".to_string());

            Category {
                name,
                color,
                count,
                subcategories,
            }
        })
        .collect();

    categories.sort_by(|a, b| b.count.cmp(&a.count));
    categories
}
