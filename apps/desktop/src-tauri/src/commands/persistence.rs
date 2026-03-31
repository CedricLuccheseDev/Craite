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
    run_blocking(|| with_db(repository::load_all_samples)).await
}

#[tauri::command]
pub async fn save_source(source: Source) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::insert_source(conn, &source))).await
}

#[tauri::command]
pub async fn load_sources() -> Result<Vec<Source>, String> {
    run_blocking(|| with_db(repository::load_all_sources)).await
}

#[tauri::command]
pub async fn update_source_enabled(path: String, enabled: bool) -> Result<(), String> {
    run_blocking(move || {
        with_db(|conn| repository::update_source_enabled(conn, &path, enabled).map(|_| ()))
    })
    .await
}

#[tauri::command]
pub async fn set_sample_hidden(id: i64, hidden: bool) -> Result<(), String> {
    run_blocking(move || {
        with_db(|conn| repository::set_sample_hidden(conn, id, hidden).map(|_| ()))
    })
    .await
}

#[tauri::command]
pub async fn delete_source(path: String) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::delete_source(conn, &path).map(|_| ()))).await
}

#[tauri::command]
pub async fn save_setting(key: String, value: String) -> Result<(), String> {
    run_blocking(move || with_db(|conn| repository::set_setting(conn, &key, &value))).await
}

#[tauri::command]
pub async fn load_all_settings() -> Result<Vec<(String, String)>, String> {
    run_blocking(|| with_db(repository::load_all_settings)).await
}

#[tauri::command]
pub async fn reset_app() -> Result<(), String> {
    run_blocking(|| {
        let conn = open_connection().str_err()?;
        // Read output_dir before clearing so we can delete the folder
        let output_dir = repository::get_setting(&conn, "output_dir").str_err()?;
        repository::clear_all_data(&conn).str_err()?;
        // Delete the generated library folder
        if let Some(dir) = output_dir {
            if !dir.is_empty() {
                let path = std::path::Path::new(&dir);
                if path.is_dir() {
                    std::fs::remove_dir_all(path).ok();
                }
            }
        }
        Ok(())
    })
    .await
}
