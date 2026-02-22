pub mod daw_detection;
pub mod filesystem;
pub mod source_paths;
pub mod wsl;

use std::collections::HashMap;
use std::path::Path;

use crate::classifier::filename::classify_by_filename;
use crate::classifier::rules::RULES;
use crate::db::models::{Category, Sample, ScanResult};
use crate::error::ResultExt;
use crate::scanner::filesystem::scan_directory;

/// Scan source directories, classify files, persist results, and return a ScanResult.
/// Shared between foreground scan command and background scanner.
pub fn execute_scan(source_paths: &[String]) -> Result<ScanResult, String> {
    let mut all_samples: Vec<Sample> = Vec::new();
    let mut id_counter: i64 = 0;

    for source_path in source_paths {
        let path = Path::new(source_path);
        let files = scan_directory(path).str_err()?;

        for file_path in files {
            let classification = classify_by_filename(&file_path);
            id_counter += 1;

            all_samples.push(Sample {
                id: id_counter,
                name: file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                path: file_path.to_string_lossy().to_string(),
                category: classification.category,
                subcategory: classification.subcategory,
                confidence: classification.confidence,
                source: source_path.clone(),
                duration: 0.0,
                sample_rate: 0,
                linked_path: None,
            });
        }
    }

    let categories = build_categories(&all_samples);
    let classified_count = all_samples.iter()
        .filter(|s| s.category != "unknown")
        .count();

    if let Ok(conn) = crate::db::connection::open_connection() {
        for source_path in source_paths {
            let _ = crate::db::repository::clear_samples_by_source(&conn, source_path);
        }
        let _ = crate::db::repository::insert_samples(&conn, &all_samples);
    }

    Ok(ScanResult {
        total_files: all_samples.len(),
        classified_files: classified_count,
        categories,
        samples: all_samples,
    })
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
        if !sample.subcategory.is_empty()
            && !entry.0.contains(&sample.subcategory)
        {
            entry.0.push(sample.subcategory.clone());
        }
    }

    let mut categories: Vec<Category> = map
        .into_iter()
        .map(|(name, (subcategories, count))| {
            let color = RULES.iter()
                .find(|r| r.category == name)
                .map(|r| r.color.to_string())
                .unwrap_or_else(|| "#888888".to_string());

            Category { name, color, count, subcategories }
        })
        .collect();

    categories.sort_by(|a, b| b.count.cmp(&a.count));
    categories
}
