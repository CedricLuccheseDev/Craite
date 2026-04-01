use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

use crate::audio::preview::AudioPreview;
use crate::background::state::BackgroundScanState;
use crate::db::models::{ScanResult, Source};
use crate::error::{run_blocking, ResultExt};
use crate::scanner::execute_scan;
use crate::scanner::filesystem::detect_sample_directories;
use crate::security::path_validator::sanitize_path;

pub struct AudioState(pub Mutex<AudioPreview>);

// SAFETY: AudioPreview contains rodio::OutputStream which is !Send on macOS (CoreAudio backend).
// Access is fully serialized through the Mutex, so sending between threads is safe.
unsafe impl Send for AudioState {}
unsafe impl Sync for AudioState {}

#[tauri::command]
pub async fn scan_directories(
    sources: Vec<String>,
    app: AppHandle,
    bg_state: State<'_, BackgroundScanState>,
) -> Result<ScanResult, String> {
    for source in &sources {
        sanitize_path(source)?;
    }
    bg_state.mark_scanning(true);
    let result = run_blocking(move || execute_scan(&sources, Some(&app))).await;
    bg_state.mark_scanning(false);
    result
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
    app: AppHandle,
) -> Result<(), String> {
    let validated = sanitize_path(&path)?;
    let mut preview = audio.inner().0.lock().str_err()?;
    let gen = preview.play(&validated).str_err()?;

    let sink = preview.sink_ref();
    let generation = preview.generation_ref();
    let played_path = path;

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(200));

        if generation.load(Ordering::SeqCst) != gen {
            return;
        }

        let is_done = sink
            .lock()
            .map(|guard: std::sync::MutexGuard<Option<rodio::Sink>>| {
                guard.as_ref().is_none_or(|s| s.empty())
            })
            .unwrap_or(true);

        if is_done {
            let _ = app.emit("preview-ended", &played_path);
            return;
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_preview(audio: State<'_, AudioState>) -> Result<(), String> {
    let mut preview = audio.inner().0.lock().str_err()?;
    preview.stop();
    Ok(())
}

#[tauri::command]
pub async fn read_audio_file(path: String) -> Result<Vec<u8>, String> {
    let validated = sanitize_path(&path)?;
    run_blocking(move || std::fs::read(&validated).str_err()).await
}
