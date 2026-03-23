use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

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
    app: AppHandle,
) -> Result<(), String> {
    let mut preview = audio.0.lock().str_err()?;
    let gen = preview.play(Path::new(&path)).str_err()?;

    // Monitor playback end in a background thread.
    // Uses a generation counter to avoid stale threads emitting events
    // after a new sample has started playing.
    let sink = preview.sink_ref();
    let generation = preview.generation_ref();
    let played_path = path;

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(200));

            // If a newer play() was called, this thread is stale
            if generation.load(Ordering::SeqCst) != gen {
                return;
            }

            let is_done = sink
                .lock()
                .map(|guard| guard.as_ref().map_or(true, |s| s.empty()))
                .unwrap_or(true);

            if is_done {
                let _ = app.emit("preview-ended", &played_path);
                return;
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_preview(audio: State<'_, AudioState>) -> Result<(), String> {
    let mut preview = audio.0.lock().str_err()?;
    preview.stop();
    Ok(())
}

/// Read an audio file and return its raw bytes for frontend Blob playback
#[tauri::command]
pub async fn read_audio_file(path: String) -> Result<Vec<u8>, String> {
    run_blocking(move || std::fs::read(&path).str_err()).await
}
