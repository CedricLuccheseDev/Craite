use tauri::State;

use crate::background::state::BackgroundScanState;
use crate::db::connection::open_connection;
use crate::db::repository;
use crate::error::ResultExt;

#[tauri::command]
pub fn set_background_scan_enabled(
    enabled: bool,
    state: State<'_, BackgroundScanState>,
) -> Result<(), String> {
    state.set_enabled(enabled);
    let conn = open_connection().str_err()?;
    repository::set_setting(
        &conn,
        "background_scan_enabled",
        if enabled { "true" } else { "false" },
    )
    .str_err()
}

#[tauri::command]
pub fn set_scan_interval(
    minutes: u64,
    state: State<'_, BackgroundScanState>,
) -> Result<(), String> {
    state.set_interval(minutes);
    let conn = open_connection().str_err()?;
    repository::set_setting(&conn, "scan_interval_minutes", &minutes.to_string()).str_err()
}

#[tauri::command]
pub fn get_background_scan_status(
    state: State<'_, BackgroundScanState>,
) -> Result<(bool, u64, bool), String> {
    Ok((state.is_enabled(), state.interval(), state.is_scanning()))
}
