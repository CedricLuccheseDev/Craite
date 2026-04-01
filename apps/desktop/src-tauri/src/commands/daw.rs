use crate::error::{run_blocking, ResultExt};
use crate::scanner::daw_detection::{detect_daws, DawInfo};
use crate::security::path_validator::sanitize_path;

#[tauri::command]
pub async fn detect_installed_daws() -> Result<Vec<DawInfo>, String> {
    run_blocking(|| Ok(detect_daws())).await
}

#[tauri::command]
pub async fn create_daw_library_folder(path: String) -> Result<String, String> {
    sanitize_path(&path)?;
    run_blocking(move || {
        std::fs::create_dir_all(&path).str_err()?;
        Ok(path)
    })
    .await
}
