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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE samples (
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

            CREATE TABLE sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                label TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                source_type TEXT NOT NULL DEFAULT 'custom',
                sample_count INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )
        .unwrap();
        conn
    }

    // --- Sample tests ---

    #[test]
    fn test_insert_and_load_samples() {
        let conn = setup_test_db();
        let samples = vec![
            Sample {
                id: None,
                name: "kick_01".to_string(),
                path: "/samples/kick_01.wav".to_string(),
                category: "kick".to_string(),
                subcategory: "".to_string(),
                confidence: 0.8,
                source: "/samples".to_string(),
                duration: Some(1.5),
                sample_rate: Some(44100),
                linked_path: None,
            },
            Sample {
                id: None,
                name: "snare_02".to_string(),
                path: "/samples/snare_02.wav".to_string(),
                category: "snare".to_string(),
                subcategory: "".to_string(),
                confidence: 0.9,
                source: "/samples".to_string(),
                duration: Some(0.8),
                sample_rate: Some(48000),
                linked_path: None,
            },
        ];

        insert_samples(&conn, &samples).unwrap();
        let loaded = load_all_samples(&conn).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "kick_01");
        assert_eq!(loaded[0].category, "kick");
        assert_eq!(loaded[1].name, "snare_02");
        assert_eq!(loaded[1].category, "snare");
    }

    #[test]
    fn test_clear_samples_by_source() {
        let conn = setup_test_db();
        let samples = vec![
            Sample {
                id: None,
                name: "sample1".to_string(),
                path: "/path1/sample1.wav".to_string(),
                category: "kick".to_string(),
                subcategory: "".to_string(),
                confidence: 0.8,
                source: "/path1".to_string(),
                duration: None,
                sample_rate: None,
                linked_path: None,
            },
            Sample {
                id: None,
                name: "sample2".to_string(),
                path: "/path2/sample2.wav".to_string(),
                category: "snare".to_string(),
                subcategory: "".to_string(),
                confidence: 0.8,
                source: "/path2".to_string(),
                duration: None,
                sample_rate: None,
                linked_path: None,
            },
        ];

        insert_samples(&conn, &samples).unwrap();
        let deleted = clear_samples_by_source(&conn, "/path1").unwrap();

        assert_eq!(deleted, 1);
        let loaded = load_all_samples(&conn).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].source, "/path2");
    }

    // --- Source tests ---

    #[test]
    fn test_insert_and_load_sources() {
        let conn = setup_test_db();
        let source = Source {
            path: "/samples/drums".to_string(),
            label: "Drums".to_string(),
            enabled: true,
            source_type: "daw".to_string(),
            sample_count: 150,
        };

        insert_source(&conn, &source).unwrap();
        let sources = load_all_sources(&conn).unwrap();

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].path, "/samples/drums");
        assert_eq!(sources[0].label, "Drums");
        assert_eq!(sources[0].enabled, true);
        assert_eq!(sources[0].sample_count, 150);
    }

    #[test]
    fn test_update_source_enabled() {
        let conn = setup_test_db();
        let source = Source {
            path: "/samples/drums".to_string(),
            label: "Drums".to_string(),
            enabled: true,
            source_type: "custom".to_string(),
            sample_count: 100,
        };

        insert_source(&conn, &source).unwrap();
        update_source_enabled(&conn, "/samples/drums", false).unwrap();

        let sources = load_all_sources(&conn).unwrap();
        assert_eq!(sources[0].enabled, false);
    }

    #[test]
    fn test_delete_source() {
        let conn = setup_test_db();
        let source = Source {
            path: "/samples/drums".to_string(),
            label: "Drums".to_string(),
            enabled: true,
            source_type: "custom".to_string(),
            sample_count: 100,
        };

        insert_source(&conn, &source).unwrap();
        let deleted = delete_source(&conn, "/samples/drums").unwrap();

        assert_eq!(deleted, 1);
        let sources = load_all_sources(&conn).unwrap();
        assert_eq!(sources.len(), 0);
    }

    // --- Settings tests ---

    #[test]
    fn test_set_and_get_setting() {
        let conn = setup_test_db();

        set_setting(&conn, "theme", "dark").unwrap();
        let value = get_setting(&conn, "theme").unwrap();

        assert_eq!(value, Some("dark".to_string()));
    }

    #[test]
    fn test_get_nonexistent_setting() {
        let conn = setup_test_db();
        let value = get_setting(&conn, "nonexistent").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_load_all_settings() {
        let conn = setup_test_db();

        set_setting(&conn, "theme", "dark").unwrap();
        set_setting(&conn, "language", "en").unwrap();

        let settings = load_all_settings(&conn).unwrap();
        assert_eq!(settings.len(), 2);
    }

    #[test]
    fn test_clear_all_data() {
        let conn = setup_test_db();

        // Insert test data
        let sample = Sample {
            id: None,
            name: "test".to_string(),
            path: "/test.wav".to_string(),
            category: "kick".to_string(),
            subcategory: "".to_string(),
            confidence: 0.8,
            source: "/test".to_string(),
            duration: None,
            sample_rate: None,
            linked_path: None,
        };
        insert_samples(&conn, &[sample]).unwrap();

        let source = Source {
            path: "/test".to_string(),
            label: "Test".to_string(),
            enabled: true,
            source_type: "custom".to_string(),
            sample_count: 1,
        };
        insert_source(&conn, &source).unwrap();

        set_setting(&conn, "key", "value").unwrap();

        // Clear all
        clear_all_data(&conn).unwrap();

        // Verify all tables are empty
        assert_eq!(load_all_samples(&conn).unwrap().len(), 0);
        assert_eq!(load_all_sources(&conn).unwrap().len(), 0);
        assert_eq!(load_all_settings(&conn).unwrap().len(), 0);
    }
}
