use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize, Clone)]
pub struct UpdateAvailablePayload {
    pub version: String,
    pub body: String,
}

#[derive(Serialize, Clone)]
pub struct UpdateProgressPayload {
    pub downloaded: u64,
    pub total: Option<u64>,
}

#[derive(Serialize, Clone)]
pub struct UpdateErrorPayload {
    pub message: String,
}

#[tauri::command]
pub async fn check_for_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;

    let update = match updater.check().await {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(()),
        Err(e) => {
            let _ = app.emit(
                "update-error",
                UpdateErrorPayload {
                    message: e.to_string(),
                },
            );
            return Ok(());
        }
    };

    let payload = UpdateAvailablePayload {
        version: update.version.clone(),
        body: update.body.clone().unwrap_or_default(),
    };
    let _ = app.emit("update-available", payload);

    Ok(())
}

#[tauri::command]
pub async fn download_and_install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;

    let update = match updater.check().await {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    };

    let mut downloaded_bytes: u64 = 0;

    update
        .download_and_install(
            |chunk_length, content_length| {
                downloaded_bytes += chunk_length as u64;
                let _ = app.emit(
                    "update-progress",
                    UpdateProgressPayload {
                        downloaded: downloaded_bytes,
                        total: content_length,
                    },
                );
            },
            || {
                let _ = app.emit("update-ready", ());
            },
        )
        .await
        .map_err(|e| e.to_string())
}
