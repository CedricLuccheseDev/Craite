use std::path::{Path, PathBuf};
use crate::db::models::ScanResult;
use crate::linker::strategy::{create_link, determine_strategy};

#[tauri::command]
pub fn classify_samples(scan_id: u32) -> Result<ScanResult, String> {
    // Re-classification will enhance existing results with Splice DB data
    let _ = scan_id;
    Err("Re-classification not yet implemented".to_string())
}

#[tauri::command]
pub fn create_links(output_dir: String) -> Result<usize, String> {
    let output_path = Path::new(&output_dir);
    std::fs::create_dir_all(output_path).map_err(|e| e.to_string())?;

    // Read the current scan results from the database
    let conn = crate::db::connection::open_connection()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT path, category, name FROM samples WHERE category != ''")
        .map_err(|e| e.to_string())?;

    let samples: Vec<(String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut linked = 0;

    for (source_path, category, name) in &samples {
        let source = PathBuf::from(source_path);
        let target = output_path.join(category).join(name);

        if target.exists() {
            continue;
        }

        let strategy = determine_strategy(&source, &target);
        if create_link(&source, &target, strategy).is_ok() {
            linked += 1;
        }
    }

    Ok(linked)
}
