use std::path::Path;
use std::sync::Mutex;
use tauri::State;

use crate::audio::preview::AudioPreview;
use crate::classifier::filename::classify_by_filename;
use crate::classifier::rules::RULES;
use crate::db::models::{Category, Sample, ScanResult, Source};
use crate::scanner::filesystem::{detect_sample_directories, scan_directory};

pub struct AudioState(pub Mutex<AudioPreview>);

#[tauri::command]
pub fn scan_directories(sources: Vec<String>) -> Result<ScanResult, String> {
    let mut all_samples: Vec<Sample> = Vec::new();
    let mut id_counter: i64 = 0;

    for source_path in &sources {
        let path = Path::new(source_path);
        let files = scan_directory(path).map_err(|e| e.to_string())?;

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

    Ok(ScanResult {
        total_files: all_samples.len(),
        classified_files: classified_count,
        categories,
        samples: all_samples,
    })
}

#[tauri::command]
pub fn detect_sources() -> Result<Vec<Source>, String> {
    let dirs = detect_sample_directories();

    Ok(dirs
        .into_iter()
        .map(|(path, label)| Source {
            path,
            label,
            enabled: true,
            source_type: "detected".to_string(),
            sample_count: 0,
        })
        .collect())
}

#[tauri::command]
pub fn preview_sample(
    path: String,
    audio: State<'_, AudioState>,
) -> Result<(), String> {
    let mut preview = audio.0.lock().map_err(|e| e.to_string())?;
    preview.play(Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_preview(audio: State<'_, AudioState>) -> Result<(), String> {
    let mut preview = audio.0.lock().map_err(|e| e.to_string())?;
    preview.stop();
    Ok(())
}

fn build_categories(samples: &[Sample]) -> Vec<Category> {
    use std::collections::HashMap;

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
