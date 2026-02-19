mod audio;
mod classifier;
mod commands;
mod db;
mod error;
mod linker;
mod scanner;
mod watcher;

use std::sync::Mutex;
use audio::preview::AudioPreview;
use commands::{classify, scan, updater};
use commands::scan::AudioState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_process::init())?;
            Ok(())
        })
        .manage(AudioState(Mutex::new(AudioPreview::new())))
        .invoke_handler(tauri::generate_handler![
            scan::scan_directories,
            scan::detect_sources,
            scan::preview_sample,
            scan::stop_preview,
            classify::classify_samples,
            classify::create_links,
            updater::check_for_update,
            updater::download_and_install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CrAIte");
}
