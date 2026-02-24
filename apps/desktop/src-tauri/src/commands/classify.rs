use std::path::{Path, PathBuf};
use crate::db::connection::open_connection;
use crate::error::{run_blocking, ResultExt};
use crate::linker::strategy::{create_link, determine_strategy};

/// Reusable link generation logic, callable from commands and background tasks.
pub fn regenerate_links(output_dir: &str) -> Result<usize, String> {
    let output_path = Path::new(output_dir);
    std::fs::create_dir_all(output_path).str_err()?;

    let conn = open_connection().str_err()?;

    let mut stmt = conn
        .prepare("SELECT path, category, name FROM samples WHERE category != ''")
        .str_err()?;

    let samples: Vec<(String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .str_err()?
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
        match create_link(&source, &target, strategy) {
            Ok(_) => linked += 1,
            Err(e) => log::warn!("Failed to link {} to {}: {}", source.display(), target.display(), e),
        }
    }

    Ok(linked)
}

#[tauri::command]
pub async fn create_links(output_dir: String) -> Result<usize, String> {
    run_blocking(move || regenerate_links(&output_dir)).await
}
