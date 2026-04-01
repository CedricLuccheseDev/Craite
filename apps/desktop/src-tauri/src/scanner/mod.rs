pub mod daw_detection;
pub mod filesystem;
pub mod source_paths;
pub mod wsl;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex as StdMutex};

use rayon::prelude::*;
use tauri::{AppHandle, Emitter};

use crate::classifier::audio::classify_by_audio;
use crate::classifier::context::apply_directory_context;
use crate::classifier::path::classify_by_path;
use crate::classifier::rules::RULES;
use crate::db::models::{Category, Sample, ScanResult};
use crate::error::ResultExt;
use crate::scanner::filesystem::scan_directory;

/// Scan source directories, classify files, persist results, and return a ScanResult.
/// Uses rayon for parallel classification and incremental scanning (mtime-based).
pub fn execute_scan(
    source_paths: &[String],
    app: Option<&AppHandle>,
) -> Result<ScanResult, String> {
    let conn = crate::db::connection::open_connection().str_err()?;

    // Read output_dir so we can skip files inside it (prevents indexing generated links
    // when the output folder is nested inside a scanned source directory).
    let output_dir = crate::db::repository::get_setting(&conn, "output_dir")
        .str_err()?
        .unwrap_or_default();

    // Shared state for throttled event emission
    let last_emit = Arc::new(StdMutex::new(std::time::Instant::now()));
    let file_counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut all_samples: Vec<Sample> = Vec::new();

    for source_path in source_paths {
        let path = Path::new(source_path);
        let files = scan_directory(path, app, &file_counter, &last_emit, &output_dir).str_err()?;

        let source_samples =
            classify_incremental(&conn, source_path, &files, &last_emit, &file_counter, app)?;
        all_samples.extend(source_samples);
    }

    // Post-classification: use sibling context to resolve remaining unknowns
    apply_directory_context(&mut all_samples);

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
    last_emit: &Arc<StdMutex<std::time::Instant>>,
    file_counter: &Arc<std::sync::atomic::AtomicUsize>,
    app: Option<&AppHandle>,
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

    // Partition files into unchanged vs needs-classification, caching mtime to avoid double syscall
    let to_classify: Vec<(&PathBuf, u64)> = files
        .iter()
        .filter_map(|file| {
            let mtime = get_mtime(file);
            let path_str = file.to_string_lossy();
            let needs_classify = match existing.get(path_str.as_ref()) {
                Some(&stored_mtime) => mtime != stored_mtime,
                None => true,
            };
            if needs_classify {
                Some((file, mtime))
            } else {
                None
            }
        })
        .collect();

    let unchanged_count = files.len() - to_classify.len();

    // Parallel classification of new/modified files using rayon
    let le = Arc::clone(last_emit);
    let fc = Arc::clone(file_counter);
    let app_clone = app.cloned();
    let new_samples: Vec<Sample> = to_classify
        .par_iter()
        .map(|(file, mtime)| {
            // Emit file name + count (throttled)
            if let Some(ref app) = app_clone {
                let count = fc.load(std::sync::atomic::Ordering::Relaxed);
                let should_emit = le
                    .lock()
                    .map(|mut last| {
                        let now = std::time::Instant::now();
                        if now.duration_since(*last).as_millis() >= 16 {
                            *last = now;
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);

                if should_emit {
                    let name = file.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    let _ = app.emit("scan-file", (name, count));
                }
            }

            let source_root = Path::new(source_path);
            let mut classification = classify_by_path(file, source_root);

            if classification.category == "unknown" {
                if let Some(audio_cls) = classify_by_audio(file) {
                    classification = audio_cls;
                }
            }

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
                duration: classification.duration as f64,
                sample_rate: classification.sample_rate,
                linked_path: None,
                mtime: *mtime,
                hidden: false,
            }
        })
        .collect();

    // Persist only the new/modified samples
    if !new_samples.is_empty() {
        crate::db::repository::insert_samples(conn, &new_samples).str_err()?;
    }

    // Reload all samples for this source (includes unchanged + newly inserted)
    let all_source_samples = if unchanged_count == 0 && removed.is_empty() {
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
    let mut unknown_count: usize = 0;

    for sample in samples {
        if sample.category == "unknown" {
            unknown_count += 1;
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

    // Append unknown at the end so the frontend can show it separately
    if unknown_count > 0 {
        categories.push(Category {
            name: "unknown".to_string(),
            color: "#6b7280".to_string(),
            count: unknown_count,
            subcategories: Vec::new(),
        });
    }

    categories
}
