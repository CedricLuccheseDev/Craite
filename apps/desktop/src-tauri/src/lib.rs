mod audio;
mod background;
mod classifier;
mod commands;
mod db;
mod error;
mod linker;
mod scanner;
mod security;
mod tray;
mod watcher;

use audio::preview::AudioPreview;
use background::state::BackgroundScanState;
use commands::scan::AudioState;
use commands::watcher::WatcherState;
use commands::{
    background as bg_cmd, classify, daw, persistence, scan, updater, watcher as watcher_cmd,
};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_process::init())?;

            // Hide window if launched via autostart with --minimized flag
            let args: Vec<String> = std::env::args().collect();
            if args.contains(&"--minimized".to_string()) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            // Load background scan settings from DB
            let bg_state = BackgroundScanState::default();
            if let Ok(conn) = db::connection::open_connection() {
                if let Ok(Some(val)) = db::repository::get_setting(&conn, "background_scan_enabled")
                {
                    bg_state.set_enabled(val == "true");
                }
                if let Ok(Some(val)) = db::repository::get_setting(&conn, "scan_interval_minutes") {
                    if let Ok(mins) = val.parse::<u64>() {
                        bg_state.set_interval(mins);
                    }
                }
            }
            app.manage(bg_state);

            // Build system tray
            tray::builder::create_tray(app.handle())?;

            // Start background scanner loop
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                background::scanner::start_periodic_scan(handle).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .manage(AudioState(Mutex::new(AudioPreview::new())))
        .manage(WatcherState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            scan::scan_directories,
            scan::detect_sources,
            scan::preview_sample,
            scan::stop_preview,
            scan::read_audio_file,
            classify::create_links,
            classify::open_folder,
            persistence::save_samples,
            persistence::load_samples,
            persistence::save_source,
            persistence::load_sources,
            persistence::update_source_enabled,
            persistence::delete_source,
            persistence::set_sample_hidden,
            persistence::update_sample_category,
            persistence::save_setting,
            persistence::load_all_settings,
            persistence::reset_app,
            daw::detect_installed_daws,
            daw::create_daw_library_folder,
            updater::check_for_update,
            updater::download_and_install_update,
            watcher_cmd::start_watching,
            watcher_cmd::stop_watching,
            bg_cmd::set_background_scan_enabled,
            bg_cmd::set_scan_interval,
            bg_cmd::get_background_scan_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CrAIte");
}
