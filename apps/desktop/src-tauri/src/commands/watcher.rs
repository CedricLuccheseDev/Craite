use std::sync::Mutex;
use tauri::{AppHandle, State};

use crate::watcher::handler::FileWatcher;

pub struct WatcherState(pub Mutex<Option<FileWatcher>>);

#[tauri::command]
pub async fn start_watching(
    paths: Vec<String>,
    app: AppHandle,
    state: State<'_, WatcherState>,
) -> Result<(), String> {
    // Stop existing watcher first (quick lock, no I/O)
    {
        let mut guard = state.0.lock().map_err(|e| e.to_string())?;
        *guard = None;
    }

    // Create watcher off the main thread (does filesystem checks)
    let watcher = tokio::task::spawn_blocking(move || {
        FileWatcher::new(&paths, app).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    *guard = Some(watcher);
    Ok(())
}

#[tauri::command]
pub fn stop_watching(state: State<'_, WatcherState>) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    *guard = None;
    Ok(())
}
