use rusqlite::{Connection, Result, params};
use crate::db::models::{Sample, Source};

// --- Samples ---

pub fn insert_samples(conn: &Connection, samples: &[Sample]) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    let mut stmt = tx.prepare(
        "INSERT OR REPLACE INTO samples
            (name, path, category, subcategory, confidence, source, duration, sample_rate, linked_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    for sample in samples {
        stmt.execute(params![
            sample.name,
            sample.path,
            sample.category,
            sample.subcategory,
            sample.confidence,
            sample.source,
            sample.duration,
            sample.sample_rate,
            sample.linked_path,
        ])?;
    }

    drop(stmt);
    tx.commit()
}

pub fn load_all_samples(conn: &Connection) -> Result<Vec<Sample>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, path, category, subcategory, confidence, source,
                duration, sample_rate, linked_path
         FROM samples ORDER BY category, name",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Sample {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            category: row.get(3)?,
            subcategory: row.get(4)?,
            confidence: row.get(5)?,
            source: row.get(6)?,
            duration: row.get(7)?,
            sample_rate: row.get(8)?,
            linked_path: row.get(9)?,
        })
    })?;

    rows.collect()
}

pub fn clear_samples_by_source(conn: &Connection, source: &str) -> Result<usize> {
    conn.execute("DELETE FROM samples WHERE source = ?1", params![source])
}

// --- Sources ---

pub fn insert_source(conn: &Connection, source: &Source) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO sources (path, label, enabled, source_type, sample_count)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            source.path,
            source.label,
            source.enabled as i32,
            source.source_type,
            source.sample_count as i64,
        ],
    )?;
    Ok(())
}

pub fn load_all_sources(conn: &Connection) -> Result<Vec<Source>> {
    let mut stmt = conn.prepare(
        "SELECT path, label, enabled, source_type, sample_count FROM sources",
    )?;

    let rows = stmt.query_map([], |row| {
        let enabled: i32 = row.get(2)?;
        let sample_count: i64 = row.get(4)?;
        Ok(Source {
            path: row.get(0)?,
            label: row.get(1)?,
            enabled: enabled != 0,
            source_type: row.get(3)?,
            sample_count: sample_count as usize,
        })
    })?;

    rows.collect()
}

pub fn update_source_enabled(conn: &Connection, path: &str, enabled: bool) -> Result<usize> {
    conn.execute(
        "UPDATE sources SET enabled = ?1 WHERE path = ?2",
        params![enabled as i32, path],
    )
}

pub fn delete_source(conn: &Connection, path: &str) -> Result<usize> {
    conn.execute("DELETE FROM sources WHERE path = ?1", params![path])
}

pub fn clear_all_data(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "DELETE FROM samples;
         DELETE FROM sources;
         DELETE FROM settings;",
    )
}

// --- Settings ---

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )?;
    Ok(())
}

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query_map(params![key], |row| row.get(0))?;
    match rows.next() {
        Some(result) => Ok(Some(result?)),
        None => Ok(None),
    }
}

pub fn load_all_settings(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    rows.collect()
}
