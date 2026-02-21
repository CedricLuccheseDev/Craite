# CrAIte

AI-powered sample organizer for producers. Scans your sample folders, classifies sounds by type (kick, snare, pad...), and creates a mirror folder structure via hardlinks/symlinks for use in browser.

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

# Install all workspace dependencies
pnpm install

# Verify Rust compiles
cd apps/desktop/src-tauri && cargo check && cd ../../..
```

## Development

```bash
# Start the full Tauri dev environment (frontend + backend)
pnpm tauri:dev

# Start only the landing page dev server
pnpm dev:landing
```

The desktop app starts Vite on `localhost:1420` and opens the Tauri window with hot reload.
The landing page starts on `localhost:3000`.

## Build

```bash
# Production build (desktop)
pnpm tauri:build

# Production build (landing)
pnpm build:landing
```

The desktop distributable will be in `apps/desktop/src-tauri/target/release/bundle/`.

## Docker (Landing)

```bash
# Build Docker image
pnpm docker:landing:build

# Run container
pnpm docker:landing:run
```

The landing page will be accessible at `http://localhost:3000`.

## Project Structure

```
apps/
  desktop/                # Tauri v2 desktop app
    src/                  # Vue 3 / TypeScript frontend
      components/         # UI components (onboarding, library, common)
      composables/        # Vue composables (useTauri, useOnboarding)
      stores/             # Pinia stores (scan, library)
      views/              # Route views (Onboarding, Library)
      types/              # TypeScript interfaces
    src-tauri/src/        # Rust backend
      commands/           # Tauri IPC commands
      scanner/            # Filesystem scanning + Splice DB reader
      classifier/         # Filename-based classification
      linker/             # Hardlink / symlink / copy strategies
      watcher/            # File system watcher (notify)
      audio/              # Audio preview (rodio)
      db/                 # SQLite database (rusqlite)

  landing/                # Nuxt 3 landing page
    pages/                # Nuxt pages
    app.vue               # Root app component
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
| Landing  | Nuxt 3 + @nuxt/ui                |
