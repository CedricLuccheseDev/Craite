use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time;

use crate::background::state::BackgroundScanState;
use crate::commands::classify::regenerate_links;
use crate::db::connection::open_connection;
use crate::db::repository;
use crate::error::ResultExt;
use crate::scanner::execute_scan;
use crate::scanner::filesystem::count_audio_files;

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundScanResult {
    pub total_files: usize,
    pub source_count: usize,
    pub linked_count: usize,
    pub skipped: bool,
}

/// Runs forever, periodically scanning enabled sources.
pub async fn start_periodic_scan(app: AppHandle) {
    time::sleep(Duration::from_secs(10)).await;

    loop {
        let state = app.state::<BackgroundScanState>();
        let interval_min = state.interval();

        if state.is_enabled() {
            run_scan_cycle(&app).await;
        }

        let sleep_secs = if interval_min == 0 { 10 } else { interval_min * 60 };
        time::sleep(Duration::from_secs(sleep_secs)).await;
    }
}

/// Single scan cycle with atomic guard against concurrent runs.
pub async fn run_scan_cycle(app: &AppHandle) {
    let state = app.state::<BackgroundScanState>();

    // Atomically try to acquire the scanning lock
    if state
        .is_scanning
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    // Quick check on a blocking thread: count files on disk vs DB
    let should_scan = tokio::task::spawn_blocking(should_perform_scan)
        .await
        .unwrap_or(true);

    if !should_scan {
        state.mark_scanning(false);
        let _ = app.emit("background-scan-skipped", ());
        return;
    }

    let _ = app.emit("background-scan-started", ());

    let result = tokio::task::spawn_blocking(perform_scan).await;

    state.mark_scanning(false);

    match result {
        Ok(Ok(scan_result)) => {
            log::info!(
                "Background scan complete: {} files across {} sources, {} linked",
                scan_result.total_files,
                scan_result.source_count,
                scan_result.linked_count
            );
            let _ = app.emit("background-scan-completed", &scan_result);
        }
        Ok(Err(e)) => {
            log::error!("Background scan failed: {}", e);
            let _ = app.emit("background-scan-error", &e);
        }
        Err(e) => {
            log::error!("Background scan task panicked: {}", e);
        }
    }
}

/// Quick check: compare audio file count on disk vs DB sample count.
fn should_perform_scan() -> bool {
    let conn = match open_connection() {
        Ok(c) => c,
        Err(_) => return true,
    };

    let sources = match repository::load_all_sources(&conn) {
        Ok(s) => s,
        Err(_) => return true,
    };
    let enabled: Vec<_> = sources.iter().filter(|s| s.enabled).collect();
    let source_paths: Vec<String> = enabled.iter().map(|s| s.path.clone()).collect();

    let output_dir = repository::get_setting(&conn, "output_dir")
        .ok()
        .flatten()
        .unwrap_or_default();

    let db_count = repository::count_all_samples(&conn).unwrap_or(0);
    let disk_count: usize = source_paths
        .iter()
        .map(|p| count_audio_files(std::path::Path::new(p), &output_dir))
        .sum();

    if disk_count == db_count && db_count > 0 {
        log::info!(
            "Background scan skipped: disk count ({}) matches DB count",
            disk_count
        );
        return false;
    }

    true
}

/// Full scan + optional link regeneration.
fn perform_scan() -> Result<BackgroundScanResult, String> {
    let conn = open_connection().str_err()?;
    let sources = repository::load_all_sources(&conn).str_err()?;
    let enabled: Vec<_> = sources.iter().filter(|s| s.enabled).collect();
    let source_paths: Vec<String> = enabled.iter().map(|s| s.path.clone()).collect();

    let scan_result = execute_scan(&source_paths, None)?;

    // Auto-regenerate links if output_dir is configured
    let linked_count = match repository::get_setting(&conn, "output_dir") {
        Ok(Some(dir)) if !dir.is_empty() => {
            let excluded: Vec<String> = repository::get_setting(&conn, "excluded_categories")
                .ok()
                .flatten()
                .and_then(|json| serde_json::from_str(&json).ok())
                .unwrap_or_default();
            regenerate_links(&dir, &excluded).unwrap_or(0)
        }
        _ => 0,
    };

    Ok(BackgroundScanResult {
        total_files: scan_result.total_files,
        source_count: enabled.len(),
        linked_count,
        skipped: false,
    })
}
