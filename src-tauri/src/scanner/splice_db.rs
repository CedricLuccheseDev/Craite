use rusqlite::Connection;
use std::path::{Path, PathBuf};
use crate::error::CraiteError;

/// Metadata from Splice's sounds.db
pub struct SpliceSample {
    pub path: String,
    pub name: String,
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub tags: Vec<String>,
}

/// Find the Splice sounds.db file
pub fn find_splice_db() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let db_path = Path::new(&home)
        .join("Splice")
        .join("sounds.db");

    if db_path.exists() {
        Some(db_path)
    } else {
        None
    }
}

/// Read sample metadata from Splice database
pub fn read_splice_metadata(db_path: &Path) -> Result<Vec<SpliceSample>, CraiteError> {
    let conn = Connection::open_with_flags(
        db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;

    let mut stmt = conn.prepare(
        "SELECT file_path, name, bpm, musical_key FROM sounds LIMIT 50000",
    )?;

    let samples = stmt.query_map([], |row| {
        Ok(SpliceSample {
            path: row.get(0)?,
            name: row.get(1)?,
            bpm: row.get(2).ok(),
            key: row.get(3).ok(),
            tags: Vec::new(),
        })
    })?
    .filter_map(|r| r.ok())
    .collect();

    Ok(samples)
}
