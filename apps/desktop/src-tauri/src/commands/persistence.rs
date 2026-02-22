use crate::db::connection::open_connection;
use crate::db::models::{Sample, Source};
use crate::db::repository;
use crate::error::{run_blocking, ResultExt};

/// Open a DB connection and run a closure against it.
fn with_db<T, F>(f: F) -> Result<T, String>
where
    F: FnOnce(&rusqlite::Connection) -> rusqlite::Result<T>,
{
    let conn = open_connection().str_err()?;
    f(&conn).str_err()
}

#[tauri::command]
pub async fn save_samples(samples: Vec<Sample>) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::insert_samples(conn, &samples))).await
}

#[tauri::command]
pub async fn load_samples() -> Result<Vec<Sample>, String> {
    run_blocking(|| with_db(|conn| repository::load_all_samples(conn))).await
}

#[tauri::command]
pub async fn save_source(source: Source) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::insert_source(conn, &source))).await
}

#[tauri::command]
pub async fn load_sources() -> Result<Vec<Source>, String> {
    run_blocking(|| with_db(|conn| repository::load_all_sources(conn))).await
}

#[tauri::command]
pub async fn update_source_enabled(path: String, enabled: bool) -> Result<(), String> {
    run_blocking(move || {
        with_db(|conn| repository::update_source_enabled(conn, &path, enabled).map(|_| ()))
    })
    .await
}

#[tauri::command]
pub async fn save_setting(key: String, value: String) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::set_setting(conn, &key, &value))).await
}

#[tauri::command]
pub async fn load_all_settings() -> Result<Vec<(String, String)>, String> {
    run_blocking(|| with_db(|conn| repository::load_all_settings(conn))).await
}

#[tauri::command]
pub async fn reset_app() -> Result<(), String> {
    run_blocking(|| with_db(|conn| repository::clear_all_data(conn))).await
}
