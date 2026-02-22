# CrAIte Desktop

Tauri v2 desktop app — AI-powered sample library organizer for music producers.

## Architecture

```
src-tauri/src/          Rust backend
├── audio/              Audio preview (rodio)
├── background/         Background scan loop + state
├── classifier/         Sample classification by filename
├── commands/           Tauri IPC command handlers
├── db/                 SQLite persistence (rusqlite)
├── error.rs            Centralized error types + ResultExt helper
├── linker/             Symlink/hardlink strategy
├── scanner/            Filesystem scanning + DAW detection
├── tray/               System tray builder + events
├── watcher/            File watcher (notify-debouncer)
└── lib.rs              App entry point + plugin setup

src/                    Vue 3 + TypeScript frontend
├── assets/styles/      CSS variables, animations
├── components/
│   ├── common/         Reusable UI (AudioPlayer, ProgressBar)
│   ├── library/        Library browser, sidebar, settings
│   └── onboarding/     First-run wizard steps
├── composables/        Vue composition functions
│   ├── useTauri.ts     Single IPC gateway to Rust
│   ├── useBackgroundScan.ts
│   ├── useScanTimer.ts
│   └── useWatcher.ts
├── stores/             Pinia state management
│   ├── scan.ts         Sources + scan results
│   ├── library.ts      Samples + filtering
│   ├── libraryConfig.ts  Output dir, link config
│   └── settings.ts     Locale preferences
├── utils/              Pure utility functions
│   ├── categoryColors.ts  Centralized color mapping
│   └── categoryBuilder.ts
├── locales/            i18n (en.json, fr.json)
├── types/              TypeScript interfaces
└── views/              Top-level route views
```

## Key Design Patterns

### Error Handling (Rust)

All Tauri commands use `ResultExt::str_err()` to convert errors to strings, and `run_blocking()` to offload sync work:

```rust
use crate::error::{run_blocking, ResultExt};

#[tauri::command]
pub async fn load_samples() -> Result<Vec<Sample>, String> {
    run_blocking(|| with_db(|conn| repository::load_all_samples(conn))).await
}
```

### Scan Pipeline

`scanner::execute_scan()` is the shared scan entry point used by both:
- **Foreground scan** (`commands/scan.rs`) — triggered by user during onboarding
- **Background scan** (`background/scanner.rs`) — periodic automatic scan

### DAW Detection

Data-driven registry in `scanner/daw_detection.rs`. Each DAW is a `DawDef` struct with detection paths and library candidates. To add a new DAW, add an entry to the `DAWS` array — no new functions needed.

### Category Colors

Single source of truth in `utils/categoryColors.ts`:
- `CATEGORY_COLORS_CSS` — CSS variables for components
- `CATEGORY_COLORS_HEX` — Hex values for data builders

### Store ↔ Backend Communication

Stores use `useTauri()` composable (not raw `invoke()`) to call Rust commands. This keeps the IPC boundary in one place.

## Commands

```bash
pnpm dev:desktop     # Vite dev server (frontend only)
pnpm tauri:dev       # Full Tauri dev mode (frontend + Rust)
pnpm tauri:build     # Production build
pnpm build:desktop   # Build frontend only
pnpm lint            # ESLint
```

## Type Sync

Rust models use `#[serde(rename_all = "camelCase")]` to match TypeScript interfaces. Keep in sync manually:
- **Rust**: `src-tauri/src/db/models.rs`
- **TypeScript**: `src/types/sample.ts`
