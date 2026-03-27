use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    path.push("craite.db");
    path
}

pub fn open_connection() -> Result<Connection> {
    let path = get_db_path();
    let conn = Connection::open(&path)?;
    initialize_tables(&conn)?;
    Ok(conn)
}

fn initialize_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS samples (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            category TEXT NOT NULL DEFAULT '',
            subcategory TEXT NOT NULL DEFAULT '',
            confidence REAL NOT NULL DEFAULT 0.0,
            source TEXT NOT NULL,
            duration REAL NOT NULL DEFAULT 0.0,
            sample_rate INTEGER NOT NULL DEFAULT 0,
            linked_path TEXT
        );

        CREATE TABLE IF NOT EXISTS sources (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            label TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1,
            source_type TEXT NOT NULL DEFAULT 'custom',
            sample_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_samples_category
            ON samples(category);

        CREATE INDEX IF NOT EXISTS idx_samples_source
            ON samples(source);",
    )?;

    // Migration: add confidence column for existing databases
    let _ =
        conn.execute_batch("ALTER TABLE samples ADD COLUMN confidence REAL NOT NULL DEFAULT 0.0;");

    // Migration: add mtime column for incremental scan
    let _ = conn.execute_batch("ALTER TABLE samples ADD COLUMN mtime INTEGER NOT NULL DEFAULT 0;");

    // Migration: add hidden column for per-sample export exclusion
    let _ = conn.execute_batch("ALTER TABLE samples ADD COLUMN hidden INTEGER NOT NULL DEFAULT 0;");

    Ok(())
}

/// Resolve the app data directory
fn dirs_next() -> Option<PathBuf> {
    std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.local/share")))
        .ok()
        .map(|base| {
            let mut p = PathBuf::from(base);
            p.push("craite");
            std::fs::create_dir_all(&p).ok();
            p
        })
}
