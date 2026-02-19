# CrAIte

AI-powered sample organizer for FL Studio producers. Scans your sample folders, classifies sounds by type (kick, snare, pad...), and creates a mirror folder structure via hardlinks/symlinks for use in FL Studio's browser.

## Prerequisites

- **Node.js** >= 18
- **pnpm** >= 8
- **Rust** >= 1.70

### Linux (Ubuntu/Debian)

```bash
sudo apt-get install -y \
  pkg-config \
  build-essential \
  libwebkit2gtk-4.1-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libasound2-dev
```

### macOS

```bash
xcode-select --install
```

### Windows

Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload and [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

## Setup

```bash
# Clone the repo
git clone git@github.com:CedricLuccheseDev/Craite.git
cd Craite

# Install frontend dependencies
pnpm install

# Verify Rust compiles
cd src-tauri && cargo check && cd ..
```

## Development

```bash
# Start the full Tauri dev environment (frontend + backend)
pnpm tauri dev
```

This starts Vite on `localhost:1420` and opens the Tauri window with hot reload.

## Build

```bash
# Production build
pnpm tauri build
```

The distributable will be in `src-tauri/target/release/bundle/`.

## Project Structure

```
src/                    # Vue 3 / TypeScript frontend
  components/           # UI components (onboarding, library, common)
  composables/          # Vue composables (useTauri, useOnboarding)
  stores/               # Pinia stores (scan, library)
  views/                # Route views (Onboarding, Library)
  types/                # TypeScript interfaces

src-tauri/src/          # Rust backend
  commands/             # Tauri IPC commands
  scanner/              # Filesystem scanning + Splice DB reader
  classifier/           # Filename-based classification
  linker/               # Hardlink / symlink / copy strategies
  watcher/              # File system watcher (notify)
  audio/                # Audio preview (rodio)
  db/                   # SQLite database (rusqlite)
```

## Tech Stack

| Layer    | Technology                        |
|----------|-----------------------------------|
| Desktop  | Tauri v2                          |
| Frontend | Vue 3 + TypeScript + Vite + Pinia |
| Backend  | Rust                              |
| Database | SQLite (rusqlite)                 |
| Audio    | rodio                             |
| Watcher  | notify                            |
