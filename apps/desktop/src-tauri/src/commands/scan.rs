use std::path::Path;
use std::sync::Mutex;
use tauri::State;

use crate::audio::preview::AudioPreview;
use crate::db::models::{ScanResult, Source};
use crate::error::{run_blocking, ResultExt};
use crate::scanner::execute_scan;
use crate::scanner::filesystem::detect_sample_directories;

pub struct AudioState(pub Mutex<AudioPreview>);

#[tauri::command]
pub async fn scan_directories(sources: Vec<String>) -> Result<ScanResult, String> {
    run_blocking(move || execute_scan(&sources)).await
}

#[tauri::command]
pub async fn detect_sources() -> Result<Vec<Source>, String> {
    run_blocking(|| {
        Ok(detect_sample_directories()
            .into_iter()
            .map(|(path, label)| Source {
                path,
                label,
                enabled: true,
                source_type: "detected".to_string(),
                sample_count: 0,
            })
            .collect())
    })
    .await
}

#[tauri::command]
pub fn preview_sample(
    path: String,
    audio: State<'_, AudioState>,
) -> Result<(), String> {
    let mut preview = audio.0.lock().str_err()?;
    preview.play(Path::new(&path)).str_err()
}

#[tauri::command]
pub fn stop_preview(audio: State<'_, AudioState>) -> Result<(), String> {
    let mut preview = audio.0.lock().str_err()?;
    preview.stop();
    Ok(())
}
