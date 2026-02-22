# Rust Backend Architecture

## Module Overview

| Module | Purpose | Key Files |
|--------|---------|-----------|
| `error` | `CraiteError` enum, `ResultExt` trait, `run_blocking()` helper | `error.rs` |
| `commands/` | Tauri IPC handlers (thin wrappers) | `scan.rs`, `persistence.rs`, `classify.rs`, `daw.rs`, `background.rs`, `watcher.rs`, `updater.rs` |
| `scanner/` | Filesystem scanning, DAW detection, shared `execute_scan()` | `mod.rs`, `filesystem.rs`, `daw_detection.rs`, `source_paths.rs` |
| `classifier/` | Rule-based sample classification | `filename.rs`, `rules.rs` |
| `db/` | SQLite connection, models, repository CRUD | `connection.rs`, `models.rs`, `repository.rs` |
| `background/` | Periodic background scanner with atomic state | `scanner.rs`, `state.rs` |
| `watcher/` | File system watcher (notify-debouncer, NonRecursive) | `handler.rs` |
| `linker/` | Link strategy (hardlink > symlink > copy) | `strategy.rs` |
| `audio/` | Audio preview playback (rodio) | `preview.rs` |
| `tray/` | System tray menu + events | `builder.rs`, `events.rs` |

## Data Flow

```
User action / Timer
       │
       ▼
  Tauri Command (commands/)
       │
       ▼
  execute_scan() (scanner/mod.rs)    ← shared between foreground & background
       │
       ├─ scan_directory()           ← filesystem.rs: recursive audio file walk
       ├─ classify_by_filename()     ← classifier/filename.rs: rule matching
       └─ insert_samples()           ← db/repository.rs: SQLite persistence
       │
       ▼
  Emit event to frontend
       │
       ▼
  Pinia store update → UI re-render
```

## Error Pattern

```rust
// Before (repeated 54 times):
let conn = open_connection().map_err(|e| e.to_string())?;
repository::insert_samples(&conn, &s).map_err(|e| e.to_string())

// After:
use crate::error::{run_blocking, ResultExt};
fn with_db<T>(f: impl FnOnce(&Connection) -> rusqlite::Result<T>) -> Result<T, String> {
    let conn = open_connection().str_err()?;
    f(&conn).str_err()
}
```

## Background Scan Safety

`BackgroundScanState` uses `AtomicBool` with `compare_exchange(AcqRel)` to prevent concurrent scan cycles:

```rust
if state.is_scanning.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_err() {
    return; // Another scan already running
}
```

## Adding a New DAW

Add an entry to the `DAWS` array in `scanner/daw_detection.rs`:

```rust
DawDef {
    id: "my_daw",
    name: "My DAW",
    candidates: |p| vec![format!("{}/path/to/daw", p.home)],
    library_candidates: |p| vec![format!("{}/Music/MyDAW/Samples", p.home)],
    library_fallback: |p| format!("{}/Music/MyDAW/CrAIte", p.home),
},
```
